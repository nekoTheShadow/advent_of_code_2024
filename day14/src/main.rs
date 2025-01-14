use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

use regex::Regex;

fn main() {
    if let Err(e) = part01() {
        eprintln!("{}", e);
    }
    if let Err(e) = part02() {
        eprintln!("{}", e);
    }
}

fn part01() -> Result<(), Box<dyn Error>> {
    let mut robots = read_file()?;
    let mut counter = HashMap::new();
    for robot in robots.iter_mut() {
        for _ in 0..100 {
            robot.update();
        }
        *counter.entry(robot.quadrant()).or_insert(0) += 1;
    }

    let total = counter[&Quadrant::Q1]
        * counter[&Quadrant::Q2]
        * counter[&Quadrant::Q3]
        * counter[&Quadrant::Q4];
    println!("{}", total);
    Ok(())
}

fn part02() -> Result<(), Box<dyn Error>> {
    let mut robots = read_file()?;
    for second in 1.. {
        let mut grid = vec![vec![false; Robot::W as usize]; Robot::H as usize];
        let mut duplicate = false;
        for robot in robots.iter_mut() {
            robot.update();
            if grid[robot.y as usize][robot.x as usize] {
                duplicate = true;
            }
            grid[robot.y as usize][robot.x as usize] = true;
        }

        if !duplicate {
            println!("PART2 : {}", second);
            for row in grid {
                let line = row
                    .iter()
                    .map(|&v| if v { "#" } else { "." })
                    .collect::<String>();
                println!("{}", line);
            }
            break;
        }
    }
    Ok(())
}

fn read_file() -> Result<Vec<Robot>, Box<dyn Error>> {
    let mut buf = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut buf)?;

    let mut robots = Vec::new();
    let re = Regex::new(r"-*[0-9]+")?;
    for line in buf.split("\n") {
        let tokens = re.find_iter(line).map(|m| m.as_str()).collect::<Vec<_>>();
        let robot = Robot {
            x: tokens[0].parse::<i32>()?,
            y: tokens[1].parse::<i32>()?,
            dx: tokens[2].parse::<i32>()?,
            dy: tokens[3].parse::<i32>()?,
        };
        robots.push(robot);
    }
    Ok(robots)
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    const H: i32 = 103;
    const W: i32 = 101;

    fn update(&mut self) {
        self.x = (self.x + self.dx + Self::W) % Self::W;
        self.y = (self.y + self.dy + Self::H) % Self::H;
    }

    fn quadrant(self) -> Quadrant {
        let mid_x = Self::W / 2;
        let mid_y = Self::H / 2;

        if self.x == mid_x || self.y == mid_y {
            return Quadrant::N;
        }

        if self.x < mid_x {
            if self.y < mid_y {
                Quadrant::Q1
            } else {
                Quadrant::Q2
            }
        } else {
            if self.y < mid_y {
                Quadrant::Q3
            } else {
                Quadrant::Q4
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
enum Quadrant {
    Q1,
    Q2,
    Q3,
    Q4,
    N,
}
