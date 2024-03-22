use std::{collections::HashMap, fs};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Claim {
    id: u32,
    left_pad: u32,
    width: u32,
    top_pad: u32,
    height: u32,
}

impl Claim {
    fn new(line: &str) -> Self {
        let tokens = line[1..]
            .split(['@', ',', ':', 'x'])
            .map(|sub_str| sub_str.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        Claim {
            id: tokens[0],
            left_pad: tokens[1],
            top_pad: tokens[2],
            width: tokens[3],
            height: tokens[4],
        }
    }

    fn iter(&self) -> Rectangle {
        Rectangle {
            width: self.width,
            height: self.height,
            current_x: self.left_pad,
            start_x: self.left_pad,
            current_y: self.top_pad,
            start_y: self.top_pad,
        }
    }
}

struct Rectangle {
    width: u32,
    height: u32,
    start_x: u32,
    current_x: u32,
    start_y: u32,
    current_y: u32,
}

impl Iterator for Rectangle {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        let returnable: (u32, u32);
        if self.current_x < (self.width + self.start_x)
            && self.current_y < (self.height + self.start_y)
        {
            returnable = (self.current_x, self.current_y);
            if self.current_x == (self.width + self.start_x - 1) {
                self.current_x = self.start_x;
                self.current_y += 1;
            } else {
                self.current_x += 1;
            }

            return Some(returnable);
        } else {
            None
        }
    }
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("usage: {} <input_filename>", args[0]);
        std::process::exit(1);
    }

    let filename = args[1].as_str();

    let input = fs::read_to_string(filename)?;

    let input_lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let mut claims: Vec<Claim> = Vec::with_capacity(input_lines.len());

    for line in input_lines {
        claims.push(Claim::new(line));
    }

    let mut grid: HashMap<(u32, u32), u32> = HashMap::new();

    for claim in &claims {
        claim.iter().for_each(|(x, y)| {
            *grid.entry((x, y)).or_default() += 1;
        });
    }

    part_1(&grid)?;
    part_2(&claims, &grid)?;

    Ok(())
}

fn part_1(grid: &HashMap<(u32, u32), u32>) -> Result<()> {
    let mut repeated_claims = 0;
    for (_, claim_counts) in grid {
        if *claim_counts > 1 {
            repeated_claims += 1;
        }
    }

    println!("contested: {}", repeated_claims);

    Ok(())
}

fn part_2(claims: &Vec<Claim>, grid: &HashMap<(u32, u32), u32>) -> Result<()> {
    let uncontested: Option<&Claim>;

    uncontested = claims
        .iter()
        .filter(|claim| {
            for (x, y) in claim.iter() {
                if *grid.get(&(x, y)).unwrap() != 1 {
                    return false;
                }
            }
            true
        })
        .next();

    println!("uncontested: {}", uncontested.unwrap().id);

    Ok(())
}
