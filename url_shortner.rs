use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

// Function to generate a short URL key
fn generate_short_url() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(6).map(char::from).collect()
}

// Function to read mappings from a file into a HashMap
fn read_mappings(filename: &str) -> io::Result<HashMap<String, String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut mappings = HashMap::new();
    
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            mappings.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    
    Ok(mappings)
}

// Function to write a mapping to a file
fn write_mapping(filename: &str, short_url: &str, long_url: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(filename)?;
    writeln!(file, "{},{}", short_url, long_url)?;
    Ok(())
}

// Main function
fn main() {
    // Check for correct number of command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <long_url>", args[0]);
        return;
    }
    
    let long_url = &args[1];
    let filename = "url_mappings.txt";
    
    // Read existing mappings from file
    let mut mappings = match read_mappings(filename) {
        Ok(mappings) => mappings,
        Err(_) => {
            eprintln!("Could not read mappings from file.");
            return;
        }
    };

    // Generate a short URL
    let short_url = generate_short_url();
    
    // Store the mapping
    mappings.insert(short_url.clone(), long_url.clone());
    if write_mapping(filename, &short_url, long_url).is_err() {
        eprintln!("Could not write mapping to file.");
        return;
    }
    
    println!("Short URL for {} is: {}", long_url, short_url);
    
    // To decode, read the mapping file and find the corresponding long URL
    if let Ok(mappings) = read_mappings(filename) {
        if let Some(original_url) = mappings.get(&short_url) {
            println!("The original URL for {} is: {}", short_url, original_url);
        } else {
            println!("No mapping found for the short URL: {}", short_url);
        }
    } else {
        eprintln!("Could not read mappings from file.");
    }
}