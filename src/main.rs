use std::path::Path;
use std::sync::Arc;
use std::process::Command;

use plaid_cli::{
    endpoints,
    credentials_provider::{
        DefaultCredentialsProvider,
        DynCredentialsProvider
    }
};

use axum::{
    routing::{get, post},
    Router,
    Extension
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // instantiate credentials provider
    let credentials_provider = Arc::new(
        DefaultCredentialsProvider::new(Path::new("./credentials.txt"))?
    ) as DynCredentialsProvider;

    // build web server
    let app = Router::new()
        .route("/", get(endpoints::root))
        .route("/create_link_token", get(endpoints::create_link_token))
        .route("/exchange_public_token", post(endpoints::exchange_public_token))
        .layer(Extension(credentials_provider));

    // run web server
    let server = axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service());

    let res = Command::new("xdg-open")
        .arg(format!("http://{}", server.local_addr()))
        .output();

    if let Err(e) = res {
        eprintln!("failed to launch xdg-open: {}", e);
        eprintln!("auto-open failed. navigate to http://{} in your web browser manually",
                  server.local_addr());
    }

    if let Err(e) = server.await {
        eprintln!("http server error: {}", e);
    }

    Ok(())
}
