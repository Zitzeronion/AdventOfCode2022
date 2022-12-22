use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    const RADIX: u32 = 10;
    // Switch between first and second star
    let star: i32 = 2; 
    // Loop through the lines of the txt file
    
    let mut f = BufReader::new(File::open("../wood.txt").unwrap());

    let mut s = String::new();
    f.read_line(&mut s).unwrap();

    let arr: Vec<Vec<u32>> = f.lines()
        .map(|l| l.unwrap().chars()//.split(char::is_whitespace)
             .map(|number| number.to_digit(RADIX).unwrap())
             .collect())
        .collect();
        
    let maxel = arr[0].len(); 
    let mut ltr = vec![vec![0; maxel]; maxel];
    let mut rtl = vec![vec![0; maxel]; maxel];
    let mut utd = vec![vec![0; maxel]; maxel];
    let mut dtu = vec![vec![0; maxel]; maxel];
    let mut allt = vec![vec![0; maxel]; maxel];
    let mut alln = vec![vec![0; maxel]; maxel];
    let mut allview = vec![vec![0; maxel]; maxel];

    // left to right 
    for i in 0..maxel {
        let mut h = arr[i][0];
        ltr[i][0] = 1;
        for j in 1..maxel {
            let tree = arr[i][j];
            // println("Element {} and h {}", tree, h)
            if tree > h {
                h = tree;
                ltr[i][j] = 1;
            };
        }
    }
    //right to left
    for i in 0..maxel {
        let mut h = arr[i][98];
        rtl[i][maxel-1] = 1;
        for j in 1..maxel {
            let tree = arr[i][98-j];
            // println("Element {} and h {}", tree, h)
            if tree > h {
                h = tree;
                rtl[i][98-j] = 1;
            };
        }
    }
    // up to down
    for i in 0..maxel {
        let mut h = arr[0][i];
        utd[0][i] = 1;
        for j in 1..maxel {
            let tree = arr[j][i];
            // println("Element {} and h {}", tree, h)
            if tree > h {
                h = tree;
                utd[j][i] = 1;
            };
        }
    }
    // down to up
    for i in 0..maxel {
        let mut h = arr[98][i];
        dtu[98][i] = 1;
        for j in 1..maxel {
            let tree = arr[98-j][i];
            // println("Element {} and h {}", tree, h)
            if tree > h {
                h = tree;
                rtl[98-j][i] = 1;
            };
        }
    }
    // sum them up
    let mut count: u32 = 0;
    for i in 0..maxel {
        for j in 0..maxel {
            let h = ltr[i][j] + rtl[i][j] + utd[i][j] + dtu[i][j];
            if h > 0 {
                allt[i][j] = 1;
                alln[i][j] = arr[i][j];
                count += 1;
            }
        }
    }
    let mut maxV = 0;
    if star == 2 {
        for i in 1..maxel-1 {
            for j in 1..maxel-1 {
                let mut nu = 1;
                let mut nd = 1;
                let mut nl = 1;
                let mut nr = 1;
                
                let h = arr[i][j];
                for k in j+1..maxel-1 {
                    if arr[i][k] < h {
                        nr += 1;
                    } else {
                        break;
                    };
                }
                for k in 1..j {
                    if arr[i][j-k] < h {
                        nl += 1;
                    } else {
                        break;
                    }
                }
                for k in i+1..maxel-1 {
                    if arr[k][j] < h {
                        nu += 1;
                    } else {
                        break;
                    }
                }
                for k in 1..i {
                    if arr[i-k][j] < h {
                        nd += 1;
                    } else {
                        break;
                    }
                } 
                let viewprod = nd * nu * nl * nr;
                
                allview[i][j] = viewprod;
                if viewprod > maxV {
                    println!("Index [{},{}] {}\nnl {}\nnr {}\nnu {}\nnd {}\nprod {}", i, j, arr[i][j], nl, nr, nu, nd, viewprod);
                    maxV = viewprod;
                } 
                
            }
        }
    }
    
    println!("First guess trees: {}", count);
    // println!("{:?}", allview[3]);
    // println!("{:?}", alln[3]);
    println!("Maxview: {}",maxV);
    Ok(())
}