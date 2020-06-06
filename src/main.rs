extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::{Region, RusotoError};
use rusoto_dynamodb::*;

use std::default::Default;
use std::collections::HashMap;
use std::result::Result;
use std::error::Error;

fn main() {
    println!("Hello, world!");
}

fn get_client() -> DynamoDbClient {
    let client = DynamoDbClient::new(Region::UsEast1);
    return client;
}

#[derive(Default)]
struct User {
    id: Option<String>,
    first_name: String,
    last_name: String,
}


async fn get_item(client: &DynamoDbClient, id: String) -> Result<User, &'static str> {
    let mut get_key = HashMap::new();
    get_key.insert(
        "id".to_string(),
        AttributeValue {
            s: Some("new_id_TODO".to_string()),
            ..Default::default()
        },
    );
    let input: GetItemInput = GetItemInput{
        table_name: "test".to_string(),
        key: get_key,
        ..Default::default()
    };

    match client.get_item(input).await {
        Err(error) => {
            println!("Error: {:?}", error);
            Err("error")
        }
        Ok(output) => {
            let item = match output.item {
                None => {
                    println!("no item present");
                    return Err("error, no item present")
                },
                Some(i) => i
            };
            Ok(User {
                id: match item.get("id") {
                    None => {
                        return Err("no id present");
                    },
                    Some(id) => id.s.clone()
                },
                first_name: match item.get("first_name") {
                    None => {
                        return Err("no first_name present");
                    },
                    Some(fname) => fname.s.as_ref().unwrap().to_string()
                },
                last_name: match item.get("last_name") {
                    None => {
                        return Err("no last_name present");
                    },
                    Some(ln) => ln.s.as_ref().unwrap().to_string()
                }
            })
        }
    }
}