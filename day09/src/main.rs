use std::{collections::VecDeque, error::Error, fs::File, io::{BufReader, Read}};

fn main() {
    if let Err(e) = part01() {
        eprintln!("{}", e);
    }
    if let Err(e) = part02() {
        eprintln!("{}", e);
    }
}

fn part01() -> Result<(), Box<dyn Error>> {
    let mut dq = VecDeque::new();
    for (i, space) in read_file()?.iter().enumerate() {
        if i % 2 == 0 {
            dq.push_back(((i / 2) as i128, *space));
        } else {
            dq.push_back((-1, *space));
        }
    }

    let mut disk = Vec::new();
    while let Some((f1, c1)) = dq.pop_front() {
        if f1 == -1 {
            let mut c = c1;
            while c > 0 {
                let (f2, c2) = dq.pop_back().unwrap();
                if f2 == -1 {
                    continue;
                }

                if c2 <= c {
                    disk.push((f2, c2));
                    c -= c2;
                } else {
                    disk.push((f2, c));
                    dq.push_back((f2, c2 - c));
                    c = 0;
                }
            }
            
        } else {
            disk.push((f1, c1));
        }
    }

    let mut sum = 0_i128;
    let mut i = 0;
    for (f, c) in disk {
        for j in 0..c {
            sum += (i + j) * f;
        }
        i += c;
    }
    println!("{:?}", sum);
    Ok(())
}

fn part02() -> Result<(), Box<dyn Error>> {
    let mut stack = Vec::new();
    for (i, space) in read_file()?.iter().enumerate() {
        if i % 2 == 0 {
            stack.push(((i / 2) as i128, *space, false));
        } else {
            stack.push((-1, *space, false));
        }
    }

    let mut disk = Vec::new();
    while let Some((fileid, space, moved)) = stack.pop() {
        if fileid == -1 {
            disk.push((fileid, space));
        } else {
            if moved {
                disk.push((fileid, space));
            } else {
                if let Some(i) = stack.iter().position(|(f, s, _)| *f == -1 && space <= *s) {
                    let (_, s, _) = stack[i];
                    if space == s {
                        stack[i] = (fileid, space, true);
                        disk.push((-1, space));
                    } else {
                        stack[i] = (fileid, space, true);
                        stack.insert(i+1, (-1, s - space, false));
                        disk.push((-1, space));
                    }
                } else {
                    disk.push((fileid, space));
                }
            }
        }
    }
    
    disk.reverse();
    let mut sum = 0_i128;
    let mut i = 0;
    for (f, c) in disk {
        if f != -1 {
            for j in 0..c {
                sum += (i + j) * f;
            }
        }
        i += c;
    }
    println!("{:?}", sum);
    

    Ok(())
}

fn read_file() -> Result<Vec<i128>, Box<dyn Error>> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;
    Ok(text.trim().chars().map(|ch| ch as i128 - '0' as i128).collect())
}