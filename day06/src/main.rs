use std::{collections::HashSet, error::Error, fs::File, io::{BufReader, Read}};

fn main() {
    if let Err(e) = part01() {
        eprintln!("{}", e);
    }
    if let Err(e) = part02() {
        eprintln!("{}", e);
    }
}

fn part01() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;
    let mut m = text.split("\n").map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let h = m.len() as i32;
    let w = m[0].len() as i32;

    let mut cur_x = 0_i32;
    let mut cur_y = 0_i32;
    for x in 0..h {
        for y in 0..w {
            if m[x as usize][y as usize] == '^' {
                cur_x = x as i32;
                cur_y = y as i32;
            }
        }
    }

    let d = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut di = 0;
    loop {
        m[cur_x as usize][cur_y as usize] = 'X';

        let (dx, dy) = d[di];
        let nxt_x = cur_x + dx;
        let nxt_y = cur_y + dy;
        if 0 <= nxt_x && nxt_x < h && 0 <= nxt_y && nxt_y < w {
            if m[nxt_x as usize][nxt_y as usize] == '#' {
                di = (di + 1) % 4;
            } else {
                cur_x = nxt_x;
                cur_y = nxt_y;
            }
        } else {
            break;
        }
    }

    let mut total = 0;
    for row in &m {
        for ch in row {
            if *ch == 'X' {
                total += 1;
            }
        }
    }
    println!("{total}");

    Ok(())
}




fn part02() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;
    let mut m = text.split("\n").map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let h = m.len() as i32;
    let w = m[0].len() as i32;

    let mut start_x = 0_i32;
    let mut start_y = 0_i32;
    for x in 0..h {
        for y in 0..w {
            if m[x as usize][y as usize] == '^' {
                start_x = x as i32;
                start_y = y as i32;
            }
        }
    }

    let mut total = 0;
    for x in 0..h {
        for y in 0..w {
            if m[x as usize][y as usize] == '.' {
                m[x as usize][y as usize] = '#';
                if is_cycle(&m, start_x, start_y) {
                    total += 1;
                }
                m[x as usize][y as usize] = '.';
            }
        }
    }
    println!("{total}");

    Ok(())
}


fn is_cycle(m: &Vec<Vec<char>>, start_x: i32, start_y: i32) -> bool {
    let h = m.len() as i32;
    let w = m[0].len() as i32;
    let mut cur_x = start_x;
    let mut cur_y = start_y;

    let d = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut di = 0;
    let mut set = HashSet::new();
    while !set.contains(&(cur_x, cur_y, di)) {
        set.insert((cur_x, cur_y, di));

        let (dx, dy) = d[di];
        let nxt_x = cur_x + dx;
        let nxt_y = cur_y + dy;
        if 0 <= nxt_x && nxt_x < h && 0 <= nxt_y && nxt_y < w {
            if m[nxt_x as usize][nxt_y as usize] == '#' {
                di = (di + 1) % 4;
            } else {
                cur_x = nxt_x;
                cur_y = nxt_y;
            }
        } else {
            return false;
        }
    }

    true
}
