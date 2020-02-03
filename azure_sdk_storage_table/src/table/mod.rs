mod batch;
use self::batch::generate_batch_payload;
pub use self::batch::BatchItem;
use crate::TableEntity;
use azure_sdk_core::errors::{
    check_status_extract_body, check_status_extract_headers_and_body, AzureError,
};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::{
    get_default_json_mime, get_json_mime_fullmetadata, get_json_mime_nometadata, ServiceType,
};
use futures::stream::Stream;
use http::HeaderMap;
use hyper::client::ResponseFuture;
use hyper::header::{self, HeaderValue};
use hyper::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::convert::TryFrom;

const TABLE_TABLES: &str = "TABLES";

#[derive(Clone)]
pub struct TableService {
    client: Client,
}

impl TableService {
    pub fn new(client: Client) -> Self {
        TableService { client }
    }

    pub async fn list_tables(&self) -> Result<Vec<String>, AzureError> {
        let entities = self.query_entities(TABLE_TABLES, None).await?;
        let e: Vec<String> = entities
            .into_iter()
            .map(|x: TableEntity<TableData>| x.payload.table_name)
            .collect();
        Ok(e)
    }

    // Create table if not exists.
    pub async fn create_table<T: Into<String>>(&self, table_name: T) -> Result<(), AzureError> {
        let body = &serde_json::to_string(&TableData {
            table_name: table_name.into(),
        })
        .unwrap();
        debug!("body == {}", body);
        let future_response = self.request_with_default_header(
            TABLE_TABLES,
            &Method::POST,
            Some(body),
            false,
            |_| {},
        )?;

        check_status_extract_body(future_response, StatusCode::CREATED).await?;
        Ok(())
    }

    pub async fn get_entity<T: DeserializeOwned>(
        &self,
        table_name: &str,
        partition_key: &str,
        row_key: &str,
    ) -> Result<Option<TableEntity<T>>, AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        let path = &entity_path(table_name, partition_key, row_key);
        let future_response =
            self.request_with_default_header(path, &Method::GET, None, false, |_| {})?;
        let (headers, body) =
            match check_status_extract_headers_and_body(future_response, StatusCode::OK).await {
                Err(AzureError::UnexpectedHTTPResult(e)) if e.status_code() == 404 => {
                    return Ok(None)
                }
                x => x,
            }?;

