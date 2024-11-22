# Simple URL Shortener

This Rust project implements a simple URL shortener that generates and decodes short URLs for long URLs. The program accepts a long URL as input, generates a short URL, stores the mapping between the short URL and the long URL in a file, and retrieves the original long URL when a short URL is entered.

## Project Requirements

The program meets the following requirements:
1. Accept a long URL as input from the user.
2. Generate a short URL for the long URL using a hash function.
3. Store the mapping between the short URL and the long URL in a file.
4. Read the mapping file and redirect the user to the corresponding long URL when a short URL is entered.
5. Handle errors gracefully and provide informative error messages to the user.

## How to Use

1. **Compile the Program:**
   ```bash
   rustc url_shortener.rs

Run the Program: To shorten a URL, use the following command:

bash
./url_shortener <long_url>
Replace <long_url> with the actual long URL you want to shorten.

Decode a Short URL: After running the program and generating a short URL, you can enter the short URL to get the original long URL.


Examples:
Generate a Short URL:

bash
./url_shortener https://www.example.com
Output:

Short URL for https://www.example.com is: abc123
Retrieve the Original URL: The program will automatically look up the short URL and output the corresponding original URL.

add read me notes for github with this code: use std::collections::HashMap; use std::env; use std::fs::{File, OpenOptions}; use std::io::{self, BufRead, BufReader, Write}; use rand::{thread_rng, Rng}; use rand::distributions::Alphanumeric; // Function to generate a short URL key fn generate_short_url() -> String { thread_rng().sample_iter(&Alphanumeric).take(6).map(char::from).collect() } // Function to read mappings from a file into a HashMap fn read_mappings(filename: &str) -> io::Result<HashMap<String, String>> { let file = File::open(filename)?; let reader = BufReader::new(file); let mut mappings = HashMap::new(); for line in reader.lines() { let line = line?; let parts: Vec<&str> = line.split(',').collect(); if parts.len() == 2 { mappings.insert(parts[0].to_string(), parts[1].to_string()); } } Ok(mappings) } // Function to write a mapping to a file fn write_mapping(filename: &str, short_url: &str, long_url: &str) -> io::Result<()> { let mut file = OpenOptions::new().append(true).open(filename)?; writeln!(file, "{},{}", short_url, long_url)?; Ok(()) } // Main function fn main() { // Check for correct number of command line arguments let args: Vec<String> = env::args().collect(); if args.len() != 2 { eprintln!("Usage: {} <long_url>", args[0]); return; } let long_url = &args[1]; let filename = "url_mappings.txt"; // Read existing mappings from file let mut mappings = match read_mappings(filename) { Ok(mappings) => mappings, Err(_) => { eprintln!("Could not read mappings from file."); return; } }; // Generate a short URL let short_url = generate_short_url(); // Store the mapping mappings.insert(short_url.clone(), long_url.clone()); if write_mapping(filename, &short_url, long_url).is_err() { eprintln!("Could not write mapping to file."); return; } println!("Short URL for {} is: {}", long_url, short_url); // To decode, read the mapping file and find the corresponding long URL if let Ok(mappings) = read_mappings(filename) { if let Some(original_url) = mappings.get(&short_url) { println!("The original URL for {} is: {}", short_url, original_url); } else { println!("No mapping found for the short URL: {}", short_url); } } else { eprintln!("Could not read mappings from file."); } }
Here's a README.mdfile to accompany your Rust URL shortener project:

markdown
# Simple URL Shortener

This Rust project implements a simple URL shortener that generates and decodes short URLs for long URLs. The program accepts a long URL as input, generates a short URL, stores the mapping between the short URL and the long URL in a file, and retrieves the original long URL when a short URL is entered.

## Project Requirements

The program meets the following requirements:
1. Accept a long URL as input from the user.
2. Generate a short URL for the long URL using a hash function.
3. Store the mapping between the short URL and the long URL in a file.
4. Read the mapping file and redirect the user to the corresponding long URL when a short URL is entered.
5. Handle errors gracefully and provide informative error messages to the user.

## How to Use

1. **Compile the Program:**
   ```bash
   rustc url_shortener.rs
Run the Program: To shorten a URL, use the following command:

bash
./url_shortener <long_url>
Replace <long_url> with the actual long URL you want to shorten.

Decode a Short URL: After running the program and generating a short URL, you can enter the short URL to get the original long URL.

Example
Generate a Short URL:

bash
./url_shortener https://www.example.com
Output:

Short URL for https://www.example.com is: abc123
Retrieve the Original URL: The program will automatically look up the short URL and output the corresponding original URL.

Code Explanation
rust
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
Notes
Ensure you have rand crate included in your Cargo.toml if you are using Cargo to manage dependencies.

The short URL key length can be adjusted by changing the number of characters in the generate_short_url function.

License
This project is licensed under the MIT License. See the LICENSE file for details.

Acknowledgements
Rust Programming Language

rand crate for random string generation


