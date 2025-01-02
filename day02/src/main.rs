use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    if let Err(e) = part01() {
        eprintln!("{e}");
    }
    if let Err(e) = part02() {
        eprintln!("{e}");
    }
}

fn part01() -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    for line in BufReader::new(File::open("input.txt")?).lines() {
        let mut xs = Vec::new();
        for token in line?.split_ascii_whitespace() {
            xs.push(token.parse::<i32>()?);
        }

        if (0..xs.len() - 1).all(|i| 1 <= xs[i] - xs[i + 1] && xs[i] - xs[i + 1] <= 3)
            || (0..xs.len() - 1).all(|i| 1 <= xs[i + 1] - xs[i] && xs[i + 1] - xs[i] <= 3)
        {
            count += 1;
        }
    }
    println!("{count}");
    Ok(())
}

fn part02() -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    for line in BufReader::new(File::open("input.txt")?).lines() {
        let mut xs = Vec::new();
        for token in line?.split_ascii_whitespace() {
            xs.push(token.parse::<i32>()?);
        }

        if check(&xs) {
            count += 1;
            continue;
        }
        xs.reverse();
        if check(&xs) {
            count += 1;
            continue;
        }
    }
    println!("{count}");
    Ok(())
}

fn check(xs: &[i32]) -> bool {
    if xs.windows(2).all(|a| 1 <= a[0] - a[1] && a[0] - a[1] <= 3) {
        return true;
    }
    if xs[1..].windows(2).all(|a| 1 <= a[0] - a[1] && a[0] - a[1] <= 3) {
        return true;
    }

    let mut stack = Vec::new();
    for &x in xs {
        if let Some(&y) = stack.last() {
            if 1 <= y - x && y - x <= 3 {
                stack.push(x);
            }
        } else {
            stack.push(x);
        }
    }
    stack.len() == xs.len() - 1
}
