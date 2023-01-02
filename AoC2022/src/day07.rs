use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    // read input
    let dir_elements = read_input("../consol.txt");
    let dircopy = dir_elements.clone();
    // get a list of memory sizes
    let memsize = mem_size(dircopy);
    let zer_s = memsize.clone();
    let zer_new = memsize.clone();
    let dircopy2 = dir_elements.clone();
    // get a list of addresses
    let mempath = mem_path(dircopy2);
    let mem2 = mempath.clone();
    let zer_ss = memsize.clone();
    let zer_sss = memsize.clone();
    let zer_pp = mempath.clone();
    let zer_ppp = mempath.clone();
    // move out one dir to create a new list
    let (outer_p, outer_s) = mem_layout(mem2, memsize);
    let fir_s = outer_s.clone();
    let fir_pp = outer_p.clone();
    let fir_ppp = outer_p.clone();
    let fir_ss = outer_s.clone();
    let fir_sss = outer_s.clone();
    
    let mut cout: Vec<String> = Vec::new();
    for i in 0..outer_p.len() {
        cout.push(outer_p[i].to_string());
    }
    let (sec_p, sec_s) = mem_layout(cout, outer_s);
    let sec_ss = sec_s.clone();
    let sec_sss = sec_s.clone();
    let sec_pp = sec_p.clone();
    let sec_ppp = sec_p.clone();
    let (thr_p, thr_s) = mem_layout(sec_p, sec_s);
    let thr_ss = thr_s.clone();
    let thr_sss = thr_s.clone();
    let thr_ppp = thr_p.clone();
    let thr_pp = thr_p.clone();
    let (fou_p, fou_s) = mem_layout(thr_p, thr_s);
    let fou_ss = fou_s.clone();
    let fou_pp = fou_p.clone();
    let (fiv_p, fiv_s) = mem_layout(fou_p, fou_s);
    let fiv_ss = fiv_s.clone();
    let fiv_pp = fiv_p.clone();
    let (six_p, six_s) = mem_layout(fiv_p, fiv_s);
    let six_ss = six_s.clone();
    let six_pp = six_p.clone();
    let (sev_p, sev_s) = mem_layout(six_p, six_s);
    println!("{:?} {}", sev_p, sev_s[0]);
    let mut dir_large: Vec<String> = Vec::new();
    let mut dir_small: Vec<String> = Vec::new();
    let mut sec_star: Vec<String> = Vec::new();
    let mut sec_star_s: Vec<i32> = Vec::new();
    let total_mem: i32 = sev_s[0];
    let maxsize: i32 = 70000000;
    let freeneed: i32 = 30000000;
    let tobefree: i32 = freeneed - (maxsize - total_mem);
    println!("This amount needs to be removed {}", tobefree);
    let toohigh: i32 = 567821;
    let toolow: i32 = 534784;
    for i in 0..fiv_pp.len() {
        dir_large.push(fiv_pp[i].to_string());
        if fiv_ss[i] > tobefree {
            // println!("{} {}", fiv_s[i], fiv_p[i]);
            let string = format!("{} {}" ,fiv_ss[i], fiv_pp[i]);
            sec_star_s.push(fiv_ss[i]);
            sec_star.push(string);
        };
    }

    for i in 0..fou_pp.len() {
        let mut count:i32 = 0;
        if fou_ss[i] > tobefree {
            // println!("{} {}", fiv_s[i], fiv_p[i]);
            let string = format!("{} {}" ,fou_ss[i], fou_pp[i]);
            sec_star_s.push(fou_ss[i]);
            sec_star.push(string);
        };
        for j in 0..dir_large.len() {
            if fou_pp[i] == dir_large[j] {
                count += 1;
            }
        }
        if (fou_ss[i] >= 100000) && (count == 0) {
            dir_large.push(fou_pp[i].to_string());
        } else if fou_ss[i] < 100000 {
            let string = format!("{} {}", fou_ss[i], fou_pp[i]);
            dir_small.push(string);
        };
    }
    
    for i in 0..thr_pp.len() {
        let mut count:i32 = 0;
        if thr_ss[i] > tobefree {
            // println!("{} {}", fiv_s[i], fiv_p[i]);
            let string = format!("{} {}" ,thr_ss[i], thr_pp[i]);
            sec_star_s.push(thr_ss[i]);
            sec_star.push(string);
        };
        for j in 0..dir_large.len() {
            if thr_pp[i] == dir_large[j] {
                count += 1;
            }
        }
        if (thr_ss[i] >= 100000) && (count == 0) {
            dir_large.push(thr_pp[i].to_string());
        } else if thr_ss[i] < 100000 {
            let string = format!("{} {}", thr_ss[i], thr_pp[i]);
            dir_small.push(string);
        };
    }

    for i in 0..sec_pp.len() {
        let mut count:i32 = 0;
        if sec_ss[i] >= tobefree {
            // println!("{} {}", fiv_s[i], fiv_p[i]);
            let string = format!("{} {}" ,sec_ss[i], sec_pp[i]);
            sec_star_s.push(sec_ss[i]);
            sec_star.push(string);
        };
        for j in 0..dir_large.len() {
            if sec_pp[i] == dir_large[j] {
                count += 1;
            }
        }
        if (sec_ss[i] >= 100000) && (count == 0) {
            dir_large.push(sec_pp[i].to_string());
            
        } else if sec_ss[i] < 100000 {
            let string = format!("{} {}", sec_ss[i], sec_pp[i]);
            dir_small.push(string);
        };
    }

    for i in 0..fir_pp.len() {
        let mut count:i32 = 0;
        if fir_ss[i] >= tobefree {
            // println!("{} {}", fiv_s[i], fiv_p[i]);
            let string = format!("{} {}" ,fir_ss[i], fir_pp[i]);
            sec_star_s.push(fir_ss[i]);
            sec_star.push(string);
        };
        for j in 0..dir_large.len() {
            if fir_pp[i] == dir_large[j] {
                count += 1;
            }
        }
        if (fir_ss[i] >= 100000) && (count == 0) {
            dir_large.push(fir_pp[i].to_string());
            
        } else if fir_ss[i] < 100000 {
            let string = format!("{} {}", fir_ss[i], fir_pp[i]);
            dir_small.push(string);
        };
    }

    for i in 0..zer_pp.len() {
        let mut count:i32 = 0;
        if zer_ss[i] > tobefree {
            // println!("{} {}", fiv_s[i], fiv_p[i]);
            let string = format!("{} {}" ,zer_ss[i], zer_pp[i]);
            sec_star_s.push(zer_ss[i]);
            sec_star.push(string);
        };
        for j in 0..dir_large.len() {
            if zer_pp[i] == dir_large[j] {
                count += 1;
            }
        }
        if (zer_ss[i] >= 100000) && (count == 0) {
            dir_large.push(zer_pp[i].to_string());
            
        } else if zer_ss[i] < 100000 {
            let string = format!("{} {}", zer_ss[i], zer_pp[i]);
            dir_small.push(string);
        };
    }

    // for i in 0..sec_star.len() {
    //     println!("This folders {}", sec_star[i]);
    // }

    let mut small: Vec<String> = Vec::new();
    let mut small_s: Vec<i32> = Vec::new();
    for i in 0..dir_small.len() {
        let output: Vec<&str> = dir_small[i].split(&[' '][..]).collect();
        let mut count: i32 = 0;
        for j in 0..dir_large.len() {
            if output[1] == dir_large[j] {
                count += 1;
            }
        };
        if count == 0 {
            small.push(output[1].to_string());
            small_s.push(output[0].parse().unwrap());
        };
    }
    let sp = small.clone();
    let sp2 = small.clone();
    let ssize = small_s.clone();

    let small_mems = mem_combine(sp, ssize);
    let mut finvec: Vec<String> = Vec::new();
    for i in 0..small_mems.len() {
        let string = format!("{} {}", small_mems[i], sp2[i]);
        finvec.push(string);
    }
    let (f_p, f_s) = mem_clean(finvec);
    let mut sumhere: i32 = 0;
    for i in 0..f_p.len() {
        if f_s[i] < 100000 {
            // println!("{} {}", f_s[i], f_p[i]);
            sumhere += f_s[i];
        }
    }
    println!("Yet another sum {}", sumhere);

    
    // println!("Total mem used: {}", total_mem);
    // println!("This is {} free mem", maxsize - total_mem);
    
    println!("Zero");

    let better_0 = mem_combine(zer_ppp, zer_sss);
    for i in 0..zer_ss.len() {
        if (better_0[i] < toohigh) && (better_0[i] > 0) {
            println!("{} {}", better_0[i], zer_pp[i]);
        } 
    }
    println!("First");
    let better_1 = mem_combine(fir_ppp, fir_sss);
    for i in 0..better_1.len() {
        if (better_1[i] < toohigh) && (better_1[i] > 0)  {
            println!("{} {}", better_1[i], fir_pp[i]);
        } 
    }
    // println!("Second");
    // let better_2 = mem_combine(sec_ppp, sec_sss);
    // for i in 0..better_2.len() {
    //     if (better_2[i] < toohigh) && (better_2[i] > 0) {
    //         println!("{} {}", better_2[i], sec_pp[i]);
    //     } 
    // }
    // println!("Third");
    // let better_3 = mem_combine(thr_ppp, thr_sss);
    // for i in 0..better_3.len() {
    //     if (better_3[i] < toohigh) && (better_3[i] > 0) {
    //         println!("{} {}", better_3[i], thr_pp[i]);
    //     } 
    // }
    // println!("Fourth");

    // for i in 0..fou_ss.len() {
    //     if fou_ss[i] < toohigh {
    //         println!("{} {}", fou_ss[i], fou_pp[i]);
    //     } 
    // }
    // println!("Fifth");

    // for i in 0..fiv_ss.len() {
    //     if fiv_ss[i] < toohigh {
    //         println!("{} {}", fiv_ss[i], fiv_pp[i]);
    //     } 
    // }
    // println!("Sixth");

    // for i in 0..six_ss.len() {
    //     if six_ss[i] < toohigh {
    //         println!("{} {}", six_ss[i], six_pp[i]);
    //     } 
    // }
    // sec_star_s.sort();
    // println!("{:?}", sec_star_s);
    
    Ok(())
}

