use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read},
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
    let m = read_matrix()?;
    let h = m.len();
    let w = m[0].len();
    let mut c = 0;
    for x in 0..h {
        for y in 0..w {
            if x + 3 < h && is_xmas(m[x][y], m[x + 1][y], m[x + 2][y], m[x + 3][y]) {
                c += 1;
            }
            if y + 3 < w && is_xmas(m[x][y], m[x][y + 1], m[x][y + 2], m[x][y + 3]) {
                c += 1;
            }
            if x + 3 < h
                && y + 3 < w
                && is_xmas(m[x][y], m[x + 1][y + 1], m[x + 2][y + 2], m[x + 3][y + 3])
            {
                c += 1;
            }
            if x + 3 < h
                && y >= 3
                && is_xmas(m[x][y], m[x + 1][y - 1], m[x + 2][y - 2], m[x + 3][y - 3])
            {
                c += 1;
            }
        }
    }
    println!("{c}");
    Ok(())
}

fn part02() -> Result<(), Box<dyn Error>> {
    let m = read_matrix()?;
    let h = m.len();
    let w = m[0].len();
    let mut c = 0;
    for x in 0..h {
        for y in 0..w {
            if 1 <= x && x < h - 1 && 1 <= y && y < w - 1 {
                let c1 = m[x - 1][y - 1];
                let c2 = m[x][y];
                let c3 = m[x + 1][y + 1];

                let d1 = m[x + 1][y - 1];
                let d2 = m[x][y];
                let d3 = m[x - 1][y + 1];

                if ((c1, c2, c3) == ('M', 'A', 'S') || (c1, c2, c3) == ('S', 'A', 'M'))
                    && ((d1, d2, d3) == ('M', 'A', 'S') || (d1, d2, d3) == ('S', 'A', 'M'))
                {
                    c += 1;
                }
            }
        }
    }

    println!("{c}");
    Ok(())
}

fn is_xmas(c1: char, c2: char, c3: char, c4: char) -> bool {
    (c1, c2, c3, c4) == ('X', 'M', 'A', 'S') || (c1, c2, c3, c4) == ('S', 'A', 'M', 'X')
}

fn read_matrix() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;
    let m = text
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok(m)
}