        TableEntity::try_from((&headers, &body as &[u8])).map(|entity| Some(entity))
    }

    pub async fn query_entities<T>(
        &self,
        table_name: &str,
        query: Option<&str>,
    ) -> Result<Vec<TableEntity<T>>, AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut path = table_name.to_owned();
        if let Some(clause) = query {
            path.push_str("?");
            path.push_str(clause);
        }

        let future_response =
            self.request_with_default_header(path.as_str(), &Method::GET, None, false, |_| {})?;
        let body = check_status_extract_body(future_response, StatusCode::OK).await?;
        let ec = serde_json::from_str::<EntityCollection<T>>(&body)?;
        Ok(ec.value)
    }

    async fn query_entity_collection<T>(
        &self,
        table_name: &str,
        query: Option<&str>,
        continuation: Option<&Continuation>,
        fullmetadata: bool,
    ) -> Result<EntityCollection<T>, AzureError>
    where
        T: DeserializeOwned + Serialize,
    {
        debug!("query_entity_collection(table_name == {}, query == {:?}, continuation == {:?}, fullmetadata == {:?}) called", table_name, query, continuation, fullmetadata);
        let mut path = table_name.to_owned();
        path.push_str("?");
        if let Some(clause) = query {
            path.push_str(clause);
        }
        if let Some(cont) = continuation {
            path.push_str("&NextPartitionKey=");
            path.push_str(&cont.next_partition_key);
            path.push_str("&NextRowKey=");
            path.push_str(&cont.next_row_key);
        }

        let future_response = self.request_with_default_header(
            path.as_str(),
            &Method::GET,
            None,
            fullmetadata,
            |_| {},
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        Ok(
            serde_json::from_slice::<EntityCollection<T>>(&body).map(|mut ec| {
                ec.continuation = continuation_from_headers(&headers);
                ec
            })?,
        )
    }

    fn stream_query_entities_metadata<'a, T>(
        &'a self,
        table_name: &'a str,
        query: Option<&'a str>,
        fullmetadata: bool,
    ) -> impl Stream<Item = Result<Vec<TableEntity<T>>, AzureError>> + 'a
    where
        T: Serialize + DeserializeOwned + 'a,
    {
        futures::stream::unfold(ContinuationState::Start, move |cont_state| {
            async move {
                let cont = match cont_state {
                    ContinuationState::Start => None,
                    ContinuationState::Next(Some(cont)) => Some(cont),
                    ContinuationState::Next(None) => return None,
                };

                debug!("cont == {:?}", cont);

                let mut path = table_name.to_owned();
                if let Some(clause) = query {
                    path.push_str("?");
                    path.push_str(clause);
                }

                let ec = self
                    .query_entity_collection(table_name, query, cont.as_ref(), fullmetadata)
                    .await;

                let ec = match ec {
                    Ok(ec) => ec,
                    Err(err) => return Some((Err(err), ContinuationState::Next(None))),
                };

                Some((Ok(ec.value), ContinuationState::Next(ec.continuation)))
            }
        })
    }

    pub fn stream_query_entities<'a, T>(
        &'a self,
        table_name: &'a str,
        query: Option<&'a str>,
    ) -> impl Stream<Item = Result<Vec<TableEntity<T>>, AzureError>> + 'a
    where
        T: Serialize + DeserializeOwned + 'a,
    {
        self.stream_query_entities_metadata(table_name, query, false)
    }

    pub fn stream_query_entities_fullmetadata<'a, T>(
        &'a self,
        table_name: &'a str,
        query: Option<&'a str>,
    ) -> impl Stream<Item = Result<Vec<TableEntity<T>>, AzureError>> + 'a
    where
        T: Serialize + DeserializeOwned + 'a,
    {
        self.stream_query_entities_metadata(table_name, query, true)
    }

    pub async fn insert_entity<T>(
        &self,
        table_name: &str,
        entity: TableEntity<T>,
    ) -> Result<TableEntity<T>, AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        let obj_ser = serde_json::to_string(&entity)?.to_owned();

        let future_response = self.request_with_default_header(
            table_name,
            &Method::POST,
            Some(&obj_ser),
            false,
            |_| {},
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;
        TableEntity::try_from((&headers, &body as &[u8]))
    }

    pub async fn update_entity<T>(
        &self,
        table_name: &str,
        mut entity: TableEntity<T>,
    ) -> Result<TableEntity<T>, AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        let obj_ser = serde_json::to_string(&entity)?.to_owned();
        let path = &entity_path(table_name, &entity.partition_key, &entity.row_key);

        // IsMatched is mandatory, we pass * if the caller
        // does not care for it.
        let etag = match entity.etag {
            Some(ref etag) => etag.as_ref(),
            None => "*",
        };

        let future_response = self.request_with_default_header(
            path,
            &Method::PUT,
            Some(&obj_ser),
            false,
            |headers| {
                headers.append(header::IF_MATCH, etag.parse().unwrap());
            },
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::NO_CONTENT).await?;

        // inject etag if present
        entity.etag = match headers.get(header::ETAG) {
            Some(etag) => Some(etag.to_str()?.to_owned()),
            None => None,
        };
        Ok(entity)
    }

    pub async fn delete_entity<'a>(
        &self,
        table_name: &str,
        partition_key: &'a str,
        row_key: &'a str,
        etag: Option<&'a str>,
    ) -> Result<(), AzureError> {
        let path = &entity_path(table_name, partition_key, row_key);

        let etag = match etag {
            Some(ref etag) => etag,
            None => "*",
        };

        let future_response = self.request(path, &Method::DELETE, None, |mut request| {
            request = request.header(
                header::ACCEPT,
                HeaderValue::from_static(get_json_mime_nometadata()),
            );
            request = request.header(header::IF_MATCH, etag);
            
            request
        })?;
        check_status_extract_body(future_response, StatusCode::NO_CONTENT).await?;
        Ok(())
    }

    pub async fn batch<T>(
        &self,
        table_name: &str,
        partition_key: &str,
        batch_items: &[BatchItem<T>],
    ) -> Result<(), AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        let payload = &generate_batch_payload(
            self.client.get_uri_prefix(ServiceType::Table).as_str(),
            table_name,
            partition_key,
            batch_items,
        );

        let future_response = self.request("$batch", &Method::POST, Some(payload), |request| {
            request.header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static(get_batch_mime()),
            )
        })?;
        check_status_extract_body(future_response, StatusCode::ACCEPTED).await?;
        // TODO deal with body response, handle batch failure.
        // let ref body = get_response_body(&mut response)?;
        // info!("{}", body);
        Ok(())
    }

    fn request_with_default_header<H>(
        &self,
        segment: &str,
        method: &Method,
        request_str: Option<&str>,
        fullmetadata: bool,
        add_extra_headers: H,
    ) -> Result<ResponseFuture, AzureError>
    where
        H: FnOnce(&mut HeaderMap),
    {
        self.request(segment, method, request_str, |mut request| {
            if fullmetadata {
                request = request.header(
                    header::ACCEPT,
                    HeaderValue::from_static(get_json_mime_fullmetadata()),
                );
            } else {
                request = request.header(
                    header::ACCEPT,
                    HeaderValue::from_static(get_json_mime_nometadata()),
                );
            }
            request = request.header(
                header::ACCEPT,
                HeaderValue::from_static(get_json_mime_nometadata()),
            );
            if request_str.is_some() {
                request = request.header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(get_default_json_mime()),
                );
            }

            // On error during build, headers_mut returns None,
            // thus we skip extra headers handling and let request fail gracefully
            if let Some(ref mut headers) = request.headers_mut() {
                add_extra_headers(headers);
            }

            request
        })
    }

    fn request<F>(
        &self,
        segment: &str,
        method: &Method,
        request_str: Option<&str>,
        headers_func: F,
    ) -> Result<ResponseFuture, AzureError>
    where
        F: FnOnce(::http::request::Builder) -> ::http::request::Builder,
    {
        trace!("{:?} {}", method, segment);
        if let Some(body) = request_str {
            trace!("Request: {}", body);
        }

        let request_vec: Option<&[u8]> = match request_str {
            Some(s) => Some(s.as_bytes()),
            None => None,
        };

        self.client
            .perform_table_request(segment, method, headers_func, request_vec)
    }
}

