use reqwest::blocking::RequestBuilder;

/// Credentials enum, currently only supporting token authentication
#[derive(Debug)]
pub enum Credentials {
    UserAuthToken { token: String },
}

impl Credentials {
    pub fn headers(&self) -> Vec<(&'static str, String)> {
        match self {
            Self::UserAuthToken { token } => vec![("Authorization", format!("Bearer {}", token))],
        }
    }
}

pub trait AuthClient {
    fn auth(self, credentials: &Credentials) -> Self;
}

impl AuthClient for RequestBuilder {
    fn auth(mut self, credentials: &Credentials) -> Self {
        for (k, v) in credentials.headers() {
            self = self.header(k, v);
        }
        self
    }
}
