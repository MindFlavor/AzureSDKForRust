#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Attachment {
    #[serde(rename = "contentType")]
    content_type: String,
    id: String,
    media: String,
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "_ts")]
    pub ts: u64,
    pub _self: String,
    #[serde(rename = "_etag")]
    pub etag: String,
}

impl crate::attachment::AttachmentName for Attachment {
    fn name(&self) -> &str {
        &self.id
    }
}
