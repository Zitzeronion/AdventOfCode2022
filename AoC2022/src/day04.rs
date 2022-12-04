use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    // File for reading
    let f = File::open("../assignments.txt")?;
    let reader = BufReader::new(f);
    // Switch between first and second star
    let star: i32 = 2; 
    let mut count: i32 = 0;
    // Loop through the lines of the txt file
   
    'outer: for line in reader.lines() {
        // read the line into a string
        let string = line?;
        // seperate the numbers from the rest
        let t1: Vec<&str> = string.split(&['-' ,','][..]).collect();
        // use the four numbers
        let e1: i32 = t1[0].parse().unwrap();
        let e2: i32 = t1[1].parse().unwrap();
        let e3: i32 = t1[2].parse().unwrap();
        let e4: i32 = t1[3].parse().unwrap();
        // check if one set contains the other
        if e1 <= e3 && e4 <= e2 {
            println!("E1 > E3, {}-{} {}-{}", e1, e2, e3, e4);
            count += 1;
            continue;
        } else if e1 >= e3 && e4 >= e2 {
            println!("E1 < E3, {}-{} {}-{}", e1, e2, e3, e4);
            count += 1;
            continue;
        };
        if star == 2 {
            // check if sets are overlapping at all
            for i in e1..=e2 {
                for j in e3..=e4 {
                    if i == j {
                        println!("Overlap, {}-{} {}-{}", e1, e2, e3, e4);
                        count += 1;
                        continue 'outer;
                    };
                }
            }
        };
    }
    // print how many are overlapping
    println!("{}", count);
    
    Ok(())
}
