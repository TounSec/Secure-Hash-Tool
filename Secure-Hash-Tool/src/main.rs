use crypto_hash::{Algorithm, hex_digest};
use std::io::{self, Write};

fn readInput() -> String
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input.");
    input.trim().to_string()
}

fn main()
{
    println!("Enter the text to hash : ");
    let input = readInput();

    // Choose the hash algorithm (e.g SHA256)
    let algorithm = Algorithm::SHA256;

    // Calculate the next hash
    let hash = hex_digest(algorithm, input.as_bytes());

    // Display the result
    println!("Hash : {}", hash);
}