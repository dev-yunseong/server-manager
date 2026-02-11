use async_trait::async_trait;
use derive_new::new;
use crate::application::client::MessageGateway;
use crate::application::config::AuthUseCase;
use crate::application::handler::MessageHandler;
use crate::domain::client::Message;

#[derive(new)]
pub struct AuthHandler {
    message_gateway: Box<dyn MessageGateway>,
    message_handler: Box<dyn MessageHandler>,
    auth_handler: Box<dyn AuthUseCase>
}

#[async_trait]
impl MessageHandler for AuthHandler {
    async fn handle(&mut self, message: Message) {
        match message.data.split_whitespace().collect::<Vec<_>>()[..] {
            ["/register", password] => {
                let response = if self.auth_handler.validate_password(password.to_string()).await {
                    self.auth_handler.register(message.client_name.clone(), message.chat_id.clone()).await;
                    "Successfully registered."
                } else {
                    "Invalid password. Usage: /register <password>"
                };
                self.message_gateway.send_message(
                    message.client_name.as_str(),
                    message.chat_id.as_str(),
                    response
                ).await
            },
            _ => {
                if (self.auth_handler.authenticate(message.client_name.clone(), message.chat_id.clone()).await) {
                    self.message_handler.handle(message).await
                } else {
                    self.message_gateway.send_message(
                        message.client_name.as_str(),
                        message.chat_id.as_str(),
                        "Registration required. Usage: /register <password>"
                    ).await
                }
            }
        }
    }
}