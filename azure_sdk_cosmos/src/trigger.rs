create_enum!(
    Operation,
    (All, "All"),
    (Create, "Create"),
    (Replace, "Replace"),
    (Delete, "Delete")
);

create_enum!(Type, (Pre, "Pre"), (Post, "Post"));

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Trigger {
    pub id: String,
    pub operation: Operation,
    pub r#type: Type,
    pub body: String,
}
