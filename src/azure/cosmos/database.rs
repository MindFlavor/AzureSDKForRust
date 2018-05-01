use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub id: String,
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_ts")]
    pub ts: u64,
    #[serde(rename = "_self")]
    pub _self: String,
    #[serde(rename = "_etag")]
    pub etag: String,
    #[serde(rename = "_colls")]
    pub colls: String,
    #[serde(rename = "_users")]
    pub users: String,
}

impl Deref for Database {
    type Target = str;

    fn deref(&self) -> &str {
        &self.id
    }
}

impl AsRef<str> for Database {
    fn as_ref(&self) -> &str {
        &self.id
    }
}