use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    if let Err(e) = part01() {
        eprintln!("{}", e);
    }
    if let Err(e) = part02() {
        eprintln!("{}", e);
    }
}

fn part01() -> Result<(), Box<dyn Error>> {
    let total = read_file()?
        .iter()
        .map(|machine| machine.calc_token1().unwrap_or(0))
        .sum::<i128>();
    println!("{}", total);
    Ok(())
}

fn part02() -> Result<(), Box<dyn Error>> {
    let total = read_file()?
        .iter_mut()
        .map(|machine| {
            machine.x += 10000000000000;
            machine.y += 10000000000000;
            machine.calc_token2().unwrap_or(0)
        })
        .sum::<i128>();
    println!("{}", total);
    Ok(())
}

fn read_file() -> Result<Vec<Machine>, Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("input.txt")?);
    let mut line1 = String::new();
    let mut line2 = String::new();
    let mut line3 = String::new();
    let mut line4 = String::new();
    let mut machines = Vec::new();

    while reader.read_line(&mut line1)? > 0 {
        reader.read_line(&mut line2)?;
        reader.read_line(&mut line3)?;
        reader.read_line(&mut line4)?;

        let (ax, ay) = parse(&line1)?;
        let (bx, by) = parse(&line2)?;
        let (x, y) = parse(&line3)?;
        machines.push(Machine {
            ax,
            ay,
            bx,
            by,
            x,
            y,
        });

        line1.clear();
        line2.clear();
        line3.clear();
        line4.clear();
    }
    Ok(machines)
}

fn parse(line: &String) -> Result<(i128, i128), Box<dyn Error>> {
    let tokens = Regex::new(r"[0-9]+")?
        .find_iter(&line)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();
    Ok((tokens[0].parse::<i128>()?, tokens[1].parse::<i128>()?))
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    ax: i128,
    ay: i128,
    bx: i128,
    by: i128,
    x: i128,
    y: i128,
}

impl Machine {
    fn calc_token1(self) -> Option<i128> {
        let mut min = std::i128::MAX;
        for a in 0.. {
            let x = self.x - a * self.ax;
            let y = self.y - a * self.ay;
            if x < 0 || y < 0 {
                break;
            }

            if x % self.bx == 0 && y % self.by == 0 && x / self.bx == y / self.by {
                let token = a * 3 + (x / self.bx);
                min = std::cmp::min(min, token);
            }
        }
        if min == std::i128::MAX {
            None
        } else {
            Some(min)
        }
    }

    fn calc_token2(self) -> Option<i128> {
        if let Some(b) = div(
            self.x * self.ay - self.y * self.ax,
            self.bx * self.ay - self.by * self.ax,
        ) {
            if let Some(a) = div(self.x - b * self.bx, self.ax) {
                return Some(a * 3 + b);
            }
        }
        None
    }
}

fn div(a: i128, b: i128) -> Option<i128> {
    if a % b == 0 {
        let d = a / b;
        if d >= 0 {
            Some(d)
        } else {
            None
        }
    } else {
        None
    }
}
