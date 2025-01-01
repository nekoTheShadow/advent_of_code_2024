use std::{collections::HashMap, error::Error, fs::File, io::{BufRead, BufReader}};

fn main() {
    if let Err(e) = part01() {
        eprintln!("{}", e);
    }
    if let Err(e) = part02() {
        eprintln!("{}", e);
    }
}

fn part01() -> Result<(), Box<dyn Error>> {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for line_result in BufReader::new(File::open("input.txt")?).lines() {
        let line = line_result?;
        let tokens = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
        xs.push(tokens[0].parse::<i32>()?);
        ys.push(tokens[1].parse::<i32>()?);
    }
    xs.sort();
    ys.sort();
    
    let total = xs.iter().zip(ys).map(|(x, y)| (x - y).abs()).sum::<i32>();
    println!("{total}");
    Ok(())
}

fn part02() -> Result<(), Box<dyn Error>> {
    let mut xs = Vec::new();
    let mut counter = HashMap::new();
    for line_result in BufReader::new(File::open("input.txt")?).lines() {
        let line = line_result?;
        let tokens = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
        let x = tokens[0].parse::<i32>()?;
        let y = tokens[1].parse::<i32>()?;

        xs.push(x);
        *counter.entry(y).or_insert(0) += 1;
    }

    let total = xs.iter().map(|x| counter.get(x).unwrap_or(&0) * x).sum::<i32>();
    println!("{total}");
    Ok(())
}