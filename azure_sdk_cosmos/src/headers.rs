pub(crate) const HEADER_VERSION: &str = "x-ms-version"; // Cow[str]
pub(crate) const HEADER_DATE: &str = "x-ms-date"; // [String]
pub(crate) const HEADER_DOCUMENTDB_IS_UPSERT: &str = "x-ms-documentdb-is-upsert"; // [bool]
pub(crate) const HEADER_INDEXING_DIRECTIVE: &str = "x-ms-indexing-directive"; // [IndexingDirective]
pub(crate) const HEADER_MAX_ITEM_COUNT: &str = "x-ms-max-item-count"; // [u64]
pub(crate) const HEADER_ITEM_COUNT: &str = "x-ms-item-count"; // [u64]
pub(crate) const HEADER_CONTINUATION: &str = "x-ms-continuation"; // [ContinuationToken]
pub(crate) const HEADER_CONSISTENCY_LEVEL: &str = "x-ms-consistency-level"; // [ConsistencyLevel]
pub(crate) const HEADER_SESSION_TOKEN: &str = "x-ms-session-token"; // [ContinuationToken]
pub(crate) const HEADER_ALLOW_MULTIPLE_WRITES: &str = "x-ms-cosmos-allow-tentative-writes"; // [bool]
pub(crate) const HEADER_A_IM: &str = "A-IM"; // Cow[str]
pub(crate) const HEADER_ACTIVITY_ID: &str = "x-ms-activity-id"; // [String]
pub(crate) const HEADER_DOCUMENTDB_PARTITIONRANGEID: &str = "x-ms-documentdb-partitionkeyrangeid"; // [String]
pub(crate) const HEADER_DOCUMENTDB_PARTITIONKEY: &str = "x-ms-documentdb-partitionkey"; // [String]
pub(crate) const HEADER_NUMBER_OF_READ_REGIONS: &str = "x-ms-number-of-read-regions";
pub(crate) const HEADER_REQUEST_CHARGE: &str = "x-ms-request-charge"; // [f64]
pub(crate) const HEADER_OFFER_THROUGHPUT: &str = "x-ms-offer-throughput"; // [u64]
pub(crate) const HEADER_OFFER_TYPE: &str = "x-ms-offer-type"; // [&str]
pub(crate) const HEADER_DOCUMENTDB_ISQUERY: &str = "x-ms-documentdb-isquery"; // [bool]
pub(crate) const HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION: &str =
    "x-ms-documentdb-query-enablecrosspartition"; // [bool]
pub(crate) const HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY: &str =
    "x-ms-documentdb-query-parallelizecrosspartitionquery"; // [bool]
pub(crate) const HEADER_DOCUMENTDB_EXPIRY_SECONDS: &str = "x-ms-documentdb-expiry-seconds"; // [u64]
pub(crate) const HEADER_CONTENT_PATH: &str = "x-ms-content-path"; // [String]
pub(crate) const HEADER_ALT_CONTENT_PATH: &str = "x-ms-alt-content-path"; // [String]
pub(crate) const HEADER_LAST_STATE_CHANGE_UTC: &str = "x-ms-last-state-change-utc"; // [DateTime<UTC>]
pub(crate) const HEADER_RESOURCE_QUOTA: &str = "x-ms-resource-quota"; // [ResourceQuota]
pub(crate) const HEADER_RESOURCE_USAGE: &str = "x-ms-resource-usage"; // [ResourceQuota]
pub(crate) const HEADER_QUORUM_ACKED_LSN: &str = "x-ms-quorum-acked-lsn"; // [u64]
pub(crate) const HEADER_CURRENT_WRITE_QUORUM: &str = "x-ms-current-write-quorum"; // [u64]
pub(crate) const HEADER_CURRENT_REPLICA_SET_SIZE: &str = "x-ms-current-replica-set-size"; // [u64]
