use std::{fs, io::BufRead, time::Instant};

use cleverlib::event_collection::EventCollection;
fn main() {
    println!("Hello, world!");
    {
        let file = fs::read("./example.clef").expect("No file found !");
        let lines = file.lines().map(|s| s.unwrap()).collect();
        println!("Create collection");

        let now = Instant::now();
        let collection = EventCollection::create(lines).unwrap();
        println!("Events: {}", collection.events.len());
        println!("Tokens: {}", collection.events[0].tokens.len());
        let mut index: i32 = 0;
        for event in collection.events {
            let rendered_event = event
                .tokens
                .iter()
                .map(|x| x.render())
                .collect::<Vec<String>>();

            println!(
                "#{} @t[{}] @l[{}] @t: {} @mt: {}",
                index,
                &&event.time.unwrap(),
                &event.level.unwrap_or_default(),
                &event.template.unwrap(),
                rendered_event.join(" ")
            );
            index = index.checked_add(1).unwrap();
        }
        println!("End {}", now.elapsed().as_millis());
    }
}
