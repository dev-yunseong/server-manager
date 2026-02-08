mod telegram_client;

trait Client {
    async fn send_message(&self, chat_id: &str, data: &str) -> bool;
    fn set_callback(&mut self, callback: impl Fn(&str) + 'static);
}