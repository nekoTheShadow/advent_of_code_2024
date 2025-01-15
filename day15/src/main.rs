use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
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
    let (mut matrix, commands) = read_file()?;

    let h = matrix.len() as i32;
    let w = matrix[0].len() as i32;
    let d = HashMap::from([('<', (0, -1)), ('>', (0, 1)), ('^', (-1, 0)), ('v', (1, 0))]);

    let mut cur_x = 0;
    let mut cur_y = 0;
    for x in 0..h {
        for y in 0..w {
            if get!(matrix, x, y) == '@' {
                cur_x = x;
                cur_y = y;
            }
        }
    }

    for command in commands {
        let (dx, dy) = d[&command];
        let mut nxt_x = cur_x;
        let mut nxt_y = cur_y;
        let mut a = vec![];
        while get!(matrix, nxt_x, nxt_y) != '#' && get!(matrix, nxt_x, nxt_y) != '.' {
            a.push((nxt_x, nxt_y));
            nxt_x += dx;
            nxt_y += dy;
        }

        if get!(matrix, nxt_x, nxt_y) != '#' {
            a.push((nxt_x, nxt_y));
            a.reverse();
            for s in a.windows(2) {
                let (x1, y1) = s[0];
                let (x2, y2) = s[1];

                let tmp = get!(matrix, x1, y1);
                get!(matrix, x1, y1) = get!(matrix, x2, y2);
                get!(matrix, x2, y2) = tmp;
            }

            cur_x += dx;
            cur_y += dy;
        }
    }

    println!("{}", culc_score(&matrix, h, w));

    Ok(())
}

fn part02() -> Result<(), Box<dyn Error>> {
    let (matrix, commands) = read_file()?;
    let mut matrix = matrix
        .iter()
        .map(|row| {
            let mut line = String::new();
            for &ch in row {
                if ch == '#' {
                    line.push_str("##");
                }
                if ch == 'O' {
                    line.push_str("[]");
                }
                if ch == '.' {
                    line.push_str("..");
                }
                if ch == '@' {
                    line.push_str("@.");
                }
            }
            line.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let h = matrix.len() as i32;
    let w = matrix[0].len() as i32;
    let d = HashMap::from([('<', (0, -1)), ('>', (0, 1)), ('^', (-1, 0)), ('v', (1, 0))]);

    let mut cur_x = 0;
    let mut cur_y = 0;
    for x in 0..h {
        for y in 0..w {
            if get!(matrix, x, y) == '@' {
                cur_x = x;
                cur_y = y;
            }
        }
    }

    for command in commands {
        let (dx, dy) = d[&command];
        if trypush(&matrix, cur_x, cur_y, dx, dy) {
            push(&mut matrix, cur_x, cur_y, dx, dy);
            cur_x += dx;
            cur_y += dy;
        }
    }

    println!("{}", culc_score(&matrix, h, w));
    Ok(())
}

fn trypush(matrix: &Vec<Vec<char>>, cur_x: i32, cur_y: i32, dx: i32, dy: i32) -> bool {
    let nxt_x = cur_x + dx;
    let nxt_y = cur_y + dy;
    if get!(matrix, nxt_x, nxt_y) == '#' {
        return false;
    }
    if get!(matrix, nxt_x, nxt_y) == '.' {
        return true;
    }

    if dy == 0 {
        if get!(matrix, nxt_x, nxt_y) == ']' {
            trypush(matrix, nxt_x, nxt_y, dx, dy) && trypush(matrix, nxt_x, nxt_y - 1, dx, dy)
        } else {
            trypush(matrix, nxt_x, nxt_y, dx, dy) && trypush(matrix, nxt_x, nxt_y + 1, dx, dy)
        }
    } else {
        trypush(matrix, nxt_x, nxt_y + dy, dx, dy)
    }
}

fn push(matrix: &mut Vec<Vec<char>>, cur_x: i32, cur_y: i32, dx: i32, dy: i32) {
    let nxt_x = cur_x + dx;
    let nxt_y = cur_y + dy;

    if get!(matrix, nxt_x, nxt_y) == '#' {
        return;
    }
    if get!(matrix, nxt_x, nxt_y) == '.' {
        swap(matrix, cur_x, cur_y, nxt_x, nxt_y);
        return;
    }

    if dy == 0 {
        if get!(matrix, nxt_x, nxt_y) == ']' {
            push(matrix, nxt_x, nxt_y, dx, dy);
            push(matrix, nxt_x, nxt_y - 1, dx, dy);
            swap(matrix, cur_x, cur_y, nxt_x, nxt_y);
        } else {
            push(matrix, nxt_x, nxt_y, dx, dy);
            push(matrix, nxt_x, nxt_y + 1, dx, dy);
            swap(matrix, cur_x, cur_y, nxt_x, nxt_y);
        }
    } else {
        push(matrix, nxt_x, nxt_y + dy, dx, dy);
        swap(matrix, nxt_x, nxt_y, nxt_x, nxt_y + dy);
        swap(matrix, cur_x, cur_y, nxt_x, nxt_y);
    }
}

fn swap(matrix: &mut Vec<Vec<char>>, cur_x: i32, cur_y: i32, nxt_x: i32, nxt_y: i32) {
    let tmp = get!(matrix, cur_x, cur_y);
    get!(matrix, cur_x, cur_y) = get!(matrix, nxt_x, nxt_y);
    get!(matrix, nxt_x, nxt_y) = tmp;
}

fn culc_score(matrix: &Vec<Vec<char>>, h: i32, w: i32) -> i32 {
    let mut total = 0;
    for x in 0..h {
        for y in 0..w {
            if get!(matrix, x, y) == 'O' || get!(matrix, x, y) == '[' {
                total += x * 100 + y;
            }
        }
    }
    total
}

fn read_file() -> Result<(Vec<Vec<char>>, Vec<char>), Box<dyn Error>> {
    let mut matrix = Vec::new();
    let mut commands = Vec::new();
    let mut first = true;
    for line_result in BufReader::new(File::open("input.txt")?).lines() {
        let line = line_result?;
        if line == "" {
            first = false;
        } else {
            if first {
                matrix.push(line.chars().collect::<Vec<_>>());
            } else {
                commands.extend(line.chars());
            }
        }
    }
    Ok((matrix, commands))
}

#[macro_export]
macro_rules! get {
    ($m:expr, $x:expr, $y:expr) => {
        $m[$x as usize][$y as usize]
    };
}
