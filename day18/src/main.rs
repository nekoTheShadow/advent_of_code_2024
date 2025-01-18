use std::{
    collections::{HashMap, HashSet, VecDeque},
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
    let all_points = read_file()?;
    let points = all_points[0..1024].into_iter().collect::<HashSet<_>>();
    let n = 71;

    let mut q = VecDeque::new();
    q.push_back((0, 0));
    let mut cost = HashMap::new();
    cost.insert((0, 0), 0);
    while let Some((cur_x, cur_y)) = q.pop_front() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nxt_x = cur_x + dx;
            let nxt_y = cur_y + dy;
            if 0 <= nxt_x
                && nxt_x < n
                && 0 <= nxt_y
                && nxt_y < n
                && !points.contains(&(nxt_x, nxt_y))
            {
                let cur_v = cost[&(cur_x, cur_y)];
                let nxt_v = cost.get(&(nxt_x, nxt_y)).unwrap_or(&i32::MAX);
                if cur_v + 1 < *nxt_v {
                    cost.insert((nxt_x, nxt_y), cur_v + 1);
                    q.push_back((nxt_x, nxt_y));
                }
            }
        }
    }

    println!("{}", cost[&(70, 70)]);
    Ok(())
}

fn read_file() -> Result<Vec<(i32, i32)>, Box<dyn Error>> {
    let mut lines = Vec::new();
    for line_result in BufReader::new(File::open("input.txt")?).lines() {
        let line = line_result?;
        let tokens = line.split(",").collect::<Vec<_>>();
        let x = tokens[0].parse::<i32>()?;
        let y = tokens[1].parse::<i32>()?;
        lines.push((x, y));
    }
    Ok(lines)
}

fn part02() -> Result<(), Box<dyn Error>> {
    let n = 71 as i32;
    let mut points = read_file()?;

    let mut uf = UnionFind::new((n * n) as usize);
    for x in 0..n {
        for y in 0..n {
            if points.contains(&(x, y)) {
                continue;
            }

            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x + dx;
                let ny = y + dy;
                if 0 <= nx && nx < n && 0 <= ny && ny < n && !points.contains(&(nx, ny)) {
                    let v1 = x * n + y;
                    let v2 = nx * n + ny;
                    uf.union(v1 as usize, v2 as usize);
                }
            }
        }
    }

    while let Some((x, y)) = points.pop() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;
            if 0 <= nx && nx < n && 0 <= ny && ny < n && !points.contains(&(nx, ny)) {
                let v1 = x * n + y;
                let v2 = nx * n + ny;
                uf.union(v1 as usize, v2 as usize);
            }
        }
        if uf.find((0 * n + 0) as usize) == uf.find((70 * n + 70) as usize) {
            println!("{},{}", x, y);
            break;
        }
    }

    Ok(())
}

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect::<Vec<_>>(),
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            return x;
        }
        self.parents[x] = self.find(self.parents[x]);
        return self.parents[x];
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x != y {
            self.parents[x] = y;
        }
    }
}
