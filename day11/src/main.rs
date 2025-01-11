use std::{collections::HashMap, error::Error, fs::File, io::{BufReader, Read}};

fn main() {
    if let Err(e) = part01() {
        eprintln!("{:?}", e);
    }
    if let Err(e) = part02() {
        eprintln!("{:?}", e);
    }
}

fn part01() -> Result<(), Box<dyn Error>> {
    let mut stones = read_file()?;
    for _ in 0..25 {
        let mut next_stones = Vec::new();
        for &stone in &stones {
            if stone == 0 {
                next_stones.push(1);
            } else {
                let xs = digits(stone);
                let n = xs.len();
                if n % 2 == 0 {
                    next_stones.push(to_u128(&xs[..n/2]));
                    next_stones.push(to_u128(&xs[n/2..]));
                } else {
                    next_stones.push(stone * 2024);
                }
            }
        }
        stones = next_stones;
    }
    println!("{}", stones.len());
    Ok(())
}

fn part02() -> Result<(), Box<dyn Error>> {
    let stones = read_file()?;
    let mut total = 0;
    let mut memo = HashMap::new();
    for stone in stones {
        total += count(stone, 75, &mut memo);
    }
    println!("{}", total);
    Ok(())
}

fn count(stone: u128, time: u128, memo: &mut HashMap<(u128, u128), u128>) -> u128 {
    if time == 0 {
        return 1;
    }
    if let Some(v) = memo.get(&(stone, time)) {
        return *v;
    }

    let v = if stone == 0 {
        count(1, time - 1, memo)
    } else {
        let xs = digits(stone);
        let n = xs.len();
        if n % 2 == 0 {
            count(to_u128(&xs[..n/2]), time - 1, memo) + count(to_u128(&xs[n/2..]), time -1, memo)
        } else {
            count(stone * 2024, time - 1,memo)
        }
    };
    memo.insert((stone, time), v);
    v
}


fn read_file() -> Result<Vec<u128>, Box<dyn Error>> {
    let mut buf = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut buf)?;

    let mut nums = Vec::new();
    for s in buf.trim().split_whitespace() {
        nums.push(s.parse::<u128>()?);
    }
    Ok(nums)
}

fn digits(stone: u128) -> Vec<u128> {
    let mut xs = Vec::new();
    let mut x = stone;
    while x > 0 {
        xs.push(x % 10);
        x /= 10;
    }
    xs.reverse();
    xs
}

fn to_u128(xs: &[u128]) -> u128 {
    xs.iter().fold(0, |acc, x| acc * 10 + x)
}