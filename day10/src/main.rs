use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
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
    let matrix = read_file()?;

    let h = matrix.len() as i32;
    let w = matrix[0].len() as i32;
    let mut total = 0;
    for x in 0..h {
        for y in 0..w {
            if get!(&matrix, x, y) != 0 {
                continue;
            }

            let mut stack = vec![(x, y)];
            let mut visited = vec![vec![false; w as usize]; h as usize];
            while let Some((i, j)) = stack.pop() {
                get!(visited, i, j) = true;
                if get!(matrix, i, j) == 9 {
                    total += 1;
                }
                for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let ni = i + di;
                    let nj = j + dj;
                    if 0 <= ni
                        && ni < h
                        && 0 <= nj
                        && nj < w
                        && get!(matrix, ni, nj) - get!(matrix, i, j) == 1
                        && !get!(visited, ni, nj)
                    {
                        stack.push((ni, nj));
                    }
                }
            }
        }
    }

    println!("{total}");
    Ok(())
}


fn part02() -> Result<(), Box<dyn Error>> {
    let matrix = read_file()?;

    let h = matrix.len() as i32;
    let w = matrix[0].len() as i32;
    let mut total = 0;
    for x in 0..h {
        for y in 0..w {
            if get!(&matrix, x, y) != 0 {
                continue;
            }

            let mut stack = vec![(x, y)];
            while let Some((i, j)) = stack.pop() {
                if get!(matrix, i, j) == 9 {
                    total += 1;
                }
                for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let ni = i + di;
                    let nj = j + dj;
                    if 0 <= ni
                        && ni < h
                        && 0 <= nj
                        && nj < w
                        && get!(matrix, ni, nj) - get!(matrix, i, j) == 1
                    {
                        stack.push((ni, nj));
                    }
                }
            }
        }
    }

    println!("{total}");
    Ok(())
}

fn read_file() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut buf = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut buf)?;
    let matrix = buf
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|ch| ch as i32 - '0' as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(matrix)
}

#[macro_export]
macro_rules! get {
    ($m:expr, $x: expr, $y: expr) => {
        $m[$x as usize][$y as usize]
    };
}
