use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug)]
pub enum KeyKind {
    Hash,
    Range,
    Spatial,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DataType {
    String,
    Number,
    Point,
    Polygon,
    LineString,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IndexingMode {
    #[serde(rename = "consistent")]
    Consistent,
    #[serde(rename = "lazy")]
    Lazy,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncludedPath {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "indexes")]
    pub indexes: Vec<IncludedPathIndex>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncludedPathIndex {
    #[serde(rename = "dataType")]
    pub data_type: DataType,
    #[serde(rename = "precision")]
    pub precision: Option<i8>,
    #[serde(rename = "kind")]
    pub kind: KeyKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExcludedPath {
    #[serde(rename = "path")]
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartitionKey {
    pub paths: Vec<String>,
    pub kind: KeyKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexingPolicy {
    #[serde(rename = "automatic")]
    pub automatic: bool,
    #[serde(rename = "indexingMode")]
    pub indexing_mode: IndexingMode,
    #[serde(rename = "includedPaths")]
    pub included_paths: Vec<IncludedPath>,
    #[serde(rename = "excludedPaths")]
    pub excluded_paths: Vec<ExcludedPath>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub id: String,
    #[serde(rename = "indexingPolicy")]
    pub indexing_policy: IndexingPolicy,
    #[serde(rename = "partitionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parition_key: Option<PartitionKey>,
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_ts")]
    pub ts: u64,
    #[serde(rename = "_self")]
    pub _self: String,
    #[serde(rename = "_etag")]
    pub etag: String,
    #[serde(rename = "_docs")]
    pub docs: String,
    #[serde(rename = "_sprocs")]
    pub sprocs: String,
    #[serde(rename = "_triggers")]
    pub triggers: String,
    #[serde(rename = "_udfs")]
    pub udfs: String,
    #[serde(rename = "_conflicts")]
    pub conflicts: String,
}

impl Collection {
    pub fn new(id: &str, indexing_policy: IndexingPolicy) -> Collection {
        Collection {
            id: id.to_owned(),
            indexing_policy: indexing_policy,
            parition_key: None,
            rid: "".to_owned(),
            ts: 0,
            _self: "".to_owned(),
            etag: "".to_owned(),
            docs: "".to_owned(),
            sprocs: "".to_owned(),
            triggers: "".to_owned(),
            udfs: "".to_owned(),
            conflicts: "".to_owned(),
        }
    }
}

impl Deref for Collection {
    type Target = str;

    fn deref(&self) -> &str {
        &self.id
    }
}

impl AsRef<str> for Collection {
    fn as_ref(&self) -> &str {
        &self.id
    }
}