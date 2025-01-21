use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    if let Err(e) = part1() {
        eprintln!("{}", e);
    }
    if let Err(e) = part2() {
        eprintln!("{}", e);
    }
}

fn part1() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for t1 in read_file()? {
        let t2 = format!("A{}", t1)
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .map(|a| path("789456123_0A", a[0], a[1]))
            .collect::<String>();
        let t3 = format!("A{}", t2)
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .map(|a| path("_^A<v>", a[0], a[1]))
            .collect::<String>();
        let t4 = format!("A{}", t3)
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .map(|a| path("_^A<v>", a[0], a[1]))
            .collect::<String>();
        sum += t1[..3].parse::<usize>()? * t4.len();
    }
    println!("{}", sum);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;

    for t in read_file()? {
        let mut memo = HashMap::new();
        sum += t[..3].parse::<u128>()? * solve(&mut memo, t[..3].to_string(), 25);
    }
    println!("{}", sum);
    Ok(())
}

fn solve(memo: &mut HashMap<(String, i32), u128>, s: String, lvl: i32) -> u128 {
    if lvl < 0 {
        return (s.len() + 1) as u128;
    }

    let key = (s.clone(), lvl);
    if let Some(v) = memo.get(&key) {
        return *v;
    }

    let t1 = format!("A{}", s);
    let t2 = format!("{}A", s);
    let mut sum = 0;
    for (c1, c2) in t1.chars().zip(t2.chars()) {
        sum += solve(memo, path2(c1, c2), lvl - 1);
    }
    memo.insert(key, sum);
    sum
}

fn path2(c1: char, c2: char) -> String {
    let i1 = "789456123_0A<v>".chars().position(|c| c == c1).unwrap();
    let i2 = "789456123_0A<v>".chars().position(|c| c == c2).unwrap();
    let x1 = i1 % 3;
    let y1 = i1 / 3;
    let x2 = i2 % 3;
    let y2 = i2 / 3;

    let mut s = String::new();
    if x2 > x1 {
        s.push_str(&">".repeat(x2 - x1));
    }
    if y2 > y1 {
        s.push_str(&"v".repeat(y2 - y1));
    }
    if y1 > y2 {
        s.push_str(&"0".repeat(y1 - y2));
    }
    if x1 > x2 {
        s.push_str(&"<".repeat(x1 - x2));
    }
    if (y1, x2) == (3, 0) || (y2, x1) == (3, 0) {
        s
    } else {
        s.chars().rev().collect()
    }
}

fn read_file() -> Result<Vec<String>, Box<dyn Error>> {
    let mut buf = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut buf)?;
    Ok(buf.split("\n").map(|s| s.to_string()).collect())
}

fn path(m: &str, c1: char, c2: char) -> String {
    let i1 = m.chars().position(|c| c == c1).unwrap();
    let i2 = m.chars().position(|c| c == c2).unwrap();
    let x1 = i1 % 3;
    let y1 = i1 / 3;
    let x2 = i2 % 3;
    let y2 = i2 / 3;

    let mut s = String::new();
    if x2 > x1 {
        s.push_str(&">".repeat(x2 - x1));
    }
    if y2 > y1 {
        s.push_str(&"v".repeat(y2 - y1));
    }
    if y1 > y2 {
        s.push_str(&"^".repeat(y1 - y2));
    }
    if x1 > x2 {
        s.push_str(&"<".repeat(x1 - x2));
    }
    if (y1, x2) == (3, 0) || (y2, x1) == (3, 0) {
        format!("{}A", s)
    } else {
        format!("{}A", s.chars().rev().collect::<String>())
    }
}
