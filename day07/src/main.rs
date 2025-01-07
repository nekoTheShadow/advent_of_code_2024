use std::{error::Error, fs::File, io::{BufRead, BufReader}};

fn main() {
    if let Err(e) = part01() {
        eprintln!("{}", e);
    }
    if let Err(e) = part02() {
        eprintln!("{}", e);
    }
}

fn part01() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for line in BufReader::new(File::open("input.txt")?).lines() {
        if let Some((left, right)) = line?.split_once(":") {
            let total = left.parse::<i128>()?;
            let mut nums = Vec::new();
            for num in right.split_whitespace() {
                nums.push(num.parse::<i128>()?);
            }

            if is_correct1(total, &nums) {
                sum += total;
            }
        }
    }
    println!("{}", sum);


    Ok(())
}

fn is_correct1(total: i128, nums: &[i128]) -> bool {
    let n = nums.len()-1;
    for bit in 0..1<<n {
        let mut sum = nums[0];
        for i in 0..n {
            if (bit >> i) & 1 == 0 {
                sum += nums[i+1];
            } else {
                sum *= nums[i+1];
            }
        }

        if sum == total {
            return true;
        }
    }
    false
}

fn part02() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for line in BufReader::new(File::open("input.txt")?).lines() {
        if let Some((left, right)) = line?.split_once(":") {
            let total = left.parse::<i128>()?;
            let mut nums = Vec::new();
            for num in right.split_whitespace() {
                nums.push(num.parse::<i128>()?);
            }

            if is_correct2(total, &nums) {
                sum += total;
            }
        }
    }
    println!("{}", sum);


    Ok(())
}

fn is_correct2(total: i128, nums: &[i128]) -> bool {
    let mut stack = vec![(nums[0], 1)];

    while let Some((sum, x)) = stack.pop() {
        if x == nums.len() {
            if sum == total {
                return true;
            }
        } else {
            stack.push((concat(sum, nums[x]), x + 1));
            stack.push((sum + nums[x], x + 1));
            stack.push((sum * nums[x], x + 1));
        }
    }

    false
}

fn concat(x1: i128, x2: i128) -> i128 {
    let mut stack = Vec::new();
    let mut y2 = x2;
    while y2 > 0 {
        stack.push(y2 % 10);
        y2 /= 10;
    }

    let mut y1 = x1;
    while let Some(v) = stack.pop() {
        y1 *= 10;
        y1 += v;
    }
    y1
}