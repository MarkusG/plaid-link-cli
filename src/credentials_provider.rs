use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::Arc;

use axum::async_trait;

pub struct DefaultCredentialsProvider {
    pub client_id: String,
    pub client_secret: String,
}

impl DefaultCredentialsProvider {
    pub fn new(path: &Path) -> Result<DefaultCredentialsProvider, Box<dyn Error>> {
        let credentials_file = File::open(path)?;
        let reader = BufReader::new(credentials_file);
        let lines: Vec<String> = reader
            .lines()
            .map(|l| l.unwrap())
            .collect();

        // first line is client id, second line is client secret
        Ok(DefaultCredentialsProvider {
            client_id: lines[0].clone(),
            client_secret: lines[1].clone()
        })
    }
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

pub type DynCredentialsProvider = Arc<dyn CredentialsProvider + Send + Sync>;

#[async_trait]
pub trait CredentialsProvider {
    async fn get_client_id(&self) -> &str;

    async fn get_client_secret(&self) -> &str;
}
