use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::{Arc, Mutex},
    time::Instant,
};

use cleverlib::event_collection::EventCollection;
fn main() {
    if let Ok(lines) = read_lines("src/example.clef") {
        // Consumes the iterator, returns an (Optional) String
        let tick = Arc::new(Mutex::new(0));
        let mut now = Instant::now();
        let c: Vec<String> = lines.flatten().collect();
        let count = c.len();
        now = Instant::now();

        let collection = EventCollection::create_par(c.clone()).unwrap();
        println!("Create collection");
        println!("Events: {}", collection.events.len());
        println!("Tokens: {}", collection.events[0].tokens.len());
        println!("Elapsed serie: {:?}", now.elapsed());

        now = Instant::now();

        let collection = EventCollection::create(c.clone()).unwrap();
        println!("Create collection");
        println!("Events: {}", collection.events.len());
        println!("Tokens: {}", collection.events[0].tokens.len());
        println!("Elapsed serie: {:?}", now.elapsed());
    }

    println!("done")
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
