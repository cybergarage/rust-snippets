use std::string::String;
use std::sync::Arc;
use std::sync::Mutex;

pub type Event = String;

pub trait Observer {
    fn event_received(&mut self, e: &Event);
}

pub type ObserverObject = Arc<Mutex<dyn Observer + Send>>;
// pub type ObserverObject = Observer;
pub type Observers = Vec<ObserverObject>;

pub struct Subject {
    observers: Observers,
}

impl Subject {
    pub fn new() -> Subject {
        Subject {
            observers: Vec::new(),
        }
    }
    pub fn add_observer(&mut self, observer: ObserverObject) {
        self.observers.push(observer);
    }

    pub fn notify(&mut self, e: &Event) {
        for observer in self.observers.iter() {
            let mut observer = observer.lock().unwrap();
            observer.event_received(e);
        }
    }
}
