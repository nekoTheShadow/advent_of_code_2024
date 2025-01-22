use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
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
    let numbers = read_file()?;
    let mut memo = HashMap::new();
    let mut sum = 0;
    for number in numbers {
        sum += calc_number(&mut memo, number, 2000);
    }
    println!("{}", sum);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let numbers = read_file()?;
    let mut map = HashMap::new();

    for number in numbers {
        let mut x = number;
        let mut xs = Vec::new();
        for _ in 0..2001 {
            xs.push(x % 10);
            x = next_number(x);
        }

        let mut visited = HashSet::new();
        for v in xs.windows(5) {
            let pat = (v[1] - v[0], v[2] - v[1], v[3] - v[2], v[4] - v[3]);
            if visited.contains(&pat) {
                continue;
            }
            visited.insert(pat);
            *map.entry(pat).or_insert(0) += v[4];
        }
    }

    println!("{}", map.values().max().unwrap());

    Ok(())
}

fn read_file() -> Result<Vec<i128>, Box<dyn Error>> {
    let mut numbers = Vec::new();
    for line_result in BufReader::new(File::open("input.txt")?).lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}

fn calc_number(memo: &mut HashMap<(i128, i128), i128>, x: i128, time: i128) -> i128 {
    if time == 0 {
        return x;
    }
    if let Some(v) = memo.get(&(x, time)) {
        return *v;
    }
    let v = calc_number(memo, next_number(x), time - 1);
    memo.insert((x, time), v);
    v
}

fn next_number(x: i128) -> i128 {
    let x = ((x * 64) ^ x) % 16777216;
    let x = ((x / 32) ^ x) % 16777216;
    let x = ((x * 2048) ^ x) % 16777216;
    x
}