fn read_input(file: &str) -> Vec<String> {
    let mut f = File::open(file).unwrap();
    let mut data = String::new();
    // Switch between first and second star
    let mut dir_elements: Vec<String> = Vec::new();
    let mut cd_list: Vec<String> = Vec::new();
    f.read_to_string(&mut data).unwrap();
    // println!("{}", data);
    let t1: Vec<&str> = data.split(&['\n'][..]).collect();
    let mut dirname = String::from("/");
    for i in 0..t1.len() {
        // println!("{} {}", i, t1[i]);
        let t2: Vec<&str> = t1[i].split(&[' '][..]).collect();
        if t2.len() == 3 && i > 0 {
            if t2[2].replace("\r", "") == ".." {
                let range = dirname.rfind(":").unwrap();
                dirname = dirname[0..range].to_string();
            } else {
                dirname = format!("{}:{}", dirname, t2[2].replace("\r", ""));
            };
        }; 
        if (t2[0] == "$") && (t2[1] == "ls\r")  {
            // println!("ls called");
            let mut j = i + 1;
            while !(t1[j].chars().nth(0).unwrap() == '$') {
                let together = format!("{} {}", t1[j].replace("\r", ""), dirname);
                dir_elements.push(together);
                if j < t1.len()-1 {
                    j += 1;
                } else {
                    break;
                };
            }
        } else if (t2[0] == "$") && (t2[1] == "cd") {
            // println!("cd called");
            cd_list.push(t2[2].replace("\r", ""));
        }
    }
    dir_elements
}

