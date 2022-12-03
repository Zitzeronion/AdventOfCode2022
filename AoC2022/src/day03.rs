use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    // File for reading
    let f = File::open("../Rucksack.txt")?;
    let reader = BufReader::new(f);
    // Items in both or all three rucksacks and the values
    let mut items: Vec<char> = Vec::new();
    let mut values: Vec<i32> = Vec::new();
    let mut stuff: Vec<String> = Vec::new();
    // Switch between first and second star
    let star: i32 = 2; 
    // Loop through the lines of the txt file
    if star == 1 {
        for line in reader.lines() {
            // read the line into a string
            let string = line?;
            // half the string, there are two compartments
            let complen = string.len()/2;
            // checkingg if this is indeed a int
            println!("{}", complen);
            // loop through the characters to find the matching ones
            'outer: for i in 1..complen+1 {
                let comp1 = string.chars().take(i).last().unwrap();
                '_inner: for j in complen+1..2*complen+1 {
                    let comp2 = string.chars().take(j).last().unwrap();
                    if comp2 == comp1 {
                        // item.push(comp2);
                        println!("Found the char appearing in both: {} and one {}", comp2, comp1);
                        items.push(comp2);
                        values.push(char2number(comp2));
                        // break the loop when one was found
                        break 'outer;
                    };
                }
            }
        }
    } else if star == 2 {
        // Here we need to find the common char in three rucksacks
        for line in reader.lines() {
            let hmm = line?;
            // First all strings are pushed into a vector
            stuff.push(hmm);
        }
        // Now we split them in to bunches of three (yes not the nicest way, but gets the job done)
        let entries = stuff.len()/3;
        println!("{}", entries);
        // Looping over these bunches
        for i in 0..entries {
            // basically strings
            let ruck1 = &stuff[0+i*3];
            let ruck2 = &stuff[1+i*3];
            let ruck3 = &stuff[2+i*3];
            // Iterate over all chars
            'first: for ch1 in ruck1.chars() {
                'sec: for ch2 in ruck2.chars() {
                    'thrid: for ch3 in ruck3.chars() {
                        if ch1 == ch2 && ch2 == ch3 {
                            // Find the one that appears in all of them
                            println!("Well well well, at last {}", ch3);
                            items.push(ch3);
                            values.push(char2number(ch3));
                            //break
                            break 'first;
                        }
                    }
                }
            }
        }
    }
    
    println!("Here are the letters:     {:?}", items);
    println!("Here are the priorities:  {:?}", values);
    let p: i32 = values.iter().sum();
    println!("Here is the sum:          {}", p);

    Ok(())
}
// Super straightforward letter to number function.
fn char2number(item: char) -> i32 {
    // Return value, default 0
    let mut star: i32 = 0; 
    // Stupid but working list of letters and values
    let alphabet = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let Alphabet = vec!['A','B','C','D','E','F','G','H','J','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let val = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26];
    // Loop alphabet
    for i in 0..26 {
        // Take a letter
        let lc: char = alphabet[i];
        let up: char = Alphabet[i];
        // Letter found, get a number
        if item == lc {
            star = val[i];
        } else if item == up {
            star = val[i]+26;
        };
    }
    // New value
    star
} 