use async_trait::async_trait;

#[async_trait]
pub trait Worker: Send {
    async fn on_tick(&mut self) -> bool; // 비즈니스 로직 실행
    fn get_name(&self) -> &str;
    fn interval(&self) -> i32;
}