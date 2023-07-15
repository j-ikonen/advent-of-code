use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn is_touching(head: (i32, i32), tail: (i32, i32)) -> bool {
    let dif_row = head.0 - tail.0;
    let dif_col = head.1 - tail.1;
    dif_col.abs() <= 1 && dif_row.abs() <= 1
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let row;
    let col;
    let dif_row = head.0 - tail.0;
    let dif_col = head.1 - tail.1;
    
    if dif_row.is_negative() {
        row = tail.0-1;
    } else if dif_row.is_positive() {
        row = tail.0+1;
    } else {
        row = tail.0;
    }

    if dif_col.is_negative() {
        col = tail.1-1;
    } else if dif_col.is_positive() {
        col = tail.1+1;
    } else {
        col = tail.1;
    }
    (row, col)
}

fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    // let offset = (5, 20);
    // let size = (24, 80);

    let mut head: (i32, i32) = (0,0);
    let mut tail: (i32, i32) = (0,0);
    let mut visited: Vec<(i32, i32)> = Vec::new();
    visited.push((0,0));

    let mut end = false;
    while !end {
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("EOF");
                end = true
            },
            Ok(_) => {
                // Parse the line
                let ins = line.as_bytes();
                let dir = ins[0] as char;
                let n = (ins[2] as char).to_string()
                                   .parse::<i32>()
                                   .unwrap();
                // println!("{} {}", dir, n);
                for _ in 0..n {
                    match dir {
                        'U' => {
                            head.0 -= 1;
                        },
                        'D' => {
                            head.0 += 1;
                        },
                        'R' => {
                            head.1 += 1;
                        },
                        'L' => {
                            head.1 -= 1;
                        },
                        _ => continue,
                    }
                    if !is_touching(head, tail) {
                        // print!("{:?}, {:?} -> ", head, tail);
                        tail = move_tail(head, tail);
                        // println!("{:?}, {:?}", head, tail);
                        visited.push(tail);
                    }
                    // let phead = (head.0+offset.0, head.1+offset.1);
                    // let ptail = (tail.0+offset.0, tail.1+offset.1);
                    // for row in 0..size.0 {
                    //     for col in 0..size.1 {
                    //         if (row, col) == phead {
                    //             print!("H");
                    //         } else if (row, col) == ptail {
                    //             print!("T");
                    //         } else {
                    //             print!(".")
                    //         }
                    //     }
                    //     print!("\n");
                    // }
                    // let mut buf = String::from("");
                    // let _ = std::io::stdin().read_line(&mut buf);
                }

            },
            Err(e) => panic!("Failed to read a line. {}", e),
        };
        line.clear();

    }
    // let mut min0 = 0;
    // let mut min1 = 0;
    println!("with duplicates {}", visited.len());

    visited.sort();
    visited.dedup();
    // for cell in &visited {
    //     println!("{:?}", cell);
    //     if cell.0 < min0 { min0 = cell.0 }
    //     if cell.1 < min1 { min1 = cell.1 }
    // }
    println!("Result {}", visited.len());
    Ok(())
}
