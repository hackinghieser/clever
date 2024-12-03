# CleverLib

A flexible Rust library for processing and analyzing log events with parallel and serial processing strategies.

## Features

- Multiple event processing strategies
  - Serial processing
  - Parallel processing
- Efficient log event parsing
- Automatic log level detection
- Simple and intuitive API

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
cleverlib = "0.1.1"
```

## Basic Usage

```rust
use cleverlib::event_collection::EventCollection;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read log file lines
    let file: File = File::open("events.log").expect("Failed to open log file");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()
        .expect("Failed to read lines");

    // Create event collection
    let event_collection: EventCollection = EventCollection::create(&lines).unwrap();

    // Print detected log levels
    println!("Log Levels: {:?}", event_collection.log_levels);

    // Filter events by log level
    let error_events: Vec<&Event> = event_collection.filter_log_level("error");
    println!("Error Events: {}", error_events.len());
}
```

## Parallel Processing

```rust
use cleverlib::event_collection::EventCollection;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file: File = File::open("large_events.log").expect("Failed to open log file");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()
        .expect("Failed to read lines");

    // Use parallel processing for large log files
    let event_collection: EventCollection = EventCollection::create_par(&lines).unwrap();

    println!("Total Events: {}", event_collection.events.len());
    println!("Unique Log Levels: {:?}", event_collection.log_levels);
}
```

## Performance Strategies

1. **Serial Processing**: `create()` - Best for smaller log files
2. **Parallel Processing**: `create_par()` - Recommended for large log files

## Key Methods

- `create(&lines)`: Process events serially
- `create_par(&lines)`: Process events in parallel
- `filter_log_level(level)`: Filter events by log level
- `filter_log_level_par(level)`: Parallel log level filtering

## Log Level Detection

The library automatically detects unique log levels during event processing. Detected levels are stored in `event_collection.log_levels`.

## Dependencies

- Requires Rayon for parallel processing
- Uses Regex for log parsing

## Contributing

Contributions welcome! Please submit pull requests or open issues on the project repository.
