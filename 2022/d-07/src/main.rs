use core::panic;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


struct Dir {
    name: String,
    id: usize,
    dirs: Vec<usize>,
    size: u64,
}

fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    
    let mut root = Dir {
        name: String::from("/"), 
        id: 0,
        dirs: Vec::new(), 
        size: 0
    };
    let mut dirs: Vec<Dir> = Vec::new();
    let mut path: Vec<usize> = Vec::new();
    let mut sum = 0;
    let mut id: usize = 1;

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
                let spl: Vec<&str> = line.split(' ').collect();
                match spl[0] {
                    // User command
                    "$" => {
                        match spl[1] {
                            "cd" => {
                                // Go back to parent directory
                                if spl[2] == ".." {
                                    path.pop();
                                
                                // Jump to root
                                } else if spl[2] == "/" {
                                    path.clear();
                                    path.push(0);
                                } else {
                                    match path.last() {
                                        // Path is empty
                                        None => {
                                            if spl[2] == "/" {
                                                path.push(0);
                                            } else {
                                                panic!("non root cd on empty path.");
                                            }
                                        },
                                        // cd dirname
                                        // Some(current dir)
                                        Some(&idx) => {
                                            let mut found = false;

                                            // Subdirectory exists
                                            for &sidx in dirs[idx].dirs.iter() {
                                                if dirs[sidx].name == spl[2] {
                                                    path.push(sidx);
                                                    found = true;
                                                    break;
                                                }
                                            }
                                            // Create and move to subdir
                                            if !found {
                                                dirs.push( Dir {
                                                    name: String::from(spl[2]),
                                                    id: id,
                                                    dirs: Vec::new(),
                                                    size: 0
                                                });
                                                id += 1;
                                                path.push(id);
                                            }
                                        }
                                    }
                                }
                            },
                            // List files and directories
                            "ls" => {},
                            e => panic!("undefined user command {}", e),
                        }
                    },
                    "dir" => {
                        let cwd = *path.last().unwrap();
                        let dr = &dirs[cwd].dirs;
                        for &idx in dr.iter() {
                            if dirs[idx].name == spl[1] {
                                unimplemented!()        
                            }
                        }
                    },
                    _ => {}
                }
                
            },
            Err(e) => panic!("Failed to read a line. {}", e),
        };
        line.clear();

    }
    println!("Result {}", result);
    Ok(())
}
