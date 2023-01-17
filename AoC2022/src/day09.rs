use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    // read input
    let (input_dir, input_mv) = read_input("../rope.txt");
    // println!("{:?}", input_dir);
    let star = 2;
    let mut head_pos = vec![(0, 0)];
    let mut knot_1 = vec![(0, 0)];
    let mut knot_2 = vec![(0, 0)];
    let mut knot_3 = vec![(0, 0)];
    let mut knot_4 = vec![(0, 0)];
    let mut knot_5 = vec![(0, 0)];
    let mut knot_6 = vec![(0, 0)];
    let mut knot_7 = vec![(0, 0)];
    let mut knot_8 = vec![(0, 0)];
    let mut tail_pos = vec![(0, 0)];
    let mut tail_h: Vec<(i32, i32)> = Vec::new();
    let text = true;
    let notext = false;
    // println!("{}", head_pos[0].1);
    // head_pos[0].1 += 1;
    // println!("{}", head_pos[0].0);
    // head_move('L', &mut head_pos);
    // println!("{}", head_pos[0].0);
    // let dist = distance(&mut head_pos, &mut tail_pos);
    // println!("They should be appart {}", dist);
    for i in 0..input_mv.len() {
        for _j in 0..input_mv[i] {
            if star == 1 {
                let head_last = head_pos.clone();
                head_move(input_dir[i], &mut head_pos);
                let dist = distance(&mut head_pos, &mut tail_pos);
                move_tail(head_last, &mut tail_pos, dist);
                // println!("Head: {:?} and Tail {:?}", head_pos, tail_pos);
                tail_h.push(tail_pos[0]);

            } else if star == 2 {
                if i == 0 {
                    let head_last = head_pos.clone();
                    head_move(input_dir[i], &mut head_pos);
                    // first knot
                    let k_1 = knot_1.clone();
                    move_here(&mut head_pos, head_last, &mut knot_1, notext);
                    // second knot
                    let k_2 = knot_2.clone();
                    move_here(&mut knot_1, k_1, &mut knot_2, notext);
                    // thrid knot
                    let k_3 = knot_3.clone();
                    move_here(&mut knot_2, k_2, &mut knot_3, notext);
                    // forth knot
                    let k_4 = knot_4.clone();
                    move_here(&mut knot_3, k_3, &mut knot_4, notext);
                    // fifth knot
                    let k_5 = knot_5.clone();
                    move_here(&mut knot_4, k_4, &mut knot_5, notext);
                    // sixth knot
                    let k_6 = knot_6.clone();
                    move_here(&mut knot_5, k_5, &mut knot_6, notext);
                    // seventh knot
                    let k_7 = knot_7.clone();
                    move_here(&mut knot_6, k_6, &mut knot_7, notext);
                    // eight knot
                    let k_8 = knot_8.clone();
                    move_here(&mut knot_7, k_7, &mut knot_8, notext);
                    // tail
                    move_here(&mut knot_8, k_8, &mut tail_pos, notext);

                    tail_h.push(tail_pos[0]);
                    // knot8_h.push(knot_1[0]);
                    println!("head {:?} k1 {:?} k2 {:?} k3 {:?} k4 {:?} k5 {:?} k6 {:?} k7 {:?} k8 {:?} tail {:?}", 
                    head_pos[0], knot_1[0], knot_2[0], knot_3[0], knot_4[0], 
                    knot_5[0], knot_6[0], knot_7[0], knot_8[0], tail_pos[0]);
                } else {
                    let head_last = head_pos.clone();
                    head_move(input_dir[i], &mut head_pos);
                    // first knot
                    // let k_1 = knot_1.clone();
                    find_closest(&mut head_pos, &mut knot_1);
                    //move_here(&mut head_pos, head_last, &mut knot_1, notext);
                    // second knot
                    // let k_2 = knot_2.clone();
                    find_closest(&mut knot_1, &mut knot_2);
                    //move_knot(&mut knot_1, &mut knot_2, input_dir[i-1], input_dir[i]);
                    // thrid knot
                    // let k_3 = knot_3.clone();
                    find_closest(&mut knot_2, &mut knot_3);
                    //move_knot(&mut knot_2, &mut knot_3, input_dir[i-1], input_dir[i]);
                    // forth knot
                    // let k_4 = knot_4.clone();
                    find_closest(&mut knot_3, &mut knot_4);
                    //move_knot(&mut knot_3, &mut knot_4, input_dir[i-1], input_dir[i]);
                    // fifth knot
                    // let k_5 = knot_5.clone();
                    find_closest(&mut knot_4, &mut knot_5);
                    //move_knot(&mut knot_4, &mut knot_5, input_dir[i-1], input_dir[i]);
                    // sixth knot
                    // let k_6 = knot_6.clone();
                    find_closest(&mut knot_5, &mut knot_6);
                    //move_knot(&mut knot_5, &mut knot_6, input_dir[i-1], input_dir[i]);
                    // seventh knot
                    // let k_7 = knot_7.clone();
                    find_closest(&mut knot_6, &mut knot_7);
                    //move_knot(&mut knot_6, &mut knot_7, input_dir[i-1], input_dir[i]);
                    // eight knot
                    // let k_8 = knot_8.clone();
                    find_closest(&mut knot_7, &mut knot_8);
                    //move_knot(&mut knot_7, &mut knot_8, input_dir[i-1], input_dir[i]);
                    // tail
                    find_closest(&mut knot_8, &mut tail_pos);
                    //move_knot(&mut knot_8, &mut tail_pos, input_dir[i-1], input_dir[i]);

                    tail_h.push(tail_pos[0]);
                    println!("head {:?} k1 {:?} k2 {:?} k3 {:?} k4 {:?} k5 {:?} k6 {:?} k7 {:?} k8 {:?} tail {:?}", 
                    head_pos[0], knot_1[0], knot_2[0], knot_3[0], knot_4[0], 
                    knot_5[0], knot_6[0], knot_7[0], knot_8[0], tail_pos[0]);
                }
            }
        }
    }
    // for i in 0..tail_h.len() {
    //     println!("Before {:?} tail {:?}", knot8_h[i], tail_h[i]);
    // }
    
    let mut history = tail_h.clone();
    history.sort();
    history.dedup();
    for i in 0..history.len() {
        println!("{:?}", history[i]);
    }
    println!("Unique elements {}", history.len());
    
    Ok(())
}