fn mem_size(dirs: Vec<String>) -> Vec<i32> {
    let mut memsize: Vec<i32> = Vec::new();
    for i in 0..dirs.len() {
        let output: Vec<&str> = dirs[i].split(&[' '][..]).collect();
        if output[0] == "dir" {

        } else {
            let filesize: i32 = output[0].parse().unwrap();
            memsize.push(filesize);
        };
    }
    // println!("{:?}", memsize);
    memsize
}

fn mem_path(dirs: Vec<String>) -> Vec<String> {
    let mut mempath: Vec<String> = Vec::new();
    for i in 0..dirs.len() {
        let output: Vec<&str> = dirs[i].split(&[' '][..]).collect();
        if output[0] == "dir" {

        } else {
            mempath.push(format!("{}", output[2]));
        };
    }
    // println!("{:?}", mempath);

    mempath
}

fn mem_combine(mems: Vec<String>, memval: Vec<i32>) -> Vec<i32> {
    let mut memcomb = memval.clone();
    for i in 0..mems.len() {
        // let mut value: i32 = 0;
        for j in i+1..mems.len() {
            if mems[i] == mems[j] {
                memcomb[i] += memcomb[j];
                memcomb[j] = 0;
            }
        }
    }
    // println!("{:?}", memcomb);

    memcomb
}

