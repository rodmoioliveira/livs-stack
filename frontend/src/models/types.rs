use std::env;

#[derive(Debug, Clone)]
pub struct Endpoints {
    pub assets: String,
    pub backend: String,
}

impl Endpoints {
    pub fn new() -> Self {
        let assets = format!("http://{}", env::var("ENDPOINT_ASSETS").unwrap());
        let backend = format!("http://{}", env::var("ENDPOINT_BACKEND").unwrap());
        Self { assets, backend }
    }

    pub fn backend_url(&self, route: &str) -> String {
        format!("{}/{}", self.backend, route)
    }

    pub fn assets_url(&self, route: &str) -> String {
        format!("{}/{}", self.assets, route)
    }
}
