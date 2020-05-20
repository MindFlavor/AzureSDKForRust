use std::borrow::Cow;

create_enum!(
    Operation,
    (All, "All"),
    (Create, "Create"),
    (Replace, "Replace"),
    (Delete, "Delete")
);

create_enum!(Type, (Pre, "Pre"), (Post, "Post"));

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trigger<'a> {
    pub id: Cow<'a, str>,
    pub operation: Operation,
    pub r#type: Type,
    pub body: Cow<'a, str>,
}
