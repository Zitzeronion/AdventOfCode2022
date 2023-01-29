use std::io;
use std::io::prelude::*;
use std::fs::File;

struct Monkey {
    number: i32,
    method: String,
    items: Vec<i128>,
    div: i128,
    yes: i32,
    no: i32,
    inspected: i32,
}

fn main() -> io::Result<()> {
    // read input
    let mut m0 = read_to_struct("../monkeys.txt", 0);
    let mut m1 = read_to_struct("../monkeys.txt", 1);
    let mut m2 = read_to_struct("../monkeys.txt", 2);
    let mut m3 = read_to_struct("../monkeys.txt", 3);
    let mut m4 = read_to_struct("../monkeys.txt", 4);
    let mut m5 = read_to_struct("../monkeys.txt", 5);
    let mut m6 = read_to_struct("../monkeys.txt", 6);
    let mut m7 = read_to_struct("../monkeys.txt", 7);

    let star = 2;
    
    for i in 0..7 {
        action_m0(&mut m0, &mut m1, &mut m2, &mut m3, &mut m4, &mut m5, &mut m6, &mut m7, star);
        action_m1(&mut m0, &mut m1, &mut m2, &mut m3, &mut m4, &mut m5, &mut m6, &mut m7, star);
        action_m2(&mut m0, &mut m1, &mut m2, &mut m3, &mut m4, &mut m5, &mut m6, &mut m7, star);
        action_m3(&mut m0, &mut m1, &mut m2, &mut m3, &mut m4, &mut m5, &mut m6, &mut m7, star);
        action_m4(&mut m0, &mut m1, &mut m2, &mut m3, &mut m4, &mut m5, &mut m6, &mut m7, star);
        action_m5(&mut m0, &mut m1, &mut m2, &mut m3, &mut m4, &mut m5, &mut m6, &mut m7, star);
        action_m6(&mut m0, &mut m1, &mut m2, &mut m3, &mut m4, &mut m5, &mut m6, &mut m7, star);
        action_m7(&mut m0, &mut m1, &mut m2, &mut m3, &mut m4, &mut m5, &mut m6, &mut m7, star);
        println!("{:?}\n", m0.items)
    }
    

    let mut bus = vec![m0.inspected, m1.inspected, m2.inspected, m3.inspected, m4.inspected, m5.inspected, m6.inspected, m7.inspected];
    bus.sort();
    println!("{:?}", bus);
    println!("Monkey business {}", bus[6]*bus[7]);
    Ok(())
}

fn read_to_struct(file: &str, nmonkey: i32) -> Monkey {
    let mut f = File::open(file).unwrap();
    let mut data = String::new();
    
    let mut m = Monkey {
        number: 0,
        method: "hmm".to_string(),
        items: [0, 0].to_vec(),
        div: 12,
        yes: 2,
        no: 1,
        inspected: 0,
    };
    f.read_to_string(&mut data).unwrap();
    let t1: Vec<&str> = data.split(&['\n'][..]).collect();
    m.number = nmonkey;
    for i in 0..t1.len() {
        let mst = format!("Monkey {}", nmonkey);
        if t1[i].contains(&mst) {
            for j in i+1..i+6 {
                let t2: Vec<&str> = t1[j].split_whitespace().collect();
                if t1[j].contains("Starting") {
                    let mut ivec: Vec<i128> = Vec::new();
                    for k in 2..t2.len() {
                        let numi = t2[k].replace(",", "");
                        ivec.push(numi.parse().unwrap());
                    }
                    m.items = ivec; 
                } else if t1[j].contains("Operation") {
                    m.method = format!("{} {} {}", t2[3], t2[4], t2[5]);
                } else if t1[j].contains("Test") {
                    m.div = t2[3].parse().unwrap();
                } else if t1[j].contains("If true") {
                    m.yes = t2[5].parse().unwrap();
                } else if t1[j].contains("false") {
                    m.no = t2[5].parse().unwrap();
                }
            }
        }
    }
    println!("{:?}",m.items);
    m
}

