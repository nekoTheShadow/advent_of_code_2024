use std::{error::Error, fs::File, io::{BufRead, BufReader, Read}};

use regex::Regex;

fn main() {
    if let Err(e) = part1() {
        eprintln!("{e}");
    }
    if let Err(e) = part2() {
        eprintln!("{e}");
    }
}

fn part1() -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;
    let mut total = 0;
    for line in BufReader::new(File::open("input.txt")?).lines() {
        for m in re.captures_iter(&line?) {
            let a = m[1].parse::<isize>()?;
            let b = m[2].parse::<isize>()?;
            total += a * b;
        }
    }
    println!("{total}");
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;
    
    let mut instructions = Vec::new();
    for cap in Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?.captures_iter(&text) {
        let a = cap[1].parse::<isize>()?;
        let b = cap[2].parse::<isize>()?;
        instructions.push(Instruction::Mul(cap.get(0).unwrap().start(), a, b));
    }
    for mat in Regex::new(r"do\(\)")?.find_iter(&text) {
        instructions.push(Instruction::Do(mat.start()));
    }
    for mat in Regex::new(r"don't\(\)")?.find_iter(&text) {
        instructions.push(Instruction::Dont(mat.start()));
    }

    instructions.sort_by_key(|instruction| instruction.start());

    let mut total = 0;
    let mut enabled = true;
    for instruction in instructions {
        match instruction {
            Instruction::Mul(_, a, b) => {
                if enabled {
                    total += a * b;
                }
            },
            Instruction::Do(_) => enabled = true,
            Instruction::Dont(_) => enabled = false,
        }
    }
    println!("{total}");
    Ok(())
}


#[derive(Debug)]
enum Instruction  {
    Mul(usize, isize, isize),
    Do(usize),
    Dont(usize)
}

impl Instruction {
    fn start(&self) -> usize {
        match self {
            Instruction::Mul(start, _, _) => *start,
            Instruction::Do(start) => *start,
            Instruction::Dont(start) => *start,
        }
    }
}