use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    // File for reading
    let f = File::open("../RockPaperScissor.txt")?;
    let reader = BufReader::new(f);
    // Three vectors, my choice, opponent choice and result
    let mut mychoice: Vec<i32> = Vec::new();
    let mut opchoice: Vec<i32> = Vec::new();
    let mut result: Vec<i32> = Vec::new();
    // One more to add choices and result
    let mut score: Vec<i32> = Vec::new();
    // I don't know much about Rust, so keep it simple
    let star: i32 = 2; 
    // Loop through the lines of the txt file
    for line in reader.lines() {
        // read the line into a string
        let string = line?;
        // From this string extract first and last char
        let op = string.chars().take(1).last().unwrap();
        let my = string.chars().take(3).last().unwrap();
        // Some spagethi logic for opponent
        if op == 'A' {
            opchoice.push(1);
        } else if op == 'B' {
            opchoice.push(2);
        } else if op == 'C' {
            opchoice.push(3);
        };
        // The first star solved here
        if star == 1 {
            // and for my choices
            if my == 'X' {
                mychoice.push(1);
            } else if my == 'Y' {
                mychoice.push(2);
            } else if my == 'Z' {
                mychoice.push(3);
            };
            // and for the points scored
            if my == 'X' && op == 'A' {
                result.push(3);
            } else if my == 'Y' && op == 'B' {
                result.push(3);
            } else if my == 'Z' && op == 'C' {
                result.push(3);
            } else if my == 'X' && op == 'B' {
                result.push(0);
            } else if my == 'X' && op == 'C' {
                result.push(6);
            } else if my == 'Y' && op == 'A' {
                result.push(6);
            } else if my == 'Y' && op == 'C' {
                result.push(0);
            } else if my == 'Z' && op == 'A' {
                result.push(0);
            } else if my == 'Z' && op == 'B' {
                result.push(6);  
            };
        // Second star here
        } else if star == 2 {
            // Now I know who has to win
            if my == 'X' && op == 'A' {
                mychoice.push(3);
                result.push(0);
            } else if my == 'X' && op == 'B' {
                mychoice.push(1);
                result.push(0);
            } else if my == 'X' && op == 'C' {
                mychoice.push(2);
                result.push(0);
            } else if my == 'Y' && op == 'A' {
                mychoice.push(1);
                result.push(3);
            } else if my == 'Y' && op == 'B' {
                mychoice.push(2);
                result.push(3);
            } else if my == 'Y' && op == 'C' {
                mychoice.push(3);
                result.push(3);
            } else if my == 'Z' && op == 'A' {
                mychoice.push(2);
                result.push(6);
            } else if my == 'Z' && op == 'B' {
                mychoice.push(3);
                result.push(6);
            } else if my == 'Z' && op == 'C' {
                mychoice.push(1);
                result.push(6);
            };
        };
    }
    // Loop through and add up
    let l = result.len();
    for i in 0..l {
        let j = mychoice[i];
        let k = result[i];
        score.push(j+k);
    }
    // Print for my sanity :)
    println!("My opponents choice:  {:?}", opchoice);
    println!("My choice:            {:?}", mychoice);
    println!("Win tie lose:         {:?}", result);
    println!("Scores:               {:?}", score);
    let points: i32 = score.iter().sum();
    println!("Points made: {}", points);
    Ok(())
}