fn action_m0(m0: &mut Monkey, 
    m1: &mut Monkey, 
    m2: &mut Monkey, 
    m3: &mut Monkey, 
    m4: &mut Monkey, 
    m5: &mut Monkey, 
    m6: &mut Monkey, 
    m7: &mut Monkey,
    star: i32) {
    let mut m = m0;
    
    let op: Vec<&str> = m.method.split_whitespace().collect();
    // loop through items
    for i in 0..m.items.len() {
        m.inspected += 1;
        let mut newi = 0.0_f32;
        if (op[0] == "old") && (op[1] == "*") {
            if op[2] == "old" {
                newi = (m.items[i] * m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] * known) as f32;
            }
        } else if (op[0] == "old") && (op[1] == "+") {
            if op[2] == "old" {
                newi = (m.items[i] + m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] + known) as f32;
            }
        }
        let mut borring = 0.0;
        if star == 1 {
            borring = newi / 3.0;
        } else if star == 2 {
            borring = newi / 1.0;
        }
        let val = borring.floor() as i128;
        if val % m.div == 0 {
            if m.yes == 1 {
                m1.items.push(val);
            } else if m.yes == 2 {
                m2.items.push(val);
            } else if m.yes == 3 {
                m3.items.push(val);
            } else if m.yes == 4 {
                m4.items.push(val);
            } else if m.yes == 5 {
                m5.items.push(val);
            } else if m.yes == 6 {
                m6.items.push(val);
            } else if m.yes == 7 {
                m7.items.push(val);
            }
        } else {
            if m.no == 1 {
                m1.items.push(val);
            } else if m.no == 2 {
                m2.items.push(val);
            } else if m.no == 3 {
                m3.items.push(val);
            } else if m.no == 4 {
                m4.items.push(val);
            } else if m.no == 5 {
                m5.items.push(val);
            } else if m.no == 6 {
                m6.items.push(val);
            } else if m.no == 7 {
                m7.items.push(val);
            }
        }
    }
    m.items = Vec::new(); 
}

fn action_m1(m0: &mut Monkey, 
    m1: &mut Monkey, 
    m2: &mut Monkey, 
    m3: &mut Monkey, 
    m4: &mut Monkey, 
    m5: &mut Monkey, 
    m6: &mut Monkey, 
    m7: &mut Monkey, 
    star: i32) {
    let mut m = m1;
    
    let op: Vec<&str> = m.method.split_whitespace().collect();
    // loop through items
    for i in 0..m.items.len() {
        m.inspected += 1;
        let mut newi = 0.0_f32;
        if (op[0] == "old") && (op[1] == "*") {
            if op[2] == "old" {
                newi = (m.items[i] * m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] * known) as f32;
            }
        } else if (op[0] == "old") && (op[1] == "+") {
            if op[2] == "old" {
                newi = (m.items[i] + m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] + known) as f32;
            }
        }
        let mut borring = 0.0;
        if star == 1 {
            borring = newi / 3.0;
        } else if star == 2{
            borring = newi / 1.0;
        }
        let val = borring.floor() as i128;
        if val % m.div == 0 {
            if m.yes == 0 {
                m0.items.push(val);
            } else if m.yes == 2 {
                m2.items.push(val);
            } else if m.yes == 3 {
                m3.items.push(val);
            } else if m.yes == 4 {
                m4.items.push(val);
            } else if m.yes == 5 {
                m5.items.push(val);
            } else if m.yes == 6 {
                m6.items.push(val);
            } else if m.yes == 7 {
                m7.items.push(val);
            }
        } else {
            if m.no == 0 {
                m0.items.push(val);
            } else if m.no == 2 {
                m2.items.push(val);
            } else if m.no == 3 {
                m3.items.push(val);
            } else if m.no == 4 {
                m4.items.push(val);
            } else if m.no == 5 {
                m5.items.push(val);
            } else if m.no == 6 {
                m6.items.push(val);
            } else if m.no == 7 {
                m7.items.push(val);
            }
        }
    }
    m.items = Vec::new(); 
}

