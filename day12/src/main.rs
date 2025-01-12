use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
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
    let mut visited = vec![vec![false; w as usize]; h as usize];
    let mut total = 0;
    for x in 0..h {
        for y in 0..w {
            if get!(visited, x, y) {
                continue;
            }

            let ch = get!(matrix, x, y);
            let mut stack = vec![(x, y)];
            let mut area = 0;
            let mut fence = 0;
            while let Some((i, j)) = stack.pop() {
                if get!(visited, i, j) {
                    continue;
                }

                get!(visited, i, j) = true;
                area += 1;
                for (ni, nj) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                    if check(ni, h) && check(nj, w) && get!(matrix, ni, nj) == ch {
                        stack.push((ni, nj));
                    } else {
                        fence += 1;
                    }
                }
            }
            total += area * fence;
        }
    }

    println!("{total}");
    Ok(())
}

fn part02() -> Result<(), Box<dyn Error>> {
    let matrix = read_file()?;
    let h = matrix.len() as i32;
    let w = matrix[0].len() as i32;
    let mut visited = vec![vec![false; w as usize]; h as usize];
    let mut total = 0;
    for x in 0..h {
        for y in 0..w {
            if get!(visited, x, y) {
                continue;
            }

            let ch = get!(matrix, x, y);
            let mut stack = vec![(x, y)];
            let mut area = 0;
            let mut fence1 = Vec::new();
            let mut fence2 = Vec::new();
            while let Some((i, j)) = stack.pop() {
                if get!(visited, i, j) {
                    continue;
                }

                get!(visited, i, j) = true;
                area += 1;
                for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let ni = i + di;
                    let nj = j + dj;
                    if check(ni, h) && check(nj, w) && get!(matrix, ni, nj) == ch {
                        stack.push((ni, nj));
                    } else {
                        if di != 0 {
                            fence1.push((i, di, j));
                        }
                        if dj != 0 {
                            fence2.push((j, dj, i));
                        }
                    }
                }
            }

            let fence = merge(&fence1) + merge(&fence2);
            total += area * fence;
        }
    }

    println!("{total}");
    Ok(())
}

fn merge(fence: &Vec<(i32, i32, i32)>) -> i32 {
    let n = fence.len();
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        for j in i+1..n {
            let (p1, q1, r1) = fence[i];
            let (p2, q2, r2) = fence[j]; 
            if p1 == p2 && q1 == q2 && (r1 - r2).abs() == 1 {
                uf.union(i, j);
            }
        }
    }
    (0..n).map(|i| uf.find(i)).collect::<HashSet<_>>().len() as i32
}

fn read_file() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut lines = Vec::new();
    for line in BufReader::new(File::open("input.txt")?).lines() {
        lines.push(line?.chars().collect::<Vec<_>>());
    }
    Ok(lines)
}

fn check(i: i32, h: i32) -> bool {
    0 <= i && i < h
}

#[macro_export]
macro_rules! get {
    ($matrix: expr, $x: expr, $y: expr) => {
        $matrix[$x as usize][$y as usize]
    };
}

pub struct UnionFind {
    parents: Vec<usize>
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self{parents: (0..n).collect()}
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            return x;
        }
        self.parents[x] = self.find(self.parents[x]);
        self.parents[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x != y {
            self.parents[x] = y;
        }
    }
}