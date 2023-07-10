use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn get_score(x: char) -> i32 {
    if x.is_ascii_lowercase() {
        (x as i32) - ('a' as i32) + 1
    } else {
        (x as i32) - ('A' as i32) + 27
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    
    let mut score = 0;
    let mut score2 = 0;

    let mut end = false;
    while !end {
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("EOF");
                end = true
            },
            Ok(_) => {
                // Parse the line
                let mut group: [String;3] = [
                    String::new(),String::new(),String::new()];
                group[0] = line.clone();
                reader.read_line(&mut group[1])?;
                reader.read_line(&mut group[2])?;
                for c in group[0].chars() {
                    match group[1].find(c) {
                        Some(_) => {
                            match group[2].find(c) {
                                Some(_) => {
                                    score2 += get_score(c);
                                    println!("{}",c);
                                    break;
                                },
                                None => continue,
                            }
                        },
                        None => continue,
                    }
                    
                }

                let second = line.split_off(line.len()/2);        
                for c in line.chars() {
                    if second.find(c) != None { 
                        score += get_score(c);
                        break;
                    }
                }
            },
            Err(e) => panic!("Failed to read a line. {}", e),
        };
        line.clear();

    }
    println!("Result {}, 2nd: {}", score, score2);
    Ok(())
}