fn read_input(file: &str) -> (Vec<char>, Vec<i32>) {
    let mut f = File::open(file).unwrap();
    let mut data = String::new();
    // Switch between first and second star
    let mut dir_list: Vec<char> = Vec::new();
    let mut move_list: Vec<i32> = Vec::new();
    f.read_to_string(&mut data).unwrap();
    // println!("{}", data);
    let t1: Vec<&str> = data.split(&['\n'][..]).collect();
    for i in 0..t1.len() {
        // println!("{} {}", i, t1[i]);
        let t2: Vec<&str> = t1[i].split_whitespace().collect();
        let n_moves: i32 = t2[1].parse().unwrap();
        dir_list.push(t2[0].chars().next().expect("string is empty"));
        // println!("{}", n_moves);
        move_list.push(n_moves);
    }
    (dir_list, move_list)
}

fn head_move(direction: char, head: &mut Vec<(i32, i32)>) {
    // println!("({},{})", head[0].0, head[0].1);
    if direction == 'R' {
        head[0].0 += 1;
    } else if direction == 'L' {
        head[0].0 -= 1;
        // println!("head is at ({},{})", head[0].0, head[0].1);
    } else if direction == 'U' {
        head[0].1 += 1;
    } else if direction == 'D' {
        head[0].1 -= 1;
    } else {
        println!{"Something is wrong!"};
    }
} 

fn distance(head: &mut Vec<(i32, i32)>, tail: &mut Vec<(i32, i32)>) -> f64 {
    let mut scalarp = 0.0_f64;
    scalarp = ((head[0].0 - tail[0].0)*(head[0].0 - tail[0].0) + (head[0].1 - tail[0].1)*(head[0].1 - tail[0].1)).into();
    let sqrtsc = scalarp.sqrt();
    sqrtsc
}

