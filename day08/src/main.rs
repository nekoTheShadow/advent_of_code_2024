use std::{collections::{HashMap, HashSet}, error::Error, fs::File, io::{BufRead, BufReader, Read}};

fn main() {
    if let Err(e) = part01() {
        println!("{}", e);
    }
    if let Err(e) = part02() {
        println!("{}", e);
    }
}


fn part01() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;
    let m = text.split("\n").map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();

    let h = m.len() as i32;
    let w = m[0].len() as i32;
    let mut antennas = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            if m[i as usize][j as usize] != '.' {
                antennas.entry(m[i as usize][j as usize]).or_insert(Vec::new()).push((i, j));
            }
        }
    }

    let mut nodes = HashSet::new();
    for (_, points) in antennas {
        for i in 0..points.len() {
            for j in i+1..points.len() {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];

                let x3 = x2 + (x2 - x1);
                let y3 = y2 + (y2 - y1);
                if 0 <= x3 && x3 < h && 0 <= y3 && y3 < w {
                    nodes.insert((x3, y3));
                }

                let x4 = x1 + (x1 - x2);
                let y4 = y1 + (y1 - y2);
                if 0 <= x4 && x4 < h && 0 <= y4 && y4 < w {
                    nodes.insert((x4, y4));
                }
            }
        }
    }

    println!("{:?}", nodes.len());

    Ok(())
}



fn part02() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;
    let m = text.split("\n").map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();

    let h = m.len() as i32;
    let w = m[0].len() as i32;

    let mut antennas = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            if m[i as usize][j as usize] != '.' {
                antennas.entry(m[i as usize][j as usize]).or_insert(Vec::new()).push((i, j));
            }
        }
    }

    let mut nodes = HashSet::new();
    for (_, points) in antennas {
        for i in 0..points.len() {
            for j in i+1..points.len() {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];

                let mut x3 = x2 + (x2 - x1);
                let mut y3 = y2 + (y2 - y1);
                while 0 <= x3 && x3 < h && 0 <= y3 && y3 < w {
                    nodes.insert((x3, y3));
                    x3 += x2 - x1;
                    y3 += y2 - y1;
                }

                let mut x4 = x1 + (x1 - x2);
                let mut y4 = y1 + (y1 - y2);
                while 0 <= x4 && x4 < h && 0 <= y4 && y4 < w {
                    nodes.insert((x4, y4));
                    x4 += x1 - x2;
                    y4 += y1 - y2;
                }
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if m[i as usize][j as usize] != '.' {
                nodes.insert((i, j));
            }
        }
    }

    println!("{:?}", nodes.len());

    Ok(())
}