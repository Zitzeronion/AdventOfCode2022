use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::convert::TryInto;

fn main() -> io::Result<()> {
    // File for reading
    // let f = File::open("../calories_small.txt").unwrap();
    let f = File::open("../crates.txt").unwrap();
    let reader = BufReader::new(f);
    // Switch between first and second star
    let star: i32 = 1; 
    // Loop through the lines of the txt file
    let mut c1: Vec<char> = Vec::new();
    let mut c2: Vec<char> = Vec::new();
    let mut c3: Vec<char> = Vec::new();
    let mut c4: Vec<char> = Vec::new();
    let mut c5: Vec<char> = Vec::new();
    let mut c6: Vec<char> = Vec::new();
    let mut c7: Vec<char> = Vec::new();
    let mut c8: Vec<char> = Vec::new();
    let mut c9: Vec<char> = Vec::new();
    let mut number: Vec<u32> = Vec::new();
    let mut from: Vec<u32> = Vec::new();
    let mut to: Vec<u32> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        
        if index + 1 < 9 {
            c1.push(line.chars().take(2).last().unwrap());
            c2.push(line.chars().take(6).last().unwrap());
            c3.push(line.chars().take(10).last().unwrap());
            c4.push(line.chars().take(14).last().unwrap());
            c5.push(line.chars().take(18).last().unwrap());
            c6.push(line.chars().take(22).last().unwrap());
            c7.push(line.chars().take(26).last().unwrap());
            c8.push(line.chars().take(30).last().unwrap());
            c9.push(line.chars().take(34).last().unwrap());
            // println!("{}. {}", index + 1, c8[index]);
        } else if index + 1 > 10{
            let t1: Vec<&str> = line.split(&[' '][..]).collect();
            let nu = t1[1].parse().unwrap();
            let na = t1[3].parse().unwrap();
            let ne = t1[5].parse().unwrap();
            number.push(nu);
            from.push(na);
            to.push(ne);   
        };
        // Show the line and its number.
        
    }
    c1.retain(|x| *x != ' ');
    c2.retain(|x| *x != ' ');
    c3.retain(|x| *x != ' ');
    c4.retain(|x| *x != ' ');
    c5.retain(|x| *x != ' ');
    c6.retain(|x| *x != ' ');
    c7.retain(|x| *x != ' ');
    c8.retain(|x| *x != ' ');
    c9.retain(|x| *x != ' ');
    // Start to reorder the vectors
    println!("Letters: {} {} {} {} {} {} {} {} {}",c1[0], c2[0], c3[0], c4[0], c5[0], c6[0], c7[0], c8[0], c9[0]);

    for i in 0..number.len() {
        let take: u32 = from[i];
        let many: u32 = number[i];
        let give: u32 = to[i];
        println!("Iteration {}\nTake {} crates and move from {} to {}",i , many, take, give);
        if take == 1 {
            // println!("{:?}", c1);
            if give == 2 {
                for j in 1..=many {
                    c2.push(c1.remove((0).try_into().unwrap()));
                    c2.rotate_right(1);
                }
            } else if give == 3 {
                for j in 1..=many {
                    c3.push(c1.remove((0).try_into().unwrap()));
                    c3.rotate_right(1);
                }
            } else if give == 4 {
                for j in 1..=many {
                    c4.push(c1.remove((0).try_into().unwrap()));
                    c4.rotate_right(1);
                }
            } else if give == 5 {
                for j in 1..=many {
                    c5.push(c1.remove((0).try_into().unwrap()));
                    c5.rotate_right(1);
                }
            } else if give == 6 {
                for j in 1..=many {
                    c6.push(c1.remove((0).try_into().unwrap()));
                    c6.rotate_right(1);
                }
            } else if give == 7 {
                for j in 1..=many {
                    c7.push(c1.remove((0).try_into().unwrap()));
                    c7.rotate_right(1);
                }
            } else if give == 8 {
                for j in 1..=many {
                    c8.push(c1.remove((0).try_into().unwrap()));
                    c8.rotate_right(1);
                }
            } else if give == 9 {
                for j in 1..=many {
                    c9.push(c1.remove((0).try_into().unwrap()));
                    c9.rotate_right(1);
                }
            };   
        } else if take == 2 {
            if give == 1 {
                for j in 1..=many {
                    c1.push(c2.remove((0).try_into().unwrap()));
                    c1.rotate_right(1);
                }
            } else if give == 3 {
                for j in 1..=many {
                    c3.push(c2.remove((0).try_into().unwrap()));
                    c3.rotate_right(1);
                }
            } else if give == 4 {
                for j in 1..=many {
                    c4.push(c2.remove((0).try_into().unwrap()));
                    c4.rotate_right(1);
                }
            } else if give == 5 {
                for j in 1..=many {
                    c5.push(c2.remove((0).try_into().unwrap()));
                    c5.rotate_right(1);
                }
            } else if give == 6 {
                for j in 1..=many {
                    c6.push(c2.remove((0).try_into().unwrap()));
                    c6.rotate_right(1);
                }
            } else if give == 7 {
                for j in 1..=many {
                    c7.push(c2.remove((0).try_into().unwrap()));
                    c7.rotate_right(1);
                }
            } else if give == 8 {
                for j in 1..=many {
                    c8.push(c2.remove((0).try_into().unwrap()));
                    c8.rotate_right(1);
                }
            } else if give == 9 {
                for j in 1..=many {
                    c9.push(c2.remove((0).try_into().unwrap()));
                    c9.rotate_right(1);
                }
            };    
        } else if take == 3 {
            if give == 1 {
                for j in 1..=many {
                    c1.push(c3.remove((0).try_into().unwrap()));
                    c1.rotate_right(1);
                }
            } else if give == 2 {
                for j in 1..=many {
                    c2.push(c3.remove((0).try_into().unwrap()));
                    c2.rotate_right(1);
                }
            } else if give == 4 {
                for j in 1..=many {
                    c4.push(c3.remove((0).try_into().unwrap()));
                    c4.rotate_right(1);
                }
            } else if give == 5 {
                for j in 1..=many {
                    c5.push(c3.remove((0).try_into().unwrap()));
                    c5.rotate_right(1);
                }
            } else if give == 6 {
                for j in 1..=many {
                    c6.push(c3.remove((0).try_into().unwrap()));
                    c6.rotate_right(1);
                }
            } else if give == 7 {
                for j in 1..=many {
                    c7.push(c3.remove((0).try_into().unwrap()));
                    c7.rotate_right(1);
                }
            } else if give == 8 {
                for j in 1..=many {
                    c8.push(c3.remove((0).try_into().unwrap()));
                    c8.rotate_right(1);
                }
            } else if give == 9 {
                for j in 1..=many {
                    c9.push(c3.remove((0).try_into().unwrap()));
                    c9.rotate_right(1);
                }
            };    
        } else if take == 4 {
            if give == 1 {
                for j in 1..=many {
                    c1.push(c4.remove((0).try_into().unwrap()));
                    c1.rotate_right(1);
                }
            } else if give == 2 {
                for j in 1..=many {
                    c2.push(c4.remove((0).try_into().unwrap()));
                    c2.rotate_right(1);
                }
            } else if give == 3 {
                for j in 1..=many {
                    c3.push(c4.remove((0).try_into().unwrap()));
                    c3.rotate_right(1);
                }
            } else if give == 5 {
                for j in 1..=many {
                    c5.push(c4.remove((0).try_into().unwrap()));
                    c5.rotate_right(1);
                }
            } else if give == 6 {
                for j in 1..=many {
                    c6.push(c4.remove((0).try_into().unwrap()));
                    c6.rotate_right(1);
                }
            } else if give == 7 {
                for j in 1..=many {
                    c7.push(c4.remove((0).try_into().unwrap()));
                    c7.rotate_right(1);
                }
            } else if give == 8 {
                for j in 1..=many {
                    c8.push(c4.remove((0).try_into().unwrap()));
                    c8.rotate_right(1);
                }
            } else if give == 9 {
                for j in 1..=many {
                    c9.push(c4.remove((0).try_into().unwrap()));
                    c9.rotate_right(1);
                }
            };    
        } else if take == 5 {
            if give == 1 {
                for j in 1..=many {
                    c1.push(c5.remove((0).try_into().unwrap()));
                    c1.rotate_right(1);
                }
            } else if give == 2 {
                for j in 1..=many {
                    c2.push(c5.remove((0).try_into().unwrap()));
                    c2.rotate_right(1);
                }
            } else if give == 3 {
                for j in 1..=many {
                    c3.push(c5.remove((0).try_into().unwrap()));
                    c3.rotate_right(1);
                }
            } else if give == 4 {
                for j in 1..=many {
                    c4.push(c5.remove((0).try_into().unwrap()));
                    c4.rotate_right(1);
                }
            } else if give == 6 {
                for j in 1..=many {
                    c6.push(c5.remove((0).try_into().unwrap()));
                    c6.rotate_right(1);
                }
            } else if give == 7 {
                for j in 1..=many {
                    c7.push(c5.remove((0).try_into().unwrap()));
                    c7.rotate_right(1);
                }
            } else if give == 8 {
                for j in 1..=many {
                    c8.push(c5.remove((0).try_into().unwrap()));
                    c8.rotate_right(1);
                }
            } else if give == 9 {
                for j in 1..=many {
                    c9.push(c5.remove((0).try_into().unwrap()));
                    c9.rotate_right(1);
                }
            };    
        } else if take == 6 {
            if give == 1 {
                for j in 1..=many {
                    c1.push(c6.remove((0).try_into().unwrap()));
                    c1.rotate_right(1);
                }
            } else if give == 2 {
                for j in 1..=many {
                    c2.push(c6.remove((0).try_into().unwrap()));
                    c2.rotate_right(1);
                }
            } else if give == 3 {
                for j in 1..=many {
                    c3.push(c6.remove((0).try_into().unwrap()));
                    c3.rotate_right(1);
                }
            } else if give == 4 {
                for j in 1..=many {
                    c4.push(c6.remove((0).try_into().unwrap()));
                    c4.rotate_right(1);
                }
            } else if give == 5 {
                for j in 1..=many {
                    c5.push(c6.remove((0).try_into().unwrap()));
                    c5.rotate_right(1);
                }
            } else if give == 7 {
                for j in 1..=many {
                    c7.push(c6.remove((0).try_into().unwrap()));
                    c7.rotate_right(1);
                }
            } else if give == 8 {
                for j in 1..=many {
                    c8.push(c6.remove((0).try_into().unwrap()));
                    c8.rotate_right(1);
                }
            } else if give == 9 {
                for j in 1..=many {
                    c9.push(c6.remove((0).try_into().unwrap()));
                    c9.rotate_right(1);
                }
            };    
        } else if take == 7 {
            if give == 1 {
                for j in 1..=many {
                    c1.push(c7.remove((0).try_into().unwrap()));
                    c1.rotate_right(1);
                }
            } else if give == 2 {
                for j in 1..=many {
                    c2.push(c7.remove((0).try_into().unwrap()));
                    c2.rotate_right(1);
                }
            } else if give == 3 {
                for j in 1..=many {
                    c3.push(c7.remove((0).try_into().unwrap()));
                    c3.rotate_right(1);
                }
            } else if give == 4 {
                for j in 1..=many {
                    c4.push(c7.remove((0).try_into().unwrap()));
                    c4.rotate_right(1);
                }
            } else if give == 5 {
                for j in 1..=many {
                    c5.push(c7.remove((0).try_into().unwrap()));
                    c5.rotate_right(1);
                }
            } else if give == 6 {
                for j in 1..=many {
                    c6.push(c7.remove((0).try_into().unwrap()));
                    c6.rotate_right(1);
                }
            } else if give == 8 {
                for j in 1..=many {
                    c8.push(c7.remove((0).try_into().unwrap()));
                    c8.rotate_right(1);
                }
            } else if give == 9 {
                for j in 1..=many {
                    c9.push(c7.remove((0).try_into().unwrap()));
                    c9.rotate_right(1);
                }
            };    
        } else if take == 8 {
            if give == 1 {
                for j in 1..=many {
                    c1.push(c8.remove((0).try_into().unwrap()));
                    c1.rotate_right(1);
                }
            } else if give == 2 {
                for j in 1..=many {
                    c2.push(c8.remove((0).try_into().unwrap()));
                    c2.rotate_right(1);
                }
            } else if give == 3 {
                for j in 1..=many {
                    c3.push(c8.remove((0).try_into().unwrap()));
                    c3.rotate_right(1);
                }
            } else if give == 4 {
                for j in 1..=many {
                    c4.push(c8.remove((0).try_into().unwrap()));
                    c4.rotate_right(1);
                }
            } else if give == 5 {
                for j in 1..=many {
                    c5.push(c8.remove((0).try_into().unwrap()));
                    c5.rotate_right(1);
                }
            } else if give == 6 {
                for j in 1..=many {
                    c6.push(c8.remove((0).try_into().unwrap()));
                    c6.rotate_right(1);
                }
            } else if give == 7 {
                for j in 1..=many {
                    c7.push(c8.remove((0).try_into().unwrap()));
                    c7.rotate_right(1);
                }
            } else if give == 9 {
                for j in 1..=many {
                    c9.push(c8.remove((0).try_into().unwrap()));
                    c9.rotate_right(1);
                }
            };    
        } else if take == 9 {
            if give == 1 {
                for j in 1..=many {
                    c1.push(c9.remove((0).try_into().unwrap()));
                    c1.rotate_right(1);
                }
            } else if give == 2 {
                for j in 1..=many {
                    c2.push(c9.remove((0).try_into().unwrap()));
                    c2.rotate_right(1);
                }
            } else if give == 3 {
                for j in 1..=many {
                    c3.push(c9.remove((0).try_into().unwrap()));
                    c3.rotate_right(1);
                }
            } else if give == 4 {
                for j in 1..=many {
                    c4.push(c9.remove((0).try_into().unwrap()));
                    c4.rotate_right(1);
                }
            } else if give == 5 {
                for j in 1..=many {
                    c5.push(c9.remove((0).try_into().unwrap()));
                    c5.rotate_right(1);
                }
            } else if give == 6 {
                for j in 1..=many {
                    c6.push(c9.remove((0).try_into().unwrap()));
                    c6.rotate_right(1);
                }
            } else if give == 7 {
                for j in 1..=many {
                    c7.push(c9.remove((0).try_into().unwrap()));
                    c7.rotate_right(1);
                }
            } else if give == 8 {
                for j in 1..=many {
                    c8.push(c9.remove((0).try_into().unwrap()));
                    c8.rotate_right(1);
                }
            };    
        };
    println!("c1:{:?}\nc2:{:?}\nc3:{:?}\nc4:{:?}\nc5:{:?}\nc6:{:?}\nc7:{:?}\nc8:{:?}\nc9:{:?}",c1, c2, c3, c4, c5, c6, c7, c8, c9);
    }

    // print how many are overlapping
    println!("Letters: {}{}{}{}{}{}{}{}{}",c1[0], c2[0], c3[0], c4[0], c5[0], c6[0], c7[0], c8[0], c9[0]);
    // println!("hmm 2 {:?}",c8);
    // println!("c1:{:?}\nc2:{:?}\nc3:{:?}\nc4:{:?}\nc5:{:?}\nc6:{:?}\nc7:{:?}\nc8:{:?}\nc9:{:?}",c1, c2, c3, c4, c5, c6, c7, c8, c9);

    
    Ok(())
}