fn action_m2(m0: &mut Monkey, 
    m1: &mut Monkey, 
    m2: &mut Monkey, 
    m3: &mut Monkey, 
    m4: &mut Monkey, 
    m5: &mut Monkey, 
    m6: &mut Monkey, 
    m7: &mut Monkey, 
    star: i32) {
    let mut m = m2;
    
    let op: Vec<&str> = m.method.split_whitespace().collect();
    // loop through items
    for i in 0..m.items.len() {
        m.inspected += 1;
        let mut newi = 0.0_f32;
        if (op[0] == "old") && (op[1] == "*") {
            if op[2] == "old" {
                newi = (m.items[i] * m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] * known) as f32;
            }
        } else if (op[0] == "old") && (op[1] == "+") {
            if op[2] == "old" {
                newi = (m.items[i] + m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] + known) as f32;
            }
        }
        let mut borring = 0.0;
        if star == 1 {
            borring = newi / 3.0;
        } else if star == 2 {
            borring = newi / 1.0;
        }
        let val = borring.floor() as i128;
        if val % m.div == 0 {
            if m.yes == 1 {
                m1.items.push(val);
            } else if m.yes == 0 {
                m0.items.push(val);
            } else if m.yes == 3 {
                m3.items.push(val);
            } else if m.yes == 4 {
                m4.items.push(val);
            } else if m.yes == 5 {
                m5.items.push(val);
            } else if m.yes == 6 {
                m6.items.push(val);
            } else if m.yes == 7 {
                m7.items.push(val);
            }
        } else {
            if m.no == 1 {
                m1.items.push(val);
            } else if m.no == 0 {
                m0.items.push(val);
            } else if m.no == 3 {
                m3.items.push(val);
            } else if m.no == 4 {
                m4.items.push(val);
            } else if m.no == 5 {
                m5.items.push(val);
            } else if m.no == 6 {
                m6.items.push(val);
            } else if m.no == 7 {
                m7.items.push(val);
            }
        }
    }
    m.items = Vec::new(); 
}

fn action_m3(m0: &mut Monkey, 
    m1: &mut Monkey, 
    m2: &mut Monkey, 
    m3: &mut Monkey, 
    m4: &mut Monkey, 
    m5: &mut Monkey, 
    m6: &mut Monkey, 
    m7: &mut Monkey,
    star: i32) {
    let mut m = m3;
    
    let op: Vec<&str> = m.method.split_whitespace().collect();
    // loop through items
    for i in 0..m.items.len() {
        m.inspected += 1;
        let mut newi = 0.0_f32;
        if (op[0] == "old") && (op[1] == "*") {
            if op[2] == "old" {
                newi = (m.items[i] * m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] * known) as f32;
            }
        } else if (op[0] == "old") && (op[1] == "+") {
            if op[2] == "old" {
                newi = (m.items[i] + m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] + known) as f32;
            }
        }
        let mut borring = 0.0;
        if star == 1 {
            borring = newi / 3.0;
        } else if star == 2{
            borring = newi / 1.0;
        }
        let val = borring.floor() as i128;
        if val % m.div == 0 {
            if m.yes == 1 {
                m1.items.push(val);
            } else if m.yes == 2 {
                m2.items.push(val);
            } else if m.yes == 0 {
                m0.items.push(val);
            } else if m.yes == 4 {
                m4.items.push(val);
            } else if m.yes == 5 {
                m5.items.push(val);
            } else if m.yes == 6 {
                m6.items.push(val);
            } else if m.yes == 7 {
                m7.items.push(val);
            }
        } else {
            if m.no == 1 {
                m1.items.push(val);
            } else if m.no == 2 {
                m2.items.push(val);
            } else if m.no == 0 {
                m0.items.push(val);
            } else if m.no == 4 {
                m4.items.push(val);
            } else if m.no == 5 {
                m5.items.push(val);
            } else if m.no == 6 {
                m6.items.push(val);
            } else if m.no == 7 {
                m7.items.push(val);
            }
        }
    }
    m.items = Vec::new(); 
}

