use super::{
    EventType,
    event::{Event, EventTrait},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type EventHandler = Arc<dyn Fn(Event) + Send + Sync>;

#[derive(Default, Clone)]
pub struct EventBus {
    pub listeners: Arc<Mutex<HashMap<EventType, Vec<EventHandler>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe<F>(&self, event_type: EventType, callback: F)
    where
        F: Fn(Event) + Send + Sync + 'static,
    {
        let mut listeners = self.listeners.lock().unwrap();
        listeners
            .entry(event_type)
            .or_default()
            .push(Arc::new(callback));
    }

    pub fn publish(&self, event: Event) {
        let listeners = self.listeners.lock().unwrap();
        if let Some(handlers) = listeners.get(&event.get_type()) {
            for handler in handlers {
                handler(event.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{GameEvent, event::Event};
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_event_bus() {
        let event_bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        event_bus.subscribe(EventType::Game, move |event| {
            if let Event::Game(GameEvent::Started) = event {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });

        event_bus.publish(Event::Game(GameEvent::Started));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
