use std::{
    collections::HashMap, error::Error, fs::File, hash::Hash, io::{BufRead, BufReader}
};

fn main() {
    if let Err(e) = part01() {
        eprintln!("{}", e);
    }
    if let Err(e) = part02() {
        eprintln!("{}", e);
    }
}

fn part01() -> Result<(), Box<dyn Error>> {
    let (patterns, towels) = read_file()?;
    let count = towels
        .iter()
        .filter(|towel| check(towel.to_string(), &patterns))
        .count();
    println!("{}", count);
    Ok(())
}

fn part02() -> Result<(), Box<dyn Error>> {
    let (patterns, towels) = read_file()?;
    let mut memo = HashMap::new();
    let mut total = 0;
    for towel in towels {
        total += count(&mut memo, towel, &patterns);
    }
    println!("{}", total);
    Ok(())
}

fn check(towel: String, patterns: &Vec<String>) -> bool {
    if towel.is_empty() {
        return true;
    }

    patterns.iter().any(|pattern| {
        towel.starts_with(pattern) && check(towel[pattern.len()..].to_string(), patterns)
    })
}

fn count(memo: &mut HashMap<String, i128>, towel: String, patterns: &Vec<String>) -> i128 {
    if towel.is_empty() {
        return 1;
    }
    if let Some(v) = memo.get(&towel) {
        return *v;
    }

    let mut sum = 0_i128;
    for pattern in patterns {
        if towel.starts_with(pattern) {
            sum += count(memo, towel[pattern.len()..].to_string(), patterns);
        }
    }
    memo.insert(towel, sum);
    sum
}

fn read_file() -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("input.txt")?);

    let patterns = read_line(&mut reader)?
        .split(",")
        .map(|token| token.trim().to_string())
        .collect::<Vec<_>>();
    read_line(&mut reader)?;
    let mut towels = Vec::new();
    loop {
        let towel = read_line(&mut reader)?;
        if towel.is_empty() {
            break;
        }
        towels.push(towel);
    }

    Ok((patterns, towels))
}

fn read_line<T: BufRead>(reader: &mut T) -> Result<String, Box<dyn Error>> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}
