use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::event::Event;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::prelude::*;
use regex::Regex;

#[derive(Default)]
pub struct EventCollection {
    pub events: Vec<Event>,
    pub log_levels: Vec<String>,
}

impl Debug for EventCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventCollection")
            .field("events", &self.events)
            .finish()
    }
}

impl EventCollection {
    pub fn create(events: &Vec<String>) -> Option<Self> {
        let mut event_collection = EventCollection {
            events: vec![],
            log_levels: vec![],
        };
        let event_list = EventCollection::read_events_serie(&mut event_collection, events);
        event_collection.events = event_list;
        Some(event_collection)
    }

    pub fn create_par(events: &Vec<String>) -> Option<Self> {
        let mut event_collection = EventCollection {
            events: vec![],
            log_levels: vec![],
        };
        let event_list = EventCollection::read_events_par(&mut event_collection, events);
        event_collection.events = event_list;
        Some(event_collection)
    }

    pub fn create_cp<T: Fn(usize) + Send + Sync + 'static>(
        events: Vec<String>,
        callback: T,
    ) -> Option<Self> {
        let mut event_collection = EventCollection {
            events: vec![],
            log_levels: vec![],
        };
        let event_list = EventCollection::read_events_cp(&event_collection, events, callback);
        event_collection.events = event_list;
        Some(event_collection)
    }

    fn read_events_cp<T: Fn(usize) + Send + Sync + 'static>(
        &self,
        events: Vec<String>,
        callback: T,
    ) -> Vec<Event> {
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

    pub fn filter_log_level_par(&self, log_level: &str) -> Vec<&Event> {
        self.events
            .par_iter()
            .filter(|event| event.level.clone().unwrap().eq_ignore_ascii_case(log_level))
            .collect()
    }

    pub fn filter_log_level(&self, log_level: &str) -> Vec<&Event> {
        self.events
            .par_iter()
            .filter(|event| {
                event
                    .level
                    .clone()
                    .unwrap_or_default()
                    .eq_ignore_ascii_case(log_level)
            })
            .collect()
    }

    fn read_events_serie(&mut self, events: &[String]) -> Vec<Event> {
        let re = Regex::new(r"\{(\w+|\d+)\}").unwrap();
        let mut log_levels: Vec<String> = vec![];
        let event_collection = events
            .iter()
            .progress()
            .map(|e| {
                let e = Event::create(e.to_string(), &re).unwrap();
                if let Some(level) = e.level.clone() {
                    if !log_levels.contains(&level) {
                        log_levels.push(level)
                    }
                }
                e
            })
            .collect();
        self.log_levels = log_levels;
        event_collection
    }

    fn read_events_par(&mut self, events: &Vec<String>) -> Vec<Event> {
        let re = Regex::new(r"\{(\w+|\d+)\}").unwrap();
        let log_levels: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
        let event_collection = events
            .par_iter()
            .progress()
            .map(|e| {
                let e = Event::create(e.to_string(), &re).unwrap();
                if let Some(level) = e.level.clone() {
                    log_levels.lock().unwrap().push(level)
                }
                e
            })
            .collect();
        self.log_levels = Arc::try_unwrap(log_levels).unwrap().into_inner().unwrap();
        event_collection
    }
}
