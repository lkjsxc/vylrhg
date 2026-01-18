use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug, Clone)]
pub enum Event {
    Tick,
    Input(String),
    Shutdown,
}

pub struct EventBus {
    tx: Sender<Event>,
    rx: Receiver<Event>,
}

impl EventBus {
    pub fn new(buffer: usize) -> Self {
        let (tx, rx) = mpsc::channel(buffer);
        Self { tx, rx }
    }

    pub fn sender(&self) -> Sender<Event> {
        self.tx.clone()
    }

    pub async fn recv(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}