fn action_m4(m0: &mut Monkey, 
    m1: &mut Monkey, 
    m2: &mut Monkey, 
    m3: &mut Monkey, 
    m4: &mut Monkey, 
    m5: &mut Monkey, 
    m6: &mut Monkey, 
    m7: &mut Monkey,
    star: i32) {
    let mut m = m4;
    
    let op: Vec<&str> = m.method.split_whitespace().collect();
    // loop through items
    for i in 0..m.items.len() {
        m.inspected += 1;
        let mut newi = 0.0_f32;
        if (op[0] == "old") && (op[1] == "*") {
            if op[2] == "old" {
                newi = (m.items[i] * m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] * known) as f32;
            }
        } else if (op[0] == "old") && (op[1] == "+") {
            if op[2] == "old" {
                newi = (m.items[i] + m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] + known) as f32;
            }
        }
        let mut borring = 0.0;
        if star == 1 {
            borring = newi / 3.0;
        } else if star == 2 {
            borring = newi / 1.0;
        }
        let val = borring.floor() as i128;
        if val % m.div == 0 {
            if m.yes == 1 {
                m1.items.push(val);
            } else if m.yes == 2 {
                m2.items.push(val);
            } else if m.yes == 3 {
                m3.items.push(val);
            } else if m.yes == 0 {
                m0.items.push(val);
            } else if m.yes == 5 {
                m5.items.push(val);
            } else if m.yes == 6 {
                m6.items.push(val);
            } else if m.yes == 7 {
                m7.items.push(val);
            }
        } else {
            if m.no == 1 {
                m1.items.push(val);
            } else if m.no == 2 {
                m2.items.push(val);
            } else if m.no == 3 {
                m3.items.push(val);
            } else if m.no == 0 {
                m0.items.push(val);
            } else if m.no == 5 {
                m5.items.push(val);
            } else if m.no == 6 {
                m6.items.push(val);
            } else if m.no == 7 {
                m7.items.push(val);
            }
        }
    }
    m.items = Vec::new(); 
}

fn action_m5(m0: &mut Monkey, 
    m1: &mut Monkey, 
    m2: &mut Monkey, 
    m3: &mut Monkey, 
    m4: &mut Monkey, 
    m5: &mut Monkey, 
    m6: &mut Monkey, 
    m7: &mut Monkey,
    star: i32) {
    let mut m = m5;
    
    let op: Vec<&str> = m.method.split_whitespace().collect();
    // loop through items
    for i in 0..m.items.len() {
        m.inspected += 1;
        let mut newi = 0.0_f32;
        if (op[0] == "old") && (op[1] == "*") {
            if op[2] == "old" {
                newi = (m.items[i] * m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] * known) as f32;
            }
        } else if (op[0] == "old") && (op[1] == "+") {
            if op[2] == "old" {
                newi = (m.items[i] + m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] + known) as f32;
            }
        }
        let mut borring = 0.0;
        if star == 1 {
            borring = newi / 3.0;
        } else if star == 2 {
            borring = newi / 1.0;
        }
        let val = borring.floor() as i128;
        if val % m.div == 0 {
            if m.yes == 1 {
                m1.items.push(val);
            } else if m.yes == 2 {
                m2.items.push(val);
            } else if m.yes == 3 {
                m3.items.push(val);
            } else if m.yes == 4 {
                m4.items.push(val);
            } else if m.yes == 0 {
                m0.items.push(val);
            } else if m.yes == 6 {
                m6.items.push(val);
            } else if m.yes == 7 {
                m7.items.push(val);
            }
        } else {
            if m.no == 1 {
                m1.items.push(val);
            } else if m.no == 2 {
                m2.items.push(val);
            } else if m.no == 3 {
                m3.items.push(val);
            } else if m.no == 4 {
                m4.items.push(val);
            } else if m.no == 0 {
                m0.items.push(val);
            } else if m.no == 6 {
                m6.items.push(val);
            } else if m.no == 7 {
                m7.items.push(val);
            }
        }
    }
    m.items = Vec::new(); 
}

