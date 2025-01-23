use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
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
    let mut g = HashMap::new();
    for (a, b) in read_file()? {
        g.entry(a.clone())
            .or_insert(HashSet::new())
            .insert(b.clone());
        g.entry(b.clone())
            .or_insert(HashSet::new())
            .insert(a.clone());
    }

    let nodes = g.keys().collect::<Vec<_>>();
    let n = nodes.len();
    let mut total = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let v1 = nodes[i];
                let v2 = nodes[j];
                let v3 = nodes[k];
                if (v1.starts_with("t") || v2.starts_with("t") || v3.starts_with("t"))
                    && g[v1].contains(v2)
                    && g[v2].contains(v3)
                    && g[v3].contains(v1)
                {
                    total += 1;
                }
            }
        }
    }

    println!("{}", total);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let mut g = HashMap::new();
    for (a, b) in read_file()? {
        g.entry(a.clone())
            .or_insert(HashSet::new())
            .insert(b.clone());
        g.entry(b.clone())
            .or_insert(HashSet::new())
            .insert(a.clone());
    }
    let nodes = g.keys().map(|node| node.to_string()).collect::<Vec<_>>();

    let mut max_set = HashSet::new();
    dfs(&g, &nodes, 0, &mut HashSet::new(), &mut max_set);

    let mut ans = max_set
        .iter()
        .map(|i| nodes[*i].clone())
        .collect::<Vec<_>>();
    ans.sort();
    println!("{}", ans.join(","));

    Ok(())
}

fn dfs(
    g: &HashMap<String, HashSet<String>>,
    nodes: &Vec<String>,
    i: usize,
    set: &mut HashSet<usize>,
    max_set: &mut HashSet<usize>,
) {
    if i == nodes.len() {
        if max_set.len() < set.len() {
            max_set.clear();
            max_set.extend(set.iter());
        }
        return;
    }

    dfs(g, nodes, i + 1, set, max_set);
    if set.iter().all(|&j| g[&nodes[j]].contains(&nodes[i])) {
        set.insert(i);
        dfs(g, nodes, i + 1, set, max_set);
        set.remove(&i);
    }
}

fn read_file() -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut lines = Vec::new();
    for line in BufReader::new(File::open("input.txt")?).lines() {
        if let Some((a, b)) = line?.split_once("-") {
            lines.push((a.into(), b.into()));
        }
    }
    Ok(lines)
}
