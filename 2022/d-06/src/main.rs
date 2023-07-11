use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    
    let mut idx: usize = 0;
    let size = 4;

    let mut end = false;
    while !end {
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("EOF");
                end = true
            },
            Ok(_) => {
                // Parse the line
                let bytes = line.as_bytes();
                let mut start: usize = 0;
                for (i, c) in bytes.iter().enumerate() {
                    for dif in 1..size {
                        let cmp_idx: i32 = (i as i32)-(dif as i32);
                        if cmp_idx < 0 || start as i32 > cmp_idx { break; }
                                                
                        if *c == bytes[cmp_idx as usize] {
                            start = 1 + cmp_idx as usize;
                            // println!("{}, {}", start, *c as char);
                            break;
                        }
                    }
                    // println!("i,c: {},{}",i,*c as char);
                    if i + 1 - start == size { idx = i + 1; break; }
                }
            },
            Err(e) => panic!("Failed to read a line. {}", e),
        };
        line.clear();

    }
    println!("Result {}", idx);
    Ok(())
}
