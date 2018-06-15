extern crate azure_sdk_for_rust;

extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use std::error::Error;

use tokio_core::reactor::Core;

use azure_sdk_for_rust::cosmos::{query::Query, AuthorizationToken, Client, TokenType};

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct MySampleStructOwned {
    id: String,
    a_string: String,
    a_number: u64,
    a_timestamp: i64,
}

fn main() {
    code().unwrap();
}

fn code() -> Result<(), Box<Error>> {
    let database_name = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");
    let query = std::env::args().nth(3).expect("please specify requested query");

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key = std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new(account, TokenType::Master, &master_key)?;

    let mut core = Core::new()?;

    let client = Client::new(authorization_token)?;

    let future = client
        .query_document(&database_name, &collection_name, &Query::from(&query as &str))
        .execute_json();

    let ret = core.run(future)?;

    println!("As JSON:\n{:?}", ret);

    for doc in ret.results {
        println!("{}", doc.result);
    }

    let future = client
        .query_document(&database_name, &collection_name, &Query::from(&query as &str))
        .execute::<MySampleStructOwned>();

    let ret = core.run(future)?;

    println!("\nAs entities:\n{:?}", ret);

    for doc in ret.results {
        println!("{:?}", doc);
    }

    // test continuation token
    // only if we have more than 2 records
    let future = client
        .query_document(&database_name, &collection_name, &Query::from(&query as &str))
        .max_item_count(2u64)
        .execute::<MySampleStructOwned>();

    let ret = core.run(future)?;

    println!(
        "Received {} entries. Continuation token is == {:?}",
        ret.results.len(),
        ret.additional_headers.continuation_token
    );

    if let Some(ct) = ret.additional_headers.continuation_token {
        let ret = {
            // if we have more, let's get them
            let future = client
                .query_document(&database_name, &collection_name, &Query::from(&query as &str))
                .continuation_token(ct)
                .execute::<MySampleStructOwned>();
            core.run(future)?
        };
        println!(
            "Received {} entries. Continuation token is == {:?}",
            ret.results.len(),
            ret.additional_headers.continuation_token
        );
    }

    Ok(())
}
