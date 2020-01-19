use crate::headers::*;
use crate::resource_quota::resource_quotas_from_str;
use crate::ResourceQuota;
use azure_sdk_core::errors::AzureError;
use chrono::{DateTime, Utc};
use http::HeaderMap;

pub(crate) fn request_charge_from_headers(headers: &HeaderMap) -> Result<f64, AzureError> {
    Ok(headers
        .get(HEADER_REQUEST_CHARGE)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_REQUEST_CHARGE.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn request_item_count_from_headers(headers: &HeaderMap) -> Result<u64, AzureError> {
    Ok(headers
        .get(HEADER_ITEM_COUNT)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_ITEM_COUNT.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn number_of_read_regions_from_headers(headers: &HeaderMap) -> Result<u32, AzureError> {
    Ok(headers
        .get(HEADER_NUMBER_OF_READ_REGIONS)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_NUMBER_OF_READ_REGIONS.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn activity_id_from_headers(headers: &HeaderMap) -> Result<uuid::Uuid, AzureError> {
    let s = headers
        .get(HEADER_ACTIVITY_ID)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_ACTIVITY_ID.to_owned()))?
        .to_str()?;
    Ok(uuid::Uuid::parse_str(s)?)
}

pub(crate) fn content_path_from_headers(headers: &HeaderMap) -> Result<&str, AzureError> {
    let s = headers
        .get(HEADER_CONTENT_PATH)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_CONTENT_PATH.to_owned()))?
        .to_str()?;
    Ok(s)
}

pub(crate) fn alt_content_path_from_headers(headers: &HeaderMap) -> Result<&str, AzureError> {
    Ok(headers
        .get(HEADER_ALT_CONTENT_PATH)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_ALT_CONTENT_PATH.to_owned()))?
        .to_str()?)
}

pub(crate) fn resource_quota_from_headers(
    headers: &HeaderMap,
) -> Result<Vec<ResourceQuota>, AzureError> {
    let s = headers
        .get(HEADER_RESOURCE_QUOTA)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_RESOURCE_QUOTA.to_owned()))?
        .to_str()?;
    Ok(resource_quotas_from_str(s)?)
}

pub(crate) fn resource_usage_from_headers(
    headers: &HeaderMap,
) -> Result<Vec<ResourceQuota>, AzureError> {
    let s = headers
        .get(HEADER_RESOURCE_USAGE)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_RESOURCE_USAGE.to_owned()))?
        .to_str()?;
    Ok(resource_quotas_from_str(s)?)
}

pub(crate) fn quorum_hacked_lsn_from_headers(headers: &HeaderMap) -> Result<u64, AzureError> {
    Ok(headers
        .get(HEADER_QUORUM_ACKED_LSN)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_QUORUM_ACKED_LSN.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn current_write_quorum_from_headers(headers: &HeaderMap) -> Result<u64, AzureError> {
    Ok(headers
        .get(HEADER_CURRENT_WRITE_QUORUM)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_CURRENT_WRITE_QUORUM.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn current_replica_set_size_from_headers(
    headers: &HeaderMap,
) -> Result<u64, AzureError> {
    Ok(headers
        .get(HEADER_CURRENT_REPLICA_SET_SIZE)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_CURRENT_REPLICA_SET_SIZE.to_owned()))?
        .to_str()?
        .parse()?)
}

pub(crate) fn schema_version_from_headers(headers: &HeaderMap) -> Result<&str, AzureError> {
    Ok(headers
        .get(HEADER_SCHEMA_VERSION)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_SCHEMA_VERSION.to_owned()))?
        .to_str()?)
}

pub(crate) fn service_version_from_headers(headers: &HeaderMap) -> Result<&str, AzureError> {
    Ok(headers
        .get(HEADER_SERVICE_VERSION)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_SERVICE_VERSION.to_owned()))?
        .to_str()?)
}

pub(crate) fn gateway_version_from_headers(headers: &HeaderMap) -> Result<&str, AzureError> {
    Ok(headers
        .get(HEADER_GATEWAY_VERSION)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_GATEWAY_VERSION.to_owned()))?
        .to_str()?)
}

pub(crate) fn last_state_change_from_headers(
    headers: &HeaderMap,
) -> Result<DateTime<Utc>, AzureError> {
    let last_modified = headers
        .get(HEADER_LAST_STATE_CHANGE_UTC)
        .ok_or_else(|| AzureError::HeaderNotFound(HEADER_LAST_STATE_CHANGE_UTC.to_owned()))?
        .to_str()?;
    debug!("last_modified == {:#}", last_modified);

    // since Azure returns "GMT" instead of +0000 as timezone we replace it
    // ourselves.
    // For example: Wed, 15 Jan 2020 23:39:44.369 GMT
    let last_modified = last_modified.replace("GMT", "+0000");
    debug!("last_modified == {:#}", last_modified);

    let last_modified = DateTime::parse_from_str(&last_modified, "%a, %e %h %Y %H:%M:%S%.f %z")?;
    debug!("last_modified == {:#}", last_modified);

    let last_modified = DateTime::from_utc(last_modified.naive_utc(), Utc);
    debug!("last_modified == {:#}", last_modified);

    Ok(last_modified)
}