fn action_m6(m0: &mut Monkey, 
    m1: &mut Monkey,
    m2: &mut Monkey, 
    m3: &mut Monkey, 
    m4: &mut Monkey, 
    m5: &mut Monkey, 
    m6: &mut Monkey, 
    m7: &mut Monkey,
    star: i32) {
    let mut m = m6;
    
    let op: Vec<&str> = m.method.split_whitespace().collect();
    // loop through items
    for i in 0..m.items.len() {
        m.inspected += 1;
        let mut newi = 0.0_f32;
        if (op[0] == "old") && (op[1] == "*") {
            if op[2] == "old" {
                newi = (m.items[i] * m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] * known) as f32;
            }
        } else if (op[0] == "old") && (op[1] == "+") {
            if op[2] == "old" {
                newi = (m.items[i] + m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] + known) as f32;
            }
        }
        let mut borring = 0.0;
        if star == 1 {
            borring = newi / 3.0;
        } else if star == 2 {
            borring = newi / 1.0;
        }
        let val = borring.floor() as i128;
        if val % m.div == 0 {
            if m.yes == 1 {
                m1.items.push(val);
            } else if m.yes == 2 {
                m2.items.push(val);
            } else if m.yes == 3 {
                m3.items.push(val);
            } else if m.yes == 4 {
                m4.items.push(val);
            } else if m.yes == 5 {
                m5.items.push(val);
            } else if m.yes == 0 {
                m0.items.push(val);
            } else if m.yes == 7 {
                m7.items.push(val);
            }
        } else {
            if m.no == 1 {
                m1.items.push(val);
            } else if m.no == 2 {
                m2.items.push(val);
            } else if m.no == 3 {
                m3.items.push(val);
            } else if m.no == 4 {
                m4.items.push(val);
            } else if m.no == 5 {
                m5.items.push(val);
            } else if m.no == 0 {
                m0.items.push(val);
            } else if m.no == 7 {
                m7.items.push(val);
            }
        }
    }
    m.items = Vec::new(); 
}


fn action_m7(m0: &mut Monkey, 
    m1: &mut Monkey, 
    m2: &mut Monkey, 
    m3: &mut Monkey, 
    m4: &mut Monkey, 
    m5: &mut Monkey, 
    m6: &mut Monkey, 
    m7: &mut Monkey,
    star: i32) {
    let mut m = m7;
    
    let op: Vec<&str> = m.method.split_whitespace().collect();
    // loop through items
    for i in 0..m.items.len() {
        m.inspected += 1;
        let mut newi = 0.0_f32;
        if (op[0] == "old") && (op[1] == "*") {
            if op[2] == "old" {
                newi = (m.items[i] * m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] * known) as f32;
            }
        } else if (op[0] == "old") && (op[1] == "+") {
            if op[2] == "old" {
                newi = (m.items[i] + m.items[i]) as f32;
            } else {
                let known: i128 = op[2].parse().unwrap();
                newi = (m.items[i] + known) as f32;
            }
        }
        let mut borring = 0.0;
        if star == 1 {
            borring = newi / 3.0;
        } else if star == 2 {
            borring = newi / 1.0;
        }
        let val = borring.floor() as i128;
        if val % m.div == 0 {
            if m.yes == 1 {
                m1.items.push(val);
            } else if m.yes == 2 {
                m2.items.push(val);
            } else if m.yes == 3 {
                m3.items.push(val);
            } else if m.yes == 4 {
                m4.items.push(val);
            } else if m.yes == 5 {
                m5.items.push(val);
            } else if m.yes == 6 {
                m6.items.push(val);
            } else if m.yes == 0 {
                m0.items.push(val);
            }
        } else {
            if m.no == 1 {
                m1.items.push(val);
            } else if m.no == 2 {
                m2.items.push(val);
            } else if m.no == 3 {
                m3.items.push(val);
            } else if m.no == 4 {
                m4.items.push(val);
            } else if m.no == 5 {
                m5.items.push(val);
            } else if m.no == 6 {
                m6.items.push(val);
            } else if m.no == 0 {
                m0.items.push(val);
            }
        }
    }
    m.items = Vec::new(); 
}

fn primes_fac(m: &mut Monkey) {
    pfactors: Vec<i128> = Vec::new();
    for i in 0..m.items.len() {
        
    } 
}