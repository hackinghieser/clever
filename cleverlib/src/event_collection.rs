use std::sync::{Arc, Mutex};

use crate::event::Event;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::prelude::*;
use regex::Regex;

pub struct EventCollection {
    pub events: Vec<Event>,
}

impl EventCollection {
    pub fn create(events: Vec<String>) -> Option<Self> {
        let mut event_collection = EventCollection { events: vec![] };
        let event_list = EventCollection::read_events_serie(&event_collection, events);
        event_collection.events = event_list;
        println!("Event collection length: {}", event_collection.events.len());
        Some(event_collection)
    }

    pub fn create_par(events: Vec<String>) -> Option<Self> {
        let mut event_collection = EventCollection { events: vec![] };
        let event_list = EventCollection::read_events_par(&event_collection, events);
        event_collection.events = event_list;
        println!("Event collection length: {}", event_collection.events.len());
        Some(event_collection)
    }

    pub fn create_cp<T: Fn(usize) + Send + Sync + 'static>(
        events: Vec<String>,
        callback: T,
    ) -> Option<Self> {
        let mut event_collection = EventCollection { events: vec![] };
        let event_list = EventCollection::read_events_cp(&event_collection, events, callback);
        event_collection.events = event_list;
        println!("Event collection length: {}", event_collection.events.len());
        Some(event_collection)
    }

    fn read_events_cp<T: Fn(usize) + Send + Sync + 'static>(
        &self,
        events: Vec<String>,
        callback: T,
    ) -> Vec<Event> {
        println!("Event Count: {0}", events.len());
        let re = Regex::new(r"\{(\w+|\d+)\}").unwrap();
        let event_collection = events
            .par_iter()
            .enumerate()
            .map(|e| {
                callback(0);
                Event::create(e.1.to_string(), &re).unwrap()
            })
            .collect();
        event_collection
    }

    fn read_events_serie(&self, events: Vec<String>) -> Vec<Event> {
        println!("Event Count: {0}", events.len());
        let re = Regex::new(r"\{(\w+|\d+)\}").unwrap();
        let event_collection = events
            .iter()
            .progress()
            .map(|e| Event::create(e.to_string(), &re).unwrap())
            .collect();
        event_collection
    }

    fn read_events_par(&self, events: Vec<String>) -> Vec<Event> {
        println!("Event Count: {0}", events.len());
        let re = Regex::new(r"\{(\w+|\d+)\}").unwrap();
        let event_collection = events
            .par_iter()
            .progress()
            .map(|e| Event::create(e.to_string(), &re).unwrap())
            .collect();
        event_collection
    }
}
