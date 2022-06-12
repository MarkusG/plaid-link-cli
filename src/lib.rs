pub mod credentials_provider;
pub mod endpoints;

use hyper_tls::HttpsConnector;
use hyper::{Client, Body, Method, Request};
use serde_json::json;

pub async fn get_link_token(client_id: &str, client_secret: &str)
    -> Result<String, Box<dyn std::error::Error>> {
    // build request body
    let body = json!(
        {
            "client_id": client_id,
            "secret": client_secret,
            "user": { "client_user_id": "unique-per-user" },
            "client_name": "Plaid App",
            "products": ["auth"],
            "country_codes": ["US"],
            "language": "en"
        });

    // build request
    let req = Request::builder()
        .method(Method::POST)
        .uri("https://sandbox.plaid.com/link/token/create")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))?;

    // create client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // send request
    let resp = client.request(req).await?;

    // get response bytes
    let bytes = hyper::body::to_bytes(resp.into_body()).await?;

    // parse to string and return
    Ok(String::from_utf8(bytes.to_vec())?)
}
