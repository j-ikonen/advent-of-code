use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    
    let mut score: i64 = 0;
    let mut score2: i64 = 0;

    let mut end = false;

    while !end {
        match reader.read_line(&mut line) {
            Ok(0) => {
                // println!("EOF");
                end = true
            },
            Ok(_) => {
                // Parse the line
                let mut chars = line.chars();
                
                let op_score = (chars.next().unwrap() as i64)
                                    - ('A' as i64) + 1;
                chars.next();
                let sel_score = (chars.next().unwrap() as i64)
                                     - ('X' as i64) + 1;
                let match_score;
                let dif = sel_score - op_score;
                if dif == 1 || dif == -2 {
                    match_score = 6;
                } else if dif == 0 {
                    match_score = 3;
                } else {
                    match_score = 0;
                }
                score += match_score + sel_score;
                let win_score = (sel_score - 1) * 3;
                let mut chs_score = (op_score + sel_score - 2) % 3;
                if chs_score == 0 { chs_score = 3; }
                // println!("{} {}, win:{}, chs:{}",
                //     (op_score as u8 - 1 + ('A' as u8)) as char,
                //     (sel_score as u8 - 1 + ('X' as u8)) as char,
                //     win_score,
                //     chs_score         
                // );
                score2 += win_score + chs_score;
                // match sel_score {
                //     // Lose
                //     1 => { score2 += (op_score - 1) % 3; },
                //     // Draw
                //     2 => { score2 += op_score + 3; },
                //     // Win
                //     3 => { score2 += (op_score + 1) % 3 + 6; },
                //     _ => panic!("not expected second letter."),
                // }
            },
            Err(e) => panic!("Failed to read a line. {}", e),
        };
        line.clear();

    }
    println!("Score {}, score2: {}", score, score2);
    Ok(())
}
