use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufReader, Read},
    os::unix::raw::uid_t,
};

fn main() {
    if let Err(e) = part01() {
        eprintln!("{}", e);
    }
    if let Err(e) = part02() {
        eprintln!("{}", e);
    }
}

fn read_file() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut buf = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut buf)?;
    let matrix = buf
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok(matrix)
}

fn part01() -> Result<(), Box<dyn Error>> {
    let mut matrix = read_file()?;
    let (start_x, start_y) = find_node(&matrix, 'S');
    let (goal_x, goal_y) = find_node(&matrix, 'E');

    let (cur_cost, _) = dijskstra(&matrix, start_x, start_y, goal_x, goal_y);

    let h = matrix.len() as i32;
    let w = matrix[0].len() as i32;
    let mut total = 0;
    for x in 1..h - 1 {
        for y in 1..w - 1 {
            if get!(matrix, x, y) == '#' {
                get!(matrix, x, y) = '.';
                let (cost, _) = dijskstra(&matrix, start_x, start_y, goal_x, goal_y);
                if cur_cost - cost >= 100 {
                    total += 1;
                }
                get!(matrix, x, y) = '#';
            }
        }
    }

    println!("{}", total);
    Ok(())
}

fn part02() -> Result<(), Box<dyn Error>> {
    let matrix = read_file()?;
    let (start_x, start_y) = find_node(&matrix, 'S');
    let (goal_x, goal_y) = find_node(&matrix, 'E');
    let (_, c) = dijskstra(&matrix, start_x, start_y, goal_x, goal_y);

    let mut path = Vec::new();
    for (x, row) in c.iter().enumerate() {
        for (y, v) in row.iter().enumerate() {
            if *v != i32::MAX {
                path.push((x, y));
            }
        }
    }
    path.sort_by_key(|(x, y)| c[*x][*y]);

    let n = path.len();
    let mut sum = 0;
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = path[i as usize];
            let (x2, y2) = path[j as usize];
            let d = x1.abs_diff(x2) + y1.abs_diff(y2);
            if d <= 20 && i + d + (n - j) <= n - 100 {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
    Ok(())
}

fn dijskstra(
    matrix: &Vec<Vec<char>>,
    start_x: i32,
    start_y: i32,
    goal_x: i32,
    goal_y: i32,
) -> (i32, Vec<Vec<i32>>) {
    let h = matrix.len() as i32;
    let w = matrix[0].len() as i32;
    let mut c = vec![vec![i32::MAX; w as usize]; h as usize];
    let mut q = VecDeque::new();
    q.push_back((start_x, start_y));
    get!(c, start_x, start_y) = 0;

    while let Some((cur_x, cur_y)) = q.pop_front() {
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nxt_x = cur_x + dx;
            let nxt_y = cur_y + dy;
            if 0 <= nxt_x
                && nxt_x < h
                && 0 <= nxt_y
                && nxt_y < w
                && get!(matrix, nxt_x, nxt_y) != '#'
                && get!(c, cur_x, cur_y) + 1 < get!(c, nxt_x, nxt_y)
            {
                get!(c, nxt_x, nxt_y) = get!(c, cur_x, cur_y) + 1;
                q.push_back((nxt_x, nxt_y));
            }
        }
    }
    (get!(c, goal_x, goal_y), c)
}

fn find_node(matrix: &Vec<Vec<char>>, target: char) -> (i32, i32) {
    for (x, row) in matrix.iter().enumerate() {
        for (y, ch) in row.iter().enumerate() {
            if *ch == target {
                return (x as i32, y as i32);
            }
        }
    }
    (-1, -1)
}

#[macro_export]
macro_rules! get {
    ($matrix:expr, $x: expr, $y: expr) => {
        $matrix[$x as usize][$y as usize]
    };
}
