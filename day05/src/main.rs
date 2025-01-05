use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    if let Err(e) = part01() {
        eprintln!("{e}")
    }
    if let Err(e) = part02() {
        eprintln!("{e}")
    }
}

fn part01() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("input.txt")?);

    let mut rules = HashSet::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if let Some((a, b)) = line.trim().split_once("|") {
            rules.insert((a.to_string(), b.to_string()));
        } else {
            break;
        }
    }

    let mut total = 0;
    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        let pages = line.trim().split(",").collect::<Vec<_>>();
        if is_correct(&rules, &pages) {
            total += pages[pages.len() / 2].parse::<i32>()?;
        }
    }

    println!("{total}");
    Ok(())
}

fn is_correct(rules: &HashSet<(String, String)>, pages: &[&str]) -> bool {
    for i in 0..pages.len() {
        for j in i + 1..pages.len() {
            let a = pages[i].to_string();
            let b = pages[j].to_string();
            if !rules.contains(&(a, b)) {
                return false;
            }
        }
    }
    true
}

// ==================================

fn part02() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("input.txt")?);

    let mut rules = HashSet::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if let Some((a, b)) = line.trim().split_once("|") {
            rules.insert((a.to_string(), b.to_string()));
        } else {
            break;
        }
    }

    let mut total = 0;
    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        let pages = line.trim().split(",").collect::<Vec<_>>();
        if is_correct(&rules, &pages) {
            continue;
        }

        total += tsort(&rules, &pages).parse::<i32>()?;
    }

    println!("{total}");
    Ok(())
}

fn tsort(rules: &HashSet<(String, String)>, pages: &[&str]) -> String {
    let n = pages.len();
    let mut count = vec![0; n];
    let mut g = vec![vec![]; n];
    for i in 0..n {
        for j in i + 1..n {
            let a = pages[i].to_string();
            let b = pages[j].to_string();
            if rules.contains(&(a, b)) {
                count[j] += 1;
                g[i].push(j);
            } else {
                count[i] += 1;
                g[j].push(i);
            }
        }
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if count[i] == 0 {
            q.push_back(i);
        }
    }

    let mut indexes = Vec::new();
    while let Some(i) = q.pop_front() {
        indexes.push(i);
        for &j in &g[i] {
            count[j] -= 1;
            if count[j] == 0 {
                q.push_back(j);
            }
        }
    }
    pages[indexes[n / 2]].to_string()
}
