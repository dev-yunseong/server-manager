mod runner;
mod registrar;

use std::collections::HashMap;
use async_trait::async_trait;

#[async_trait]
pub trait Worker {
    async fn on_tick(&mut self);
    fn get_name(&self) -> &str;
    fn interval(&self) -> i32;
}

pub struct WorkerRegistry {
    workers: HashMap<String, Box<dyn Worker>>
}

impl WorkerRegistry {

    fn new() -> Self {
        Self {
            workers: HashMap::new()
        }
    }

    fn register(&mut self, worker: Box<dyn Worker>) {
        self.workers.insert(
            worker.get_name().to_string(),
            worker
        );
    }
}