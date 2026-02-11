use async_trait::async_trait;
use crate::application::config::AuthUseCase;
use crate::infrastructure::config;

pub struct AuthAdapter {
    password: Option<String>
}

impl AuthAdapter {
    pub fn new() -> Self {
        Self {
            password: None
        }
    }
}

#[async_trait]
impl AuthUseCase for AuthAdapter {
    async fn set_password(&self, password: String) {
        let mut config = config::read().await;
        config.password = password;
        config::write(config).await;
    }

    async fn validate_password(&mut self, password: String) -> bool {
        match &self.password {
            Some(password) => password.eq(password.as_str()),
            None => {
                let config = config::read().await;
                self.password = Some(config.password);
                self.password.as_ref().unwrap().as_str().eq(password.as_str())
            }
        }
    }

    async fn register(&self, client_name: String, identity: String) {
        todo!()
    }

    async fn authenticate(&mut self, client_name: String, identity: String) -> bool {
        todo!()
    }
}