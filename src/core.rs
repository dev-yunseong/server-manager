
pub trait Worker {
    async fn on_tick(&mut self);
    fn get_name(&self) -> &str;
    fn interval(&self) -> i32;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}