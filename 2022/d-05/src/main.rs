use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn get_container(idx: usize, s: &String) -> Option<char> {
    let sidx = idx * 4 + 1;
    match s.as_bytes()[sidx] as char {
        ' ' => None,
        s => Some(s),
    }
}
fn get_instruction(s: &String) -> [usize;3] {
    let split: Vec<&str> = s.split_ascii_whitespace().collect();
    [
        split[1].parse().unwrap(),
        split[3].parse::<usize>().unwrap()-1,
        split[5].parse::<usize>().unwrap()-1
    ]
}

fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    
    let mut stacks = Vec::<Vec::<char>>::new();
    for _ in 0..9  {
        stacks.push(Vec::new());
    }


    let mut end = false;
    let mut parse_stacks = true;

    while !end {
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("EOF");
                end = true;
            },
            Ok(_) => {
                // Parse the line
                if parse_stacks {
                    if line.as_bytes()[1] as char == '1' {
                        parse_stacks = false;
                        let _ = reader.read_line(&mut line);
                        line.clear();
                                
                        continue;
                    } else {
                        // Parse a line of containers
                        for i in 0..9 {
                            match get_container(i, &line) {
                                Some(c) => {
                                    stacks[i].insert(0, c);
                                },
                                None => {}
                            }
                        }
                    }
                // Parse instructions
                } else {
                    let ins = get_instruction(&line);
                    // println!("{}, {}, {}", ins[0], ins[1], ins[2]);
                    let source = &mut stacks[ins[1]];
                    let slice = source.split_off(source.len()-ins[0]);
                    for c in slice {
                        // let content = stacks[ins[1]].pop().unwrap();
                        stacks[ins[2]].push(c);
                    }
                }
            },
            Err(e) => panic!("Failed to read a line. {}", e),
        };
        line.clear();

    }
    let mut s = String::new();
    for v in stacks.iter() {
        s.push(*v.last().unwrap());
    }
    println!("Result {}", s);
    Ok(())
}
