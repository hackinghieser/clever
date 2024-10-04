use crate::event::Event;

pub struct EventCollection {
    pub events: Vec<Event>,
}

impl EventCollection {
    pub fn create(events: Vec<String>) -> Option<Self> {
        let mut event_collection = EventCollection { events: vec![] };
        let event_list = EventCollection::read_events(&event_collection, events);
        event_collection.events = event_list;
        println!("Event collection length: {}", event_collection.events.len());
        Some(event_collection)
    }

    fn read_events(&self, events: Vec<String>) -> Vec<Event> {
        println!("Event Count: {0}", events.len());
        let mut event_collection: Vec<Event> = Vec::new();
        for event in events {
            let event = Event::create(event.to_string()).unwrap();
            event_collection.push(event);
        }
        event_collection
    }
}