fn mem_clean(mems: Vec<String>) -> (Vec<String>, Vec<i32>) {
    let mut mem_v: Vec<i32> = Vec::new();
    let mut mem_s: Vec<String> = Vec::new();
    for i in 0..mems.len() {
        let mem_i: Vec<&str> = mems[i].split(&[' '][..]).collect();
        if mem_i[0] == "0" {

        } else {
            let filesize: i32 = mem_i[0].parse().unwrap();
            mem_v.push(filesize);
            mem_s.push(mem_i[1].to_string());
        };
    }
    // println!("{:?}", mem_v);
    (mem_s, mem_v)
}

// This does what it should do
fn move_out(mempath: Vec<String>) -> Vec<String> {
    let mut fp: Vec<String> = Vec::new();
    for i in 0..mempath.len() {
        let mut string = mempath[i].to_string();
        if string.contains(":") {
            let new_str = mempath[i].to_string();
            let range = new_str.rfind(":").unwrap();
            string = new_str[0..range].to_string();
            // println!("Old string {}\nNew string {}", new_str, string);
            fp.push(string);
            
        } else {
            fp.push(mempath[i].to_string());
            // println!("Did not expect so many yes {}", i);
        };
    }
    // println!("{:?}", fp);
    fp
}

fn mem_layout(memp: Vec<String>, mems: Vec<i32>) -> (Vec<String>, Vec<i32>) {
    let outer = memp.clone();
    let memout = move_out(memp);
    let mem2 = memout.clone();
    let mz3 = mems.clone();
    // sum up memory in one dir
    let dir_1 = mem_combine(memout, mz3);
    // create a string with both memory and address
    let mut layer: Vec<String> = Vec::new();
    for i in 0..dir_1.len() {
        let string = format!("{} {}", dir_1[i], mem2[i]);
        layer.push(string);
    }
    let (outer_p, outer_s) = mem_clean(layer);
    (outer_p, outer_s)
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
  // empty vector input case
  if strs.len() == 0 {
    return String::new();
  } else if strs.len() == 1 {
    // only one String in vector
    return strs[0].clone();
  }

  // Pick an arbitrary word, I chose the first word
  let base = &strs[0];

  // declare tracking vars to walk through the vector and track best match
  let mut sub_string = String::new();
  let mut best_match = String::new();

  for char in base.chars() {
    // grow the sub_string one character at a time
    sub_string.push(char);

    // look for the sub_string in all the other words, skipping the first word
    for word in &strs[1..] {
      if word.contains(&sub_string) {
        // if you find the substring in the word
        // if substring is longer than current best match
        if sub_string.len() > best_match.len() {
          // update best match
          best_match = String::from(&sub_string);
        }
      } else {
        // found a substring that is not in all the words
        if sub_string.len() == best_match.len() && sub_string.contains(&best_match) {
          best_match.pop();
        }
        // clear out the substring
        sub_string.clear();
        break;
      }
    }
  }
  // return the best found substring
  best_match
}