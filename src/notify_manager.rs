use std::string::String;
use std::sync::Arc;
use std::sync::Mutex;

pub type Message = String;

pub trait Observer {
    fn message_received(&mut self, msg: &Message);
}

pub type ObserverObject = Arc<Mutex<dyn Observer + Send>>;
// pub type ObserverObject = Observer;
pub type Observers = Vec<ObserverObject>;

pub struct NotifytManager {
    observers: Observers,
}

impl NotifytManager {
    pub fn new() -> NotifytManager {
        NotifytManager {
            observers: Vec::new(),
        }
    }
    pub fn add_observer(&mut self, observer: ObserverObject) {
        self.observers.push(observer);
    }

    pub fn notify(&mut self, msg: &Message) {
        for observer in self.observers.iter() {
            let mut observer = observer.lock().unwrap();
            observer.message_received(msg);
        }
    }
}
