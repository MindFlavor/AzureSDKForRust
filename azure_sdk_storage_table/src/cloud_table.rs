use crate::{
    entity_path, get_batch_mime, Batch, Continuation2, MetadataDetail, TableClient, TableEntity,
};
use azure_sdk_core::errors::{
    check_status_extract_body, check_status_extract_headers_and_body, AzureError,
};
use azure_sdk_storage_core::ServiceType;
use hyper::{header, Method, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;
use std::convert::TryFrom;

/// Represents a table in the Microsoft Azure Table service.
#[derive(Clone)]
pub struct CloudTable {
    client: TableClient,
    table_name: String,
}

impl CloudTable {
    /// Creates an CloadTable using the specified client and table name
    pub fn new<T: Into<String>>(client: TableClient, table: T) -> Self {
        CloudTable {
            client,
            table_name: table.into(),
        }
    }

    /// Creates the table in the storage service with default request options.
    pub async fn create(&self) -> Result<(), AzureError> {
        self.client.create_table(&self.table_name).await
    }

    /// Creates the table in the storage service using default request options if it does not already exist.
    pub async fn create_if_not_exists(&self) -> Result<(), AzureError> {
        self.create().await.or_else(|err| match err {
            AzureError::UnexpectedHTTPResult(e) if e.status_code() == 409 => Ok(()),
            e => Err(e),
        })
    }

    pub async fn get_entity<T>(
        &self,
        partition_key: &str,
        row_key: &str,
    ) -> Result<Option<TableEntity<T>>, AzureError>
    where
        T: DeserializeOwned,
    {
        let path = &entity_path(&self.table_name, partition_key, row_key);
        let future_response = self.client.request_with_default_header(
            path,
            &Method::GET,
            None,
            MetadataDetail::None, // etag is provided through header, no extra meta info is required
            |req| req,
        )?;
        let (headers, body) =
            match check_status_extract_headers_and_body(future_response, StatusCode::OK).await {
                Err(AzureError::UnexpectedHTTPResult(e)) if e.status_code() == 404 => {
                    return Ok(None)
                }
                x => x,
            }?;
        let entity = TableEntity::try_from((&headers, &body as &[u8]))?;
        Ok(Some(entity))
    }

    /// Insert a new entity into the table. If entity already exists, the operation fails.
    /// See https://docs.microsoft.com/en-us/rest/api/storageservices/insert-entity
    pub async fn insert_entity<T>(
        &self,
        partition_key: &str,
        row_key: &str,
        payload: T,
    ) -> Result<TableEntity<T>, AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        let entity: TableEntity<T> = TableEntity {
            partition_key: partition_key.to_owned(),
            row_key: row_key.to_owned(),
            etag: None,
            timestamp: None,
            payload,
        };
        let obj_ser = serde_json::to_string(&entity)?.to_owned();

        let future_response = self.client.request_with_default_header(
            &self.table_name,
            &Method::POST,
            Some(&obj_ser),
            MetadataDetail::None,
            |req| req,
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;
        let entity = TableEntity::try_from((&headers, &body as &[u8]))?;
        Ok(entity)
    }

    /// Insert or updates an entity. Even if the entity is already present the operation succeeds and the
    /// entity is replaced.
    /// See https://docs.microsoft.com/en-us/rest/api/storageservices/insert-or-replace-entity
    pub async fn insert_or_update_entity<T>(
        &self,
        partition_key: &str,
        row_key: &str,
        payload: T,
    ) -> Result<TableEntity<T>, AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        let entity: TableEntity<T> = TableEntity {
            partition_key: partition_key.to_owned(),
            row_key: row_key.to_owned(),
            etag: None,
            timestamp: None,
            payload,
        };
        let obj_ser = serde_json::to_string(&entity)?.to_owned();

        let future_response = self.client.request_with_default_header(
            &self.table_name,
            &Method::PUT,
            Some(&obj_ser),
            MetadataDetail::None,
            |req| req,
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;
        let entity = TableEntity::try_from((&headers, &body as &[u8]))?;
        Ok(entity)
    }

    /// Update an existing entity.
    /// See https://docs.microsoft.com/en-us/rest/api/storageservices/update-entity2
    pub async fn update_entity<T>(
        &self,
        entity: TableEntity<T>,
    ) -> Result<TableEntity<T>, AzureError>
    where
        T: Serialize + DeserializeOwned,
    {
        let obj_ser = serde_json::to_string(&entity)?.to_owned();
        let path = &entity_path(&self.table_name, &entity.partition_key, &entity.row_key);

        let etag = entity.etag;

        let future_response = self.client.request_with_default_header(
            path,
            &Method::PUT,
            Some(&obj_ser),
            MetadataDetail::None,
            |mut request| {
                if let Some(etag) = etag {
                    request = request.header(header::IF_MATCH, etag);
                }
                request
            },
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::NO_CONTENT).await?;
        let entity = TableEntity::try_from((&headers, &body as &[u8]))?;
        Ok(entity)
    }

    pub async fn delete_entity<'a>(
        &self,
        partition_key: &'a str,
        row_key: &'a str,
        etag: Option<&'a str>,
    ) -> Result<(), AzureError> {
        let path = &entity_path(&self.table_name, partition_key, row_key);

        let etag = match etag {
            Some(ref etag) => etag,
            None => "*",
        };

        let future_response = self.client.request_with_default_header(
            path,
            &Method::DELETE,
            None,
            MetadataDetail::None,
            |request| request.header(header::IF_MATCH, etag),
        )?;

        check_status_extract_body(future_response, StatusCode::NO_CONTENT).await?;
        Ok(())
    }

    pub async fn delete_entity_by_entity<'a, T>(
        &self,
        entity: TableEntity<T>,
    ) -> Result<(), AzureError> {
        self.delete_entity(
            &entity.partition_key,
            &entity.row_key,
            entity.etag.as_deref(),
        )
        .await
    }

    pub async fn query_entities<T>(
        &self,
        query: Option<&str>,
        continuation: &mut Continuation2,
    ) -> Result<Option<Vec<TableEntity<T>>>, AzureError>
    where
        T: DeserializeOwned + Serialize,
    {
        debug!(
            "query_entities(query = {:?}, continuation = {:?})",
            query, continuation
        );
        if continuation.fused {
            return Ok(None);
        }

        let mut path = self.table_name.to_owned();
        path.push_str("?");
        if let Some(clause) = query {
            path.push_str(clause);
        }
        if let Some(ref cont) = continuation.next {
            path.push_str("&NextPartitionKey=");
            path.push_str(&cont.partition_key);
            path.push_str("&NextRowKey=");
            path.push_str(&cont.row_key);
        }

        let future_response = self.client.request_with_default_header(
            path.as_str(),
            &Method::GET,
            None,
            MetadataDetail::Full, // etag is provided through metadata only
            |req| req,
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        log::trace!("body == {:?}", std::str::from_utf8(&body));
        let entities = serde_json::from_slice::<EntityCollection2<T>>(&body)?;
        *continuation = Continuation2::try_from(&headers)?;
        Ok(Some(entities.value))
    }

    /*
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
    */

    pub async fn batch(&self, batch: Batch) -> Result<(), AzureError> {
        let payload = batch.into_payload(self.client.get_uri_prefix().as_str(), &self.table_name);

        let future_response =
            self.client
                .request("$batch", &Method::POST, Some(&payload), |request| {
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
}

#[derive(Debug, Serialize, Deserialize)]
struct EntityCollection2<T> {
    value: Vec<TableEntity<T>>,
}
