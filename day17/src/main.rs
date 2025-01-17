use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    u128,
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
    let (a, b, c, program) = read_file()?;
    let out = run(&program, a)?;
    println!(
        "{}",
        out.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
    Ok(())
}

fn part02() -> Result<(), Box<dyn Error>> {
    let (a, b, c, program) = read_file()?;
    let x = find_a(&program, 0, 0)?;
    println!("{}", x);
    Ok(())
}

fn find_a(program: &Vec<u128>, a: u128, i: usize) -> Result<u128, Box<dyn Error>> {
    let n = program.len();
    if i == n {
        return Ok(a);
    } else {
        for x in 0..8 {
            let out1 = run(program, a * 8 + x)?;
            if out1[0] == program[n - i - 1] {
                let out2 = find_a(program, a * 8 + x, i + 1)?;
                if out2 != u128::MAX {
                    return Ok(out2);
                }
            }
        }
    }
    Ok(u128::MAX)
}

fn run(program: &Vec<u128>, a: u128) -> Result<Vec<u128>, Box<dyn Error>> {
    let mut a = a;
    let mut b = 0;
    let mut c = 0;
    let mut out = Vec::new();
    let mut ip = 0;
    while ip < program.len() {
        let opecode = program[ip];
        let operand = program[ip + 1];

        match opecode {
            0 => {
                a = a / pow(2, combo(operand, a, b, c));
            }
            1 => {
                b = b ^ operand;
            }
            2 => {
                b = combo(operand, a, b, c) % 8;
            }
            3 => {
                if a != 0 {
                    ip = combo(operand, a, b, c) as usize;
                    continue;
                }
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                out.push(combo(operand, a, b, c) % 8);
            }
            6 => {
                b = a / pow(2, combo(operand, a, b, c));
            }
            7 => {
                c = a / pow(2, combo(operand, a, b, c));
            }
            _ => {
                unreachable!()
            }
        }

        ip += 2;
    }
    Ok(out)
}

fn read_file() -> Result<(u128, u128, u128, Vec<u128>), Box<dyn Error>> {
    let mut buf = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut buf)?;
    let lines = buf.split("\n").collect::<Vec<_>>();

    let a = parse_digits(lines[0])?[0];
    let b = parse_digits(lines[1])?[0];
    let c = parse_digits(lines[2])?[0];
    let program = parse_digits(lines[4])?;
    Ok((a, b, c, program))
}

fn parse_digits(line: &str) -> Result<Vec<u128>, Box<dyn Error>> {
    let mut digits = Vec::new();
    if let Some(x) = line.find(':') {
        for token in line[x + 2..].split(",") {
            digits.push(token.parse()?);
        }
    }
    Ok(digits)
}

fn combo(operand: u128, a: u128, b: u128, c: u128) -> u128 {
    if operand == 4 {
        return a;
    }
    if operand == 5 {
        return b;
    }
    if operand == 6 {
        return c;
    }
    operand
}

fn pow(x: u128, y: u128) -> u128 {
    if y == 0 {
        return 1;
    }
    if y % 2 == 0 {
        pow(x * x, y / 2)
    } else {
        pow(x, y - 1) * x
    }
}
