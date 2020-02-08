use azure_sdk_core::errors::AzureError;
use chrono::{DateTime, Utc};
use http::header;
use http::HeaderMap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoData {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableEntity<T> {
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,

    #[serde(skip_serializing, rename = "odata.etag")]
    pub etag: Option<String>,

    #[serde(
        skip_serializing,
        deserialize_with = "deserialize::optional_timestamp",
        rename = "Timestamp"
    )]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(flatten)]
    pub payload: T,
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for TableEntity<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;

    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;
        log::trace!("headers == {:?}", headers);
        log::trace!("body == {:?}", std::str::from_utf8(body));

        let mut entity: Self = serde_json::from_slice(&body)?;

        if let Some(etag) = headers.get(header::ETAG) {
            entity.etag = Some(etag.to_str()?.to_owned());
        }

        Ok(entity)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ContinuationCursor {
    pub(crate) partition_key: String,
    pub(crate) row_key: String,
}

#[derive(Debug, Clone)]
pub struct Continuation {
    pub(crate) fused: bool,
    pub(crate) next: Option<ContinuationCursor>,
}

impl Continuation {
    pub fn start() -> Self {
        Continuation {
            fused: false,
            next: None,
        }
    }
}

impl std::convert::TryFrom<&HeaderMap> for Continuation {
    type Error = AzureError;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        const HEADER_NEXTPARTITIONKEY: &str = "x-ms-continuation-NextPartitionKey";
        const HEADER_NEXTROWKEY: &str = "x-ms-continuation-NextRowKey";

        if headers.contains_key(HEADER_NEXTPARTITIONKEY) && headers.contains_key(HEADER_NEXTROWKEY)
        {
            Ok(Continuation {
                fused: false,
                next: Some(ContinuationCursor {
                    partition_key: headers[HEADER_NEXTPARTITIONKEY].to_str()?.to_string(),
                    row_key: headers[HEADER_NEXTROWKEY].to_str()?.to_string(),
                }),
            })
        } else {
            Ok(Continuation {
                fused: true,
                next: None,
            })
        }
    }
}

/// Helper functions to (de)serialize entity properties
mod deserialize {
    use chrono::{DateTime, Utc};
    use serde::{
        de::{Error, Visitor},
        Deserializer,
    };
    use std::fmt;

    struct TimestampVisitor;

    pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        d.deserialize_str(TimestampVisitor)
    }

    impl<'de> Visitor<'de> for TimestampVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a timestamp string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match value.parse::<DateTime<Utc>>() {
                Ok(date) => Ok(date),
                Err(e) => Err(E::custom(format!("Parse error {} for {}", e, value))),
            }
        }
    }

    struct OptionalTimestampVisitor;

    impl<'de> Visitor<'de> for OptionalTimestampVisitor {
        type Value = Option<DateTime<Utc>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "null or a timestamp string")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, d: D) -> Result<Option<DateTime<Utc>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(Some(d.deserialize_str(TimestampVisitor)?))
        }
    }

    pub fn optional_timestamp<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        d.deserialize_option(OptionalTimestampVisitor)
    }
}