#[derive(Clone)]
pub struct TableStorage {
    service: TableService,
    table_name: String,
}

impl TableStorage {
    pub fn new<S: Into<String>>(service: TableService, table_name: S) -> Self {
        TableStorage {
            service,
            table_name: table_name.into(),
        }
    }

    pub async fn create(&self) -> Result<(), AzureError> {
        self.service.create_table(self.table_name.clone()).await
    }

    pub async fn create_if_not_exists(&self) -> Result<(), AzureError> {
        self.create().await.or_else(ignore_409)
    }

    pub async fn get_entity<T: DeserializeOwned>(
        &self,
        partition_key: &str,
        row_key: &str,
    ) -> Result<Option<TableEntity<T>>, AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        self.service
            .get_entity(&self.table_name, partition_key, row_key)
            .await
    }

    pub async fn query_entities<T>(
        &self,
        query: Option<&str>,
    ) -> Result<Vec<TableEntity<T>>, AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        self.service.query_entities(&self.table_name, query).await
    }

    pub fn stream_query_entities<'a, T>(
        &'a self,
        query: Option<&'a str>,
    ) -> impl Stream<Item = Result<Vec<TableEntity<T>>, AzureError>> + 'a
    where
        T: Serialize + DeserializeOwned + 'a,
    {
        self.service.stream_query_entities(&self.table_name, query)
    }

    pub fn stream_query_entities_fullmetadata<'a, T>(
        &'a self,
        query: Option<&'a str>,
    ) -> impl Stream<Item = Result<Vec<TableEntity<T>>, AzureError>> + 'a
    where
        T: Serialize + DeserializeOwned + 'a,
    {
        self.service
            .stream_query_entities_fullmetadata(&self.table_name, query)
    }

    pub async fn insert_entity<T>(&self, entity: TableEntity<T>) -> Result<TableEntity<T>, AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        self.service
            .insert_entity::<T>(&self.table_name, entity)
            .await
    }

    pub async fn update_entity<T>(&self, entity: TableEntity<T>) -> Result<TableEntity<T>, AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        self.service.update_entity(&self.table_name, entity).await
    }

    pub async fn delete_entity<'a>(
        &self,
        partition_key: &'a str,
        row_key: &'a str,
        etag: Option<&'a str>,
    ) -> Result<(), AzureError> {
        self.service
            .delete_entity(&self.table_name, partition_key, row_key, etag)
            .await
    }

    pub async fn batch<T>(
        &self,
        partition_key: &str,
        batch_items: &[BatchItem<T>],
    ) -> Result<(), AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        self.service
            .batch(&self.table_name, partition_key, batch_items)
            .await
    }
}

const HEADER_NEXTPARTITIONKEY: &str = "x-ms-continuation-NextPartitionKey";
const HEADER_NEXTROWKEY: &str = "x-ms-continuation-NextRowKey";

fn continuation_from_headers(headers: &HeaderMap) -> Option<Continuation> {
    if headers.contains_key(HEADER_NEXTPARTITIONKEY) && headers.contains_key(HEADER_NEXTROWKEY) {
        Some(Continuation {
            next_partition_key: headers[HEADER_NEXTPARTITIONKEY]
                .to_str()
                .unwrap()
                .to_string(),
            next_row_key: headers[HEADER_NEXTROWKEY].to_str().unwrap().to_string(),
        })
    } else {
        None
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TableData {
    table_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EntityCollection<T> {
    value: Vec<TableEntity<T>>,
    #[serde(skip)]
    continuation: Option<Continuation>,
}

#[derive(Debug, Clone)]
struct Continuation {
    next_partition_key: String,
    next_row_key: String,
}

#[derive(Debug, Clone)]
enum ContinuationState {
    Start,
    Next(Option<Continuation>),
}

#[inline]
fn entity_path(table_name: &str, partition_key: &str, row_key: &str) -> String {
    table_name.to_owned() + "(PartitionKey='" + partition_key + "',RowKey='" + row_key + "')"
}

#[inline]
pub fn get_batch_mime() -> &'static str {
    "multipart/mixed; boundary=batch_a1e9d677-b28b-435e-a89e-87e6a768a431"
}

#[inline]
pub fn ignore_409(err: AzureError) -> Result<(), AzureError> {
    match err {
        AzureError::UnexpectedHTTPResult(e) if e.status_code() == 409 => Ok(()),
        e => Err(e),
    }
}
