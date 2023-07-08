use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    
    let mut max_cal = [0;3];
    let mut cal = 0;
    let mut end = false;

    while !end {
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("EOF");
                end = true
            },
            Ok(_) => {
                let res = line.trim().parse::<i32>(); 
                match res 
                {
                    Err(_) => {
                            if cal > max_cal[0] {
                                max_cal[2] = max_cal[1];
                                max_cal[1] = max_cal[0];
                                max_cal[0] = cal;
                            } else if cal > max_cal[1] {
                                max_cal[2] = max_cal[1];
                                max_cal[1] = cal;
                            } else if cal > max_cal[2] {
                                max_cal[2] = cal;
                            }
                        
                        // println!("Elf {} has {} calories", idx, cal);
                        cal = 0;
                    }
                    Ok(c) => { cal += c; },
                };
                // {
                //     Ok(i) => println!("parse: {}", i),
                //     Err(e) => println!("{}", e),
                // }
                
            },
            Err(e) => panic!("Failed to read a line. {}", e),
        };
        line.clear();

    }
    let sum = max_cal[2] + max_cal[1] + max_cal[0];
    println!("Top 3 calorie sum is {}", sum);
    Ok(())
}
