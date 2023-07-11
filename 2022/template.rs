use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    
    let result = 0;

    let mut end = false;
    while !end {
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("EOF");
                end = true
            },
            Ok(_) => {
                // Parse the line            
            },
            Err(e) => panic!("Failed to read a line. {}", e),
        };
        line.clear();

    }
    println!("Result {}", result);
    Ok(())
}
