use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    // read input
    // let regist = read_input("../calories_small.txt");
    let regist = read_input("../cycles.txt");
    println!("{:?}", regist);
    let star = 1;
    println!("{:?}", regist.last().unwrap().1);
    let mut helper = regist.clone();
    let hmm = full_list(&mut helper);
    let img = hmm.clone();
    let n = get_signal(hmm);
    println!("{}", n);
    let s = draw_signal(img);
    println!("{}", s);
    Ok(())
}

fn read_input(file: &str) -> Vec<(i32, i32)> {
    let mut f = File::open(file).unwrap();
    let mut data = String::new();
    // Switch between first and second star
    let mut reg_list: Vec<(i32, i32)> = vec![(1, 1)];
    f.read_to_string(&mut data).unwrap();
    // println!("{}", data);
    let mut register = 1;
    let mut cycle = 1;
    let t1: Vec<&str> = data.split(&['\n'][..]).collect();
    for i in 0..t1.len() {
        let t2: Vec<&str> = t1[i].split_whitespace().collect();
        // println!("{} {:?}", i, t2);
        if t2.len() == 1 {
            cycle += 1;
            reg_list.push((cycle, register));
            // println!("Yes! in {}", i);
        } else {
            let addn: i32 = t2[1].parse().unwrap();
            cycle += 2;
            register += addn; 
            reg_list.push((cycle, register));
        }
        // println!("{}", n_moves);
        // move_list.push(n_moves);
    }
    reg_list
}

fn full_list(rig: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut comprig: Vec<(i32, i32)> = vec![(0, 1)];
    let maxr = rig.last().unwrap().0;
    let mut clock: Vec<i32> = vec![0];
    let mut num: Vec<i32> = vec![1];
    let mut count: i32 = 0;
    let mut creg: i32 = 1;
    for i in 1..=maxr {
        count += 1;
        clock.push(count);
        for j in 0..rig.len() {
            if rig[j].0 == i {
                creg = rig[j].1;
                break;
            }
        }
        num.push(creg);
        comprig.push((count, creg));
    }
    println!("{:?}", comprig);
    comprig
} 

fn get_signal(rig: Vec<(i32, i32)>) -> i32 {
    let n1 = rig[20].0 * rig[20].1;
    let n2 = rig[60].0 * rig[60].1;
    let n3 = rig[100].0 * rig[100].1;
    let n4 = rig[140].0 * rig[140].1;
    let n5 = rig[180].0 * rig[180].1;
    let n6 = rig[220].0 * rig[220].1;
    
    let nsum = n1 + n2 + n3 + n4 + n5 + n6;

    nsum
}

fn draw_signal(rig: Vec<(i32, i32)>) -> String {
    let mut sig = "".to_string(); 
    for i in 1..rig.len() {
        if i < 41 {
            if (rig[i].0 - 1 == rig[i].1) || (rig[i].0 - 1 == rig[i].1-1) || (rig[i].0 - 1 == rig[i].1+1) {
                if i == 40 {
                    sig = format!("{}#\n",sig);
                } else {
                    sig = format!("{}#",sig);
                }
            } else {
                if i == 40 {
                    sig = format!("{}.\n",sig);
                } else {
                    sig = format!("{}.",sig);
                }
            }
        } else if (i < 81) && (i > 40) {
            if (rig[i].0 - 41 == rig[i].1) || (rig[i].0 - 41 == rig[i].1-1) || (rig[i].0 - 41 == rig[i].1+1) {
                if i == 80 {
                    sig = format!("{}#\n",sig);
                } else {
                    sig = format!("{}#",sig);
                }
            } else {
                if i == 80 {
                    sig = format!("{}.\n",sig);
                } else {
                    sig = format!("{}.",sig);
                }
            }
        } else if (i < 121) && (i > 40) {
            if (rig[i].0 - 81 == rig[i].1) || (rig[i].0 - 81 == rig[i].1-1) || (rig[i].0 - 81 == rig[i].1+1) {
                if i == 120 {
                    sig = format!("{}#\n",sig);
                } else {
                    sig = format!("{}#",sig);
                }
            } else {
                if i == 120 {
                    sig = format!("{}.\n",sig);
                } else {
                    sig = format!("{}.",sig);
                }
            }
        } else if (i < 161) && (i > 120) {
            if (rig[i].0 - 121 == rig[i].1) || (rig[i].0 - 121 == rig[i].1-1) || (rig[i].0 - 121 == rig[i].1+1) {
                if i == 160  {
                    sig = format!("{}#\n",sig);
                } else {
                    sig = format!("{}#",sig);
                }
            } else {
                if i == 160 {
                    sig = format!("{}.\n",sig);
                } else {
                    sig = format!("{}.",sig);
                }
            }
        } else if (i < 201) && (i > 160) {
            if (rig[i].0 - 161 == rig[i].1) || (rig[i].0 - 161 == rig[i].1-1) || (rig[i].0 - 161 == rig[i].1+1) {
                if i == 200  {
                    sig = format!("{}#\n",sig);
                } else {
                    sig = format!("{}#",sig);
                }
            } else {
                if i == 200 {
                    sig = format!("{}.\n",sig);
                } else {
                    sig = format!("{}.",sig);
                }
            }
        } else {
            if (rig[i].0 - 201 == rig[i].1) || (rig[i].0 - 201 == rig[i].1-1) || (rig[i].0 - 201 == rig[i].1+1) {
                sig = format!("{}#",sig);
            } else {
                sig = format!("{}.",sig);
            }
        }    
    }

    sig
}






