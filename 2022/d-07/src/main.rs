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
    
    let mut dirs: Vec<Dir> = Vec::new();
    let mut path: Vec<usize> = Vec::new();
    let mut sum = 0;
    let mut id: usize = 1;
    
    dirs.push(Dir {
        name: String::from("/"), 
        id: 0,
        dirs: Vec::new(), 
        size: 0
    });


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
                
                // Change directory
                let cwd = *path.last().unwrap_or(&0);
                if spl[0] == "$" && spl[1] == "cd" {
                    
                    // println!("dir: {}, size: {}", dirs[cwd].name, dirs[cwd].size);
                    // Go to root
                    if spl[2] == "/\n" {
                        path.clear();
                        path.push(0);
                        println!("cd / asd");
                    // Go up on step
                    } else if spl[2] == "..\n" {
                        path.pop().unwrap();
                        // println!("cd .. asd");

                    // Go to directory within cwd
                    } else {
                        let mut found = false;
                        // println!("cd {}", spl[2]);
                        for &d in dirs[cwd].dirs.iter() {
                            if dirs[d].name == spl[2] { 
                                found = true; 
                                path.push(d);
                                break; 
                            }
                        }
                        if !found {
                            dirs.push( Dir {
                                name: String::from(spl[2]),
                                id: id,
                                dirs: Vec::new(),
                                size: 0,
                            } );
                            dirs[cwd].dirs.push(id);
                            path.push(id);
                            id += 1;
                        }

                    }
                    // println!("path: {:?}", path);
                } else if spl[0] == "$" && spl[1] == "ls\n" {
                    dirs[cwd].size = 0;
                    // println!("ls");
                // Register a new directory in cwd
                } else if spl[0] == "dir" {
                    
                // Add file size to dir size
                } else {
                    // println!("parse: {:?}", spl);
                    dirs[cwd].size += spl[0].parse::<u64>().unwrap();
                    // println!("file: {}", spl[0]);
                }
            },
            Err(e) => panic!("Failed to read a line. {}", e),
        };
        line.clear();

    }

    // Collect directory sizes
    let mut v: Vec<(usize, u64)> = Vec::new();
    dir_sizes(&mut v, &dirs, 0);
    // println!("DIR SIZES");
    // for a in v.iter() {
    //     println!("id:{}, size: {}", a.0, a.1);
    // }
    // The minimum amount of space that needs to be freed
    let min_to_free = v.last().unwrap().1 + 30000000 - 70000000;
    println!("min to free: {}", min_to_free);

    let mut min_dir: (usize, u64) = (0, v.last().unwrap().1);

    // Sum the small enough directories
    for (id, sz) in v.iter() {
        if sz < &100000 { sum += sz; }
        if *sz > min_to_free && *sz < min_dir.1 {
            min_dir = (*id, *sz);
        } 
    }
    println!("dir size to delete: {}", min_dir.1);



    println!("Result {}", sum);
    Ok(())
}

fn dir_sizes(v: &mut Vec<(usize, u64)>, dirs: &Vec<Dir>, cwd: usize) -> u64 {
    let mut s = dirs[cwd].size;
    for &d in &dirs[cwd].dirs {
        s += dir_sizes(v, dirs, d);
    }
    v.push((cwd, s));
    s
}
