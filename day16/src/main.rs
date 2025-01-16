use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    if let Err(e) = solve() {
        eprintln!("{}", e);
    }
}

fn solve() -> Result<(), Box<dyn Error>> {
    let matrix = read_file()?;
    let (start_x, start_y) = find_node(&matrix, 'S');
    let (end_x, end_y) = find_node(&matrix, 'E');

    let diffs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut pq = BinaryHeap::new();
    let mut costs = HashMap::new();
    let mut prev = HashMap::new();
    pq.push((Reverse(0), start_x, start_y, 0));
    costs.insert((start_x, start_y, 0_usize), 0_i32);
    while let Some((Reverse(cur_cost), cur_x, cur_y, di)) = pq.pop() {
        if get_cost(&costs, cur_x, cur_y, di) < cur_cost {
            continue;
        }

        let (dx, dy) = diffs[di];
        let nxt_x = cur_x + dx;
        let nxt_y = cur_y + dy;

        let nxt_cost = cur_cost + 1;
        if get_matrix!(matrix, nxt_x, nxt_y) != '#' {
            if nxt_cost < get_cost(&costs, nxt_x, nxt_y, di) {
                costs.insert((nxt_x, nxt_y, di), nxt_cost);
                pq.push((Reverse(nxt_cost), nxt_x, nxt_y, di));
                prev.insert((nxt_x, nxt_y, di), vec![(cur_x, cur_y, di)]);
            } else if nxt_cost == get_cost(&costs, nxt_x, nxt_y, di) {
                prev.get_mut(&(nxt_x, nxt_y, di))
                    .unwrap()
                    .push((cur_x, cur_y, di));
            }
        }

        let nxt_cost = cur_cost + 1000;
        let ni = (di + 1) % 4;
        if nxt_cost < get_cost(&costs, cur_x, cur_y, ni) {
            costs.insert((cur_x, cur_y, ni), nxt_cost);
            pq.push((Reverse(nxt_cost), cur_x, cur_y, ni));
            prev.insert((cur_x, cur_y, ni), vec![(cur_x, cur_y, di)]);
        } else if nxt_cost == get_cost(&costs, cur_x, cur_y, ni) {
            prev.get_mut(&(cur_x, cur_y, ni))
                .unwrap()
                .push((cur_x, cur_y, di));
        }

        let nxt_cost = cur_cost + 1000;
        let ni = (di + 3) % 4;
        if nxt_cost < get_cost(&costs, cur_x, cur_y, ni) {
            costs.insert((cur_x, cur_y, ni), nxt_cost);
            pq.push((Reverse(nxt_cost), cur_x, cur_y, ni));
            prev.insert((cur_x, cur_y, ni), vec![(cur_x, cur_y, di)]);
        } else if nxt_cost == get_cost(&costs, cur_x, cur_y, ni) {
            prev.get_mut(&(cur_x, cur_y, ni))
                .unwrap()
                .push((cur_x, cur_y, di));
        }
    }

    let min_di = (0..4)
        .min_by_key(|i| get_cost(&costs, end_x, end_y, *i))
        .unwrap();

    let mut stack = vec![(end_x, end_y, min_di)];
    let mut visited = HashSet::new();
    while let Some((cur_x, cur_y, cur_di)) = stack.pop() {
        visited.insert((cur_x, cur_y));
        if let Some(v) = prev.get(&(cur_x, cur_y, cur_di)) {
            for &(pre_x, pre_y, pre_di) in v {
                stack.push((pre_x, pre_y, pre_di));
            }
        }
    }

    println!("PART01 : {}", get_cost(&costs, end_x, end_y, min_di));
    println!("PART02 : {}", visited.len());
    Ok(())
}

fn read_file() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut matrix = Vec::new();
    for line in BufReader::new(File::open("input.txt")?).lines() {
        matrix.push(line?.chars().collect());
    }
    Ok(matrix)
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

fn get_cost(costs: &HashMap<(i32, i32, usize), i32>, x: i32, y: i32, di: usize) -> i32 {
    if let Some(v) = costs.get(&(x, y, di)) {
        *v
    } else {
        i32::MAX
    }
}

#[macro_export]
macro_rules! get_matrix {
    ($matrix: expr, $x: expr, $y: expr) => {
        $matrix[$x as usize][$y as usize]
    };
}
