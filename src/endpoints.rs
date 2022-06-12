use std::fs;

use crate::credentials_provider::DynCredentialsProvider;

use crate::get_link_token;

use axum::response::Html;
use axum::Extension;
use axum::Json;
use serde::Deserialize;

pub async fn root() -> Html<String> {
    // serve page to begin the link flow
    let page = fs::read_to_string("link.html").unwrap();
    Html(page)
}

pub async fn create_link_token(
    Extension(credentials): Extension<DynCredentialsProvider>)
    -> String {
    // get credentials
    let client_id = credentials.get_client_id().await;
    let client_secret = credentials.get_client_secret().await;

    // get link token
    if let Ok(t) = get_link_token(client_id, client_secret).await {
        t
    } else {
        panic!();
    }
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct PublicToken {
    public_token: String
}

pub async fn exchange_public_token(Json(payload): Json<PublicToken>) -> String {
    // TODO
    println!("{:?}", payload);
    String::new()
}