fn move_tail(head_t1: Vec<(i32, i32)>, tail: &mut Vec<(i32, i32)>, dist: f64) {
    if dist == 0.0 {
        // no need to act
    } else if dist == 2.0_f64.sqrt() {
        // same here
    } else if dist == 1.0 {
        // here as well
    } else {
        // but now the tail has to move!
        *tail = head_t1.to_vec();
    }
}

fn move_here(head_pos: &mut Vec<(i32, i32)>, head_last: Vec<(i32, i32)>, tail_pos: &mut Vec<(i32, i32)>, text: bool) {
    let dist = distance(head_pos, tail_pos);
    move_tail(head_last, tail_pos, dist);
    if text {
        println!("Head: {:?} and Tail {:?}", head_pos[0], tail_pos[0]);
    }
}

fn move_knot(head_knot: &mut Vec<(i32, i32)>, tail_knot: &mut Vec<(i32, i32)>, last_dir: char, new_dir: char) {
    let dist = distance(head_knot, tail_knot);
    if dist == 2.0_f64 {
        head_move(new_dir, tail_knot);
    } else if (dist > 2.0_f64.sqrt()) && !(dist == 2.0_f64) {
        if last_dir == 'R' {
            tail_knot[0].0 += 1;
            head_move(new_dir, tail_knot);
        } else if last_dir == 'L' {
            tail_knot[0].0 -= 1;
            head_move(new_dir, tail_knot);
        } else if last_dir == 'U' {
            tail_knot[0].1 += 1;
            head_move(new_dir, tail_knot);
        } else if last_dir == 'D' {
            tail_knot[0].1 -= 1;
            head_move(new_dir, tail_knot);
        } else {
            println!{"Something is wrong!"};
        }
    } 
}

fn find_closest(head_knot: &mut Vec<(i32, i32)>, tail_knot: &mut Vec<(i32, i32)>) {
    let dist = distance(head_knot, tail_knot);
    
    if dist == 2.0_f64 {
        for i in 0..4 {
            let mut prob = tail_knot.clone();
            if i == 0 {
                prob[0].0 += 1;
                let new_dist = distance(head_knot, &mut prob);
                if new_dist == 1.0_f64 {
                    *tail_knot = prob.to_vec();
                    break;
                }
            } else if i == 1 {
                prob[0].0 -= 1;
                let new_dist = distance(head_knot, &mut prob);
                if new_dist == 1.0_f64 {
                    *tail_knot = prob.to_vec();
                    break;
                }
            } else if i == 2 {
                prob[0].1 += 1;
                let new_dist = distance(head_knot, &mut prob);
                if new_dist == 1.0_f64 {
                    *tail_knot = prob.to_vec();
                    break;
                }
            } else if i == 3 {
                prob[0].1 -= 1;
                let new_dist = distance(head_knot, &mut prob);
                if new_dist == 1.0_f64 {
                    *tail_knot = prob.to_vec();
                    break;
                }
            } else {
                println!{"Something is wrong!"};
            }
        }
    } else if (dist > 2.0_f64.sqrt()) && !(dist == 2.0_f64) {
        for i in 0..4 {
            let mut prob = tail_knot.clone();
            if i == 0 {
                prob[0].0 += 1;
                prob[0].1 += 1;
                let new_dist = distance(head_knot, &mut prob);
                if new_dist <= 1.8_f64 {
                    *tail_knot = prob.to_vec();
                    break;
                }
            } else if i == 1 {
                prob[0].0 -= 1;
                prob[0].1 -= 1;
                let new_dist = distance(head_knot, &mut prob);
                if new_dist <= 1.8_f64 {
                    *tail_knot = prob.to_vec();
                    break;
                }
            } else if i == 2 {
                prob[0].0 += 1;
                prob[0].1 -= 1;
                let new_dist = distance(head_knot, &mut prob);
                if new_dist <= 1.8_f64 {
                    *tail_knot = prob.to_vec();
                    break;
                }
            } else if i == 3 {
                prob[0].0 -= 1;
                prob[0].1 += 1;
                let new_dist = distance(head_knot, &mut prob);
                if new_dist <= 1.8_f64 {
                    *tail_knot = prob.to_vec();
                    break;
                }
            } else {
                println!{"Something is wrong!"};
            }
        }
    } 
}