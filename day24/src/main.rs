use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read},
    usize,
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
    let mut memory = HashMap::new();
    let mut stmts = Vec::new();
    let mut is_first = true;
    for line_result in BufReader::new(File::open("input.txt")?).lines() {
        let line = line_result?;
        if line == "" {
            is_first = false;
            continue;
        }

        if is_first {
            let (wire, bit) = line.split_once(": ").unwrap();
            memory.insert(wire.to_string(), bit.parse::<u128>()?);
        } else {
            let token = line.split_whitespace().collect::<Vec<_>>();
            stmts.push((
                token[0].trim().to_string(),
                token[1].trim().to_string(),
                token[2].trim().to_string(),
                token[4].trim().to_string(),
            ));
        }
    }

    let n = stmts.len();
    let mut c = 0;
    while c < n {
        for (left, gate, right, result) in &stmts {
            if let (Some(b1), Some(b2), None) = (
                memory.get(&left.to_string()),
                memory.get(&right.to_string()),
                memory.get(&result.to_string()),
            ) {
                if gate == "AND" {
                    memory.insert(result.clone(), b1 & b2);
                } else if gate == "OR" {
                    memory.insert(result.clone(), b1 | b2);
                } else if gate == "XOR" {
                    memory.insert(result.clone(), b1 ^ b2);
                }
                c += 1;
            }
        }
    }

    let mut a = memory
        .into_iter()
        .filter(|(k, v)| k.starts_with("z"))
        .collect::<Vec<_>>();
    a.sort_by_key(|(k, v)| k.clone());
    a.reverse();

    let b = a.into_iter().fold(0_u128, |acc, (k, v)| (acc << 1) | v);
    println!("{}", b);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;
    let sections = text.split("\n\n").collect::<Vec<_>>();
    let gates = sections[1]
        .split("\n")
        .map(|line| {
            let tokens = line.split(" ").collect::<Vec<_>>();
            let in1 = tokens[0];
            let op = tokens[1];
            let in2 = tokens[2];
            let out = tokens[4];
            (in1, in2, op, out)
        })
        .collect::<Vec<_>>();

    let mut wrong = HashSet::new();
    for x in 0..gates.len() {
        let (in1, in2, op, out) = gates[x];

        if out.starts_with("z") && op != "XOR" && out != "z45" {
            wrong.insert(out);
        }

        if op == "XOR"
            && !["x", "y", "z"].iter().any(|c| out.starts_with(c))
            && !["x", "y", "z"].iter().any(|c| in1.starts_with(c))
            && !["x", "y", "z"].iter().any(|c| in2.starts_with(c))
        {
            wrong.insert(out);
        }

        if op == "AND" && !(in1.ends_with("00") || in2.ends_with("00")) {
            for y in 0..gates.len() {
                let (sub_in1, sub_in2, sub_op, sub_out) = gates[y];
                if (out == sub_in1 || out == sub_in2) && sub_op != "OR" {
                    wrong.insert(out);
                }
            }
        }

        if op == "XOR" {
            for y in 0..gates.len() {
                let (sub_in1, sub_in2, sub_op, sub_out) = gates[y];
                if (out == sub_in1 || out == sub_in2) && sub_op == "OR" {
                    wrong.insert(out);
                }
            }
        }
    }

    let mut ans = wrong.iter().map(|out| out.to_string()).collect::<Vec<_>>();
    ans.sort();
    println!("{}", ans.join(","));
    Ok(())
}
