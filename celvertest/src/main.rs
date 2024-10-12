use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::{Arc, Mutex},
    time::Instant,
};

use cleverlib::{
    event,
    event_collection::{self, EventCollection},
};
fn main() {
    if let Ok(lines) = read_lines("src/example1.clef") {
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

        let result = collection.filter_log_level("Debug");
        println!("Result Filter Debug: Count {} ", result.len());
        let result = collection.filter_log_level("Warning");
        println!("Result Filter Warning: Count: {} ", result.len(),);

        println!("done")
    }
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
