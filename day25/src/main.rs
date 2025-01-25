use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    if let Err(e) = part1() {
        eprintln!("{}", e);
    }
}

fn part1() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for s in text.split("\n\n") {
        let matrix = s
            .split("\n")
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        if s.starts_with("#") {
            locks.push(to_counts(&matrix));
        } else {
            keys.push(to_counts(&matrix));
        }
    }

    let mut ans = 0;
    for lock in &locks {
        for key in &keys {
            let n = lock.len();
            if (0..n).all(|i| lock[i] + key[i] <= 5) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
    Ok(())
}

fn to_counts(matrix: &Vec<Vec<char>>) -> Vec<i32> {
    let h = matrix.len();
    let w = matrix[0].len();

    let mut counts = Vec::new();
    for y in 0..w {
        let mut count = 0;
        for x in 0..h {
            if matrix[x][y] == '#' {
                count += 1;
            }
        }
        counts.push(count - 1);
    }
    counts
}
