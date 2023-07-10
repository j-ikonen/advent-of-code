use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    
    let mut count = 0;
    let mut count2 = 0;

    let mut end = false;

    while !end {
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("EOF");
                end = true
            },
            Ok(_) => {
                // Parse the line
                let mut areas: [i32;4] = [0;4];
                for (i, s) in line.trim().split(|c| c == '-' || c == ',').enumerate() {
                    // println!("{}", s);
                    areas[i] = s.parse().unwrap();
                }

                if (areas[0] >= areas[2] && areas[1] <= areas[3]) ||
                    (areas[0] <= areas[2] && areas[1] >= areas[3]) {
                        count += 1;
                }
                if (areas[2] <= areas[0] && areas[0] <= areas[3]) ||
                    (areas[2] <= areas[1] && areas[1] <= areas[3]) ||
                    (areas[0] <= areas[2] && areas[2] <= areas[1]) ||
                    (areas[0] <= areas[3] && areas[3] <= areas[1])
                {
                        count2 += 1;
                }
                         
            },
            Err(e) => panic!("Failed to read a line. {}", e),
        };
        line.clear();

    }
    println!("Result {}, 2nd {}", count, count2);
    Ok(())
}
