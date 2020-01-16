use crate::errors::{item_or_error, TokenParsingError};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DatabaseResourceQuota {
    pub databases: u64,
    pub collections: u64,
    pub users: u64,
    pub permissions: u64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DocumentResourceQuota {
    pub document_size: u64,
    pub documents_size: u64,
    pub collection_size: u64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ResourceQuota {
    Databases(DatabaseResourceQuota),
    StoredProcedures(u64),
    Collections(u64),
    Documents(DocumentResourceQuota),
    Users(u64),
    Permissions(u64),
    Triggers(u64),
    Functions(u64),
}

impl std::convert::TryFrom<&str> for ResourceQuota {
    type Error = failure::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        println!("ResourceQuota::try_into({})", s);

        if s.starts_with("databases=") {
            let tokens: Vec<&str> = s.split(';').collect();
            Ok(ResourceQuota::Databases(DatabaseResourceQuota {
                databases: str::parse(item_or_error(s, &tokens, "databases=")?)?,
                collections: str::parse(item_or_error(s, &tokens, "collections=")?)?,
                users: str::parse(item_or_error(s, &tokens, "users=")?)?,
                permissions: str::parse(item_or_error(s, &tokens, "permissions=")?)?,
            }))
        } else if s.starts_with("storedProcedures=") {
            Ok(ResourceQuota::StoredProcedures(str::parse(
                &s["storedProcedures=".len()..s.len() - 1],
            )?))
        } else if s.starts_with("collections=") {
            Ok(ResourceQuota::Collections(str::parse(
                &s["collections=".len()..s.len() - 1],
            )?))
        } else if s.starts_with("documentSize=") {
            let tokens: Vec<&str> = s.split(';').collect();
            Ok(ResourceQuota::Documents(DocumentResourceQuota {
                document_size: str::parse(item_or_error(s, &tokens, "documentSize=")?)?,
                documents_size: str::parse(item_or_error(s, &tokens, "documentsSize=")?)?,
                collection_size: str::parse(item_or_error(s, &tokens, "collectionSize=")?)?,
            }))
        } else if s.starts_with("users=") {
            Ok(ResourceQuota::Users(str::parse(
                &s["users=".len()..s.len() - 1],
            )?))
        } else if s.starts_with("permissions=") {
            Ok(ResourceQuota::Permissions(str::parse(
                &s["permissions=".len()..s.len() - 1],
            )?))
        } else if s.starts_with("triggers=") {
            Ok(ResourceQuota::Triggers(str::parse(
                &s["triggers=".len()..s.len() - 1],
            )?))
        } else if s.starts_with("functions=") {
            Ok(ResourceQuota::Functions(str::parse(
                &s["functions=".len()..s.len() - 1],
            )?))
        } else {
            Err(TokenParsingError::UnsupportedStartingString { s: s.to_owned() }.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn parse_resource_quota() {
        let resource_quota: ResourceQuota = "storedProcedures=25;".try_into().unwrap();
        assert_eq!(resource_quota, ResourceQuota::StoredProcedures(25));

        let resource_quota: ResourceQuota =
            "databases=100;collections=5000;users=500000;permissions=2000000;"
                .try_into()
                .unwrap();

        let database_resource_quota =
            if let ResourceQuota::Databases(database_resource_quota) = resource_quota {
                database_resource_quota
            } else {
                panic!()
            };
        assert_eq!(database_resource_quota.databases, 100);
        assert_eq!(database_resource_quota.collections, 5000);
        assert_eq!(database_resource_quota.users, 500000);
        assert_eq!(database_resource_quota.permissions, 2000000);

        let resource_quota: ResourceQuota = "collections=27;".try_into().unwrap();
        assert_eq!(resource_quota, ResourceQuota::Collections(27));

        let resource_quota: ResourceQuota = "documentSize=0;documentsSize=2;collectionSize=3;"
            .try_into()
            .unwrap();

        let document_resource_quota =
            if let ResourceQuota::Documents(document_resource_quota) = resource_quota {
                document_resource_quota
            } else {
                panic!()
            };
        assert_eq!(document_resource_quota.document_size, 0);
        assert_eq!(document_resource_quota.documents_size, 2);
        assert_eq!(document_resource_quota.collection_size, 3);

        let resource_quota: ResourceQuota = "users=500000;".try_into().unwrap();
        assert_eq!(resource_quota, ResourceQuota::Users(500000));

        let resource_quota: ResourceQuota = "permissions=2000000;".try_into().unwrap();
        assert_eq!(resource_quota, ResourceQuota::Permissions(2000000));

        let resource_quota: ResourceQuota = "triggers=25;".try_into().unwrap();
        assert_eq!(resource_quota, ResourceQuota::Triggers(25));

        let resource_quota: ResourceQuota = "functions=26;".try_into().unwrap();
        assert_eq!(resource_quota, ResourceQuota::Functions(26));
    }
}
