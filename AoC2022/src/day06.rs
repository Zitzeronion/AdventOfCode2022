use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    // File for reading
    let f = File::open("../buffer.txt")?;
    // let f = File::open("../buffer.txt")?;
    let reader = BufReader::new(f);
    
    // Switch between first and second star
    let star: i32 = 2; 
    // Loop through the lines of the txt file
    let mut count = 0;
    for line in reader.lines() {
        // read the line into a string
        let string = line?;
        // half the string, there are two compartments
        let maxiter = string.len() - 4;
        let maxiter2 = string.len() - 14;
        if star == 1 {
            for i in 0..maxiter {
                let complen = &string[i..i+4];
                let first = complen.chars().nth(0).unwrap();
                let sec = complen.chars().nth(1).unwrap();
                let third = complen.chars().nth(2).unwrap();
                let forth = complen.chars().nth(3).unwrap();
                if !(first == sec) {
                    if !(first == third) {
                        if !(first == forth) {
                            if !(sec == third) {
                                if !(sec == forth) {
                                    if !(third == forth) {
                                        println!("Iteration {} sequence {}", i, complen);
                                        count = i+4;
                                        break;
                                    }   
                                }  
                            }  
                        } 
                    }  
                }; 
            }
        } else if star == 2 {
            for i in 0..maxiter2 {
                let complen = &string[i..i+14];
                let c1 = complen.chars().nth(0).unwrap();
                let c2 = complen.chars().nth(1).unwrap();
                let c3 = complen.chars().nth(2).unwrap();
                let c4 = complen.chars().nth(3).unwrap();
                let c5 = complen.chars().nth(4).unwrap();
                let c6 = complen.chars().nth(5).unwrap();
                let c7 = complen.chars().nth(6).unwrap();
                let c8 = complen.chars().nth(7).unwrap();
                let c9 = complen.chars().nth(8).unwrap();
                let c10 = complen.chars().nth(9).unwrap();
                let c11 = complen.chars().nth(10).unwrap();
                let c12 = complen.chars().nth(11).unwrap();
                let c13 = complen.chars().nth(12).unwrap();
                let c14 = complen.chars().nth(13).unwrap();
                let mut signal = vec![&c1, &c2, &c3, &c4, &c5, &c6, &c7, &c8, &c9, &c10, &c11, &c12, &c13, &c14];
                signal.retain(|value| *value != &c1);
                let r1 = signal.len();
                if r1 == 13 {
                    signal.retain(|value| *value != &c2);
                    let r2 = signal.len();
                    if r2 == 12 {
                        signal.retain(|value| *value != &c3);
                        let r3 = signal.len();
                        if r3 == 11 {
                            signal.retain(|value| *value != &c4);
                            let r4 = signal.len();
                            if r4 == 10 {
                                signal.retain(|value| *value != &c5);
                                let r5 = signal.len();
                                if r5 == 9 {
                                    signal.retain(|value| *value != &c6);
                                    let r6 = signal.len();
                                    if r6 == 8 {
                                        signal.retain(|value| *value != &c7);
                                        let r7 = signal.len();
                                        if r7 == 7 {
                                            signal.retain(|value| *value != &c8);
                                            let r8 = signal.len();
                                            if r8 == 6 {
                                                signal.retain(|value| *value != &c9);
                                                let r9 = signal.len();
                                                if r9 == 5 {
                                                    signal.retain(|value| *value != &c10);
                                                    let r10 = signal.len();
                                                    if r10 == 4 {
                                                        println!("Easy {} \n{}",i , complen);
                                                    }
                                                }
                                            };
                                        };
                                    };
                                };
                            };
                        };
                    };    
                };
            }
        };        
    }
    println!("This is the char pos {}", count);
    Ok(())
}
