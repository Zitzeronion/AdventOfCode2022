use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    // File for reading
    let f = File::open("../calories.txt")?;
    let reader = BufReader::new(f);
    // Some vectors and counter
    let mut vec: Vec<i32> = Vec::new();
    //let mut cal: Vec<i32> = Vec::new();
    let mut j = 0i32;
    // Loop through the lines of the txt file
    for line in reader.lines() {
        let string = line?;
        // Sum up consequetive lines until the next elf
        if string.len() > 1 {
            let i = match string.parse::<i32>() {
                Ok(i) => i,
                Err(_e) => -1,
            };
            j += i;
        // If a empty line is encountered push the calory value of an elf to a vector
        // and set the calory counter to 0.
        } else {
            vec.push(j);
            j = 0;
        };
    }
    // Print the vector with the elfs 
    // println!("{:?}", vec);

    // Write of the max value :) (First star)
    let maxVal = vec.iter().max();
    match maxVal {
        Some(max) => println!( "Max value: {}", max ),
        None      => println!( "Vector is empty" ),
    }

    // For the second star I sort the vector
    vec.sort();
    println!("{:?}", vec);
    println!("{}", vec.len());
    // k is to sum up the three values
    let mut k = 0i32;
    let ll = vec.len()-3;
    let lm = vec.len();
    for n in ll..lm {
        k += vec[n];
    }
    println!("This should be the sum {}", k);
    Ok(())
}
