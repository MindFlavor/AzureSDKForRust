use crate::TableEntity;
use serde::Serialize;
use serde_json;

const BATCH_MAX_SIZE: usize = 100;

const BATCH_BEGIN: &str = r#"--batch_a1e9d677-b28b-435e-a89e-87e6a768a431
Content-Type: multipart/mixed; boundary=changeset_8a28b620-b4bb-458c-a177-0959fb14c977

"#;
const BATCH_END: &str = "--batch_a1e9d677-b28b-435e-a89e-87e6a768a431\n";
const CHANGESET_BEGIN: &str = r#"--changeset_8a28b620-b4bb-458c-a177-0959fb14c977
Content-Type: application/http
Content-Transfer-Encoding: binary

"#;
const CHANGESET_END: &str = "--changeset_8a28b620-b4bb-458c-a177-0959fb14c977--\n";
const UPDATE_HEADER: &str = "Content-Type: application/json\n";
const ACCEPT_HEADER: &str = "Accept: application/json;odata=nometadata\n";
const IF_MATCH_HEADER: &str = "If-Match: *\n";

quick_error! {
    #[derive(Debug)]
    pub enum BatchError {
        UnexpectedPartitionKey {
            display("Batch operation cannot be executed in multiple partitions")
        }
        TooManyOperations  {
            display("Batch operation size limit reached")
        }
        JSONError(err: serde_json::Error) {
            from()
            display("json error: {}", err)
            cause(err)
        }
    }
}

pub enum BatchOperation {
    Delete {
        row_key: String,
        etag: Option<String>,
    },

    Insert {
        row_key: String,
        payload: String,
    },

    Update {
        row_key: String,
        payload: String,
        etag: Option<String>,
    },
}

pub struct Batch {
    partition_key: String,
    items: Vec<BatchOperation>,
}

impl Batch {
    pub fn new(partition_key: String) -> Batch {
        Batch {
            partition_key: partition_key,
            items: vec![],
        }
    }

    pub fn add_operation(&mut self, op: BatchOperation) -> Result<&mut Self, BatchError> {
        self.items.push(op);
        if self.items.len() > BATCH_MAX_SIZE {
            Err(BatchError::TooManyOperations)
        } else {
            Ok(self)
        }
    }

    pub fn add_insert<T>(&mut self, row_key: String, data: &T) -> Result<&mut Self, BatchError>
    where
        T: Serialize,
    {
        self.add_operation(BatchOperation::Insert {
            row_key: row_key,
            payload: serde_json::to_string(data)?,
        })
    }

    pub fn add_insert_entity<T>(&mut self, entity: TableEntity<T>) -> Result<&mut Self, BatchError>
    where
        T: Serialize,
    {
        if entity.partition_key != self.partition_key {
            Err(BatchError::UnexpectedPartitionKey)
        } else {
            self.add_insert(entity.row_key, &entity.payload)
        }
    }

    pub fn add_update<T>(
        &mut self,
        row_key: String,
        data: &T,
        etag: Option<String>,
    ) -> Result<&mut Self, BatchError>
    where
        T: Serialize,
    {
        self.add_operation(BatchOperation::Update {
            row_key: row_key.to_owned(),
            payload: serde_json::to_string(data)?,
            etag: etag,
        })
    }

    pub fn add_update_entity<T>(&mut self, entity: TableEntity<T>) -> Result<&mut Self, BatchError>
    where
        T: Serialize,
    {
        if entity.partition_key != self.partition_key {
            Err(BatchError::UnexpectedPartitionKey)
        } else {
            self.add_update(entity.row_key, &entity.payload, entity.etag)
        }
    }

    pub fn add_delete(
        &mut self,
        row_key: String,
        etag: Option<String>,
    ) -> Result<&mut Self, BatchError> {
        self.add_operation(BatchOperation::Delete {
            row_key: row_key.to_owned(),
            etag: etag,
        })
    }

    pub fn add_delete_entity<T>(
        &mut self,
        entity: TableEntity<T>,
    ) -> Result<&mut Self, BatchError> {
        if entity.partition_key != self.partition_key {
            Err(BatchError::UnexpectedPartitionKey)
        } else {
            self.add_delete(entity.row_key, entity.etag)
        }
    }

    pub fn into_payload(self, uri_prefix: &str, table: &str) -> String {
        let mut payload: String = BATCH_BEGIN.to_owned();
        for item in self.items {
            payload.push_str(CHANGESET_BEGIN);
            /*item.into_payload(&mut payload);

            /*payload.push_str(if item.1.is_some() { "PUT" } else { "DELETE" });
            payload.push_str(" ");
            payload.push_str(uri_prefix);
            payload.push_str(entity_path(table, primary_key, item.0.as_str()).as_str());*/

            payload.push_str(" HTTP/1.1\n");
            payload.push_str(ACCEPT_HEADER);
            if let Some(ref v) = item.1 {
                payload.push_str(UPDATE_HEADER);
                payload.push_str("\n");
                payload.push_str(serde_json::to_string(v).unwrap().as_str());
            } else {
                payload.push_str(IF_MATCH_HEADER);
            }*/

            payload.push_str("\n");
        }
        payload + CHANGESET_END + BATCH_END
    }
}
