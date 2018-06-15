mod list_container_options;
pub use self::list_container_options::{ListContainerOptions, LIST_CONTAINER_OPTIONS_DEFAULT};

use chrono::{DateTime, Utc};
use futures::future::*;
use hyper::{Method, StatusCode};
use std::{fmt, str::FromStr};
use xml::Element;

use azure::core::{
    enumerations,
    errors::{check_status_extract_body, AzureError, TraversingError},
    incompletevector::IncompleteVector,
    lease::{LeaseDuration, LeaseState, LeaseStatus},
    parsing::{cast_must, cast_optional, traverse, FromStringOptional},
    util::format_header_value,
};
use azure::storage::client::Client;

const HEADER_BLOB_PUBLIC_ACCESS: &str = "x-ms-blob-public-access"; // [PublicAccess]

create_enum!(PublicAccess, (None, "none"), (Container, "container"), (Blob, "blob"));

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub last_modified: DateTime<Utc>,
    pub e_tag: String,
    pub lease_status: LeaseStatus,
    pub lease_state: LeaseState,
    pub lease_duration: Option<LeaseDuration>,
}

impl Container {
    pub fn new(name: &str) -> Container {
        Container {
            name: name.to_owned(),
            last_modified: Utc::now(),
            e_tag: "".to_owned(),
            lease_status: LeaseStatus::Unlocked,
            lease_state: LeaseState::Available,
            lease_duration: None,
        }
    }

    pub fn parse(elem: &Element) -> Result<Container, AzureError> {
        let name = cast_must::<String>(elem, &["Name"])?;
        let last_modified = cast_must::<DateTime<Utc>>(elem, &["Properties", "Last-Modified"])?;
        let e_tag = cast_must::<String>(elem, &["Properties", "Etag"])?;

        let lease_state = cast_must::<LeaseState>(elem, &["Properties", "LeaseState"])?;

        let lease_duration = cast_optional::<LeaseDuration>(elem, &["Properties", "LeaseDuration"])?;

        let lease_status = cast_must::<LeaseStatus>(elem, &["Properties", "LeaseStatus"])?;

        Ok(Container {
            name,
            last_modified,
            e_tag,
            lease_status,
            lease_state,
            lease_duration,
        })
    }

    pub fn delete(&mut self, c: &Client) -> impl Future<Item = (), Error = AzureError> {
        let uri = format!("https://{}.blob.core.windows.net/{}?restype=container", c.account(), self.name);

        let req = c.perform_request(&uri, Method::DELETE, |_| {}, None);

        done(req)
            .from_err()
            .and_then(move |future_response| check_status_extract_body(future_response, StatusCode::ACCEPTED).and_then(|_| ok(())))
    }

    pub fn create(c: &Client, container_name: &str, pa: PublicAccess) -> impl Future<Item = (), Error = AzureError> {
        let uri = format!("https://{}.blob.core.windows.net/{}?restype=container", c.account(), container_name);

        let req = c.perform_request(
            &uri,
            Method::PUT,
            |ref mut request| {
                request.header(HEADER_BLOB_PUBLIC_ACCESS, format_header_value(pa).unwrap());
            },
            Some(&[]),
        );

        done(req)
            .from_err()
            .and_then(move |future_response| check_status_extract_body(future_response, StatusCode::CREATED).and_then(|_| ok(())))
    }

    // TODO
    // pub fn get_acl(c : &Client, gao : &GetAclOptions)

    pub fn list(c: &Client, lco: &ListContainerOptions) -> impl Future<Item = IncompleteVector<Container>, Error = AzureError> {
        let mut uri = format!(
            "https://{}.blob.core.windows.net?comp=list&maxresults={}",
            c.account(),
            lco.max_results
        );

        if !lco.include_metadata {
            uri = format!("{}&include=metadata", uri);
        }

        if let Some(ref prefix) = lco.prefix {
            uri = format!("{}&prefix={}", uri, prefix);
        }

        if let Some(ref nm) = lco.next_marker {
            uri = format!("{}&marker={}", uri, nm);
        }

        if let Some(ref timeout) = lco.timeout {
            uri = format!("{}&timeout={}", uri, timeout);
        }

        let req = c.perform_request(&uri, Method::GET, |_| {}, None);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::OK)
                .and_then(|body| done(incomplete_vector_from_response(&body)).from_err())
        })
    }
}

fn incomplete_vector_from_response(body: &str) -> Result<IncompleteVector<Container>, AzureError> {
    let elem: Element = body.parse()?;

    let mut v = Vec::new();

    for container in traverse(&elem, &["Containers", "Container"], true)? {
        v.push(Container::parse(container)?);
    }

    let next_marker = match cast_optional::<String>(&elem, &["NextMarker"])? {
        Some(ref nm) if nm == "" => None,
        Some(nm) => Some(nm),
        None => None,
    };

    Ok(IncompleteVector::new(next_marker, v))
}
