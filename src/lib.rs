use serde_json::Value;
use serde::Deserialize;
use mongodb::{Client, options::ClientOptions};
use std::error::Error;
use std::env;

pub async fn create_client() -> Result<Client, Box<dyn Error>> {
    let client_options = ClientOptions::parse(&env::var("MONGODBENDPOINT")?).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub resource: String,
    pub path: String,
    pub http_method: String,
    pub query_string_parameters: Value,
    pub body: Value,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
