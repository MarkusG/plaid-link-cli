use axum::{
    async_trait,
    routing::{get, post},
    response::Html,
    Json,
    Router,
    Extension
};
use hyper_tls::HttpsConnector;
use hyper::{Client, Body, Method, Request};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::sync::Arc;
use serde::Deserialize;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // instantiate credentials provider
    let credentials_file = File::open(Path::new("./credentials.txt"))?;
    let reader = BufReader::new(credentials_file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let credentials_provider = Arc::new(DefaultCredentialsProvider {
        client_id: lines[0].clone(),
        client_secret: lines[1].clone()
    }) as DynCredentialsProvider;

    // build web server
    let app = Router::new()
        .route("/", get(root))
        .route("/create_link_token", get(create_link_token))
        .route("/exchange_public_token", post(exchange_public_token))
        .layer(Extension(credentials_provider));

    // run web server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

struct DefaultCredentialsProvider {
    pub client_id: String,
    pub client_secret: String,
}

#[async_trait]
impl CredentialsProvider for DefaultCredentialsProvider {
    async fn get_client_id(&self) -> &str {
        return &self.client_id;
    }

    async fn get_client_secret(&self) -> &str {
        return &self.client_secret;
    }
}

type DynCredentialsProvider = Arc<dyn CredentialsProvider + Send + Sync>;

#[async_trait]
trait CredentialsProvider {
    async fn get_client_id(&self) -> &str;

    async fn get_client_secret(&self) -> &str;
}

async fn root() -> Html<String> {
    let page = fs::read_to_string("link.html").unwrap();
    Html(page)
}

async fn create_link_token(Extension(credentials): Extension<DynCredentialsProvider>) -> String {
    let client_id = credentials.get_client_id().await;
    let client_secret = credentials.get_client_secret().await;
    if let Ok(t) = get_link_token(client_id, client_secret).await {
        t
    } else {
        panic!();
    }
}

async fn exchange_public_token(Json(payload): Json<PublicToken>) -> String {
    println!("{:?}", payload);
    String::new()
}

#[derive(Debug)]
#[derive(Deserialize)]
struct PublicToken {
    public_token: String
}

async fn get_link_token(client_id: &str, client_secret: &str)
    -> Result<String, Box<dyn std::error::Error>> {
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

    let req = Request::builder()
        .method(Method::POST)
        .uri("https://sandbox.plaid.com/link/token/create")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let resp = client.request(req).await?;
    let bytes = hyper::body::to_bytes(resp.into_body()).await?;
    Ok(String::from_utf8(bytes.to_vec())?)
}
