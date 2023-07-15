use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    
    let mut forest: Vec<Vec<(u8,bool)>> = Vec::new();

    let mut end = false;
    while !end {
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("EOF");
                end = true
            },
            Ok(_) => {
                // Parse the line
                forest.push(
                    Vec::from(
                        line.trim()
                            .chars()
                            .map(|c| (c.to_string().parse::<u8>().unwrap(),false))
                            .collect::<Vec<(u8,bool)>>()));
            },
            Err(e) => panic!("Failed to read a line. {}", e),
        };
        line.clear();

    }

    let height = forest.len();
    let width = forest[0].len();
    let mut max: i32 = -1;
    
    for row in forest.iter_mut() {
        // From left
        for tree in row.iter_mut() {
            if tree.0 as i32 > max {
                tree.1 = true;
                max = tree.0 as i32;
                if max == 9 { break; }
            }
        }
        max = -1;

        // From right
        for tree in row.iter_mut().rev() {
            if tree.0 as i32 > max {
                tree.1 = true;
                max = tree.0 as i32;
                if max == 9 { break; }
            }
        }
        max = -1;
    }

    for col in 0..width {
        // From top
        for row in 0..height {
            let tree: &mut (u8, bool) = &mut forest[row][col];
            if tree.0 as i32 > max {
                tree.1 = true;
                max = tree.0 as i32;
                if max == 9 { break; }
            }
        }
        max = -1;
        // From bottom
        for row in (0..height).rev() {
            let tree: &mut (u8, bool) = &mut forest[row][col];
            if tree.0 as i32 > max {
                tree.1 = true;
                max = tree.0 as i32;
                if max == 9 { break; }
            }
        }
        max = -1;
    }
    let count = forest.iter().flatten().fold(0, |acc, tree|
         if tree.1 { acc + 1 } else { acc });

    // for row in forest.iter() {
    //     for tree in row.iter() {
    //         print!("{}", tree.1 as u8);
    //     }
    //     print!("\n");
    // }
    let mut max_ss = 0;
    let mut trees: Vec<u32> = Vec::new();
    for col in 1..width-1 {
        for row in 1..height-1 {
            let h: u8 = forest[row][col].0;
            let mut scores = [0;4];
            // top
            if row > 0 {
                for top in (0..row).rev() {
                    scores[0] += 1;
                    if h <= forest[top][col].0 {
                        break;
                    }
                }
            }
            // bot
            if row < height {
                for bot in row+1..height {
                    scores[1] += 1;
                    if h <= forest[bot][col].0 { break; }
                }
            }
            // left
            if col > 0 {
                for left in (0..col).rev() {
                    scores[2] += 1;
                    if h <= forest[row][left].0 { break; }
                }
            }
            // right
            if col < width {
                for right in col+1..width {
                    scores[3] += 1;
                    if h <= forest[row][right].0 { break; }
                }
            }
            // println!("{:?}", scores);
            
            let ss = scores[0] * scores[1] * scores[2] * scores[3];
            if ss > max_ss { 
                max_ss = ss;
                // println!("{:?}, (row,col): ({}, {})", scores, row, col);
            }
            trees.push(ss);
        }
    }

    // let mut scenic_score = 0;
    // for ss in trees {
    //     if ss > scenic_score { scenic_score = ss; }
    // }
    println!("Result {}, ss {}", count, max_ss);
    Ok(())
}
