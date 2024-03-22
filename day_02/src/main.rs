use std::{collections::HashMap, fs, io::Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("usage: {} <input_filename>", args[0]);
        std::process::exit(1);
    }

    let filename = args[1].as_str();

    let input = fs::read_to_string(filename)?;

    part_1(input.as_str())?;
    part_2(input.as_str())?;

    Ok(())
}

fn part_1(input: &str) -> Result<()> {
    // count lines with exactly two of something
    let mut x2_count = 0;
    let mut x3_count = 0;
    for line in input.lines() {
        let mut map: HashMap<char, i32> = HashMap::new();
        for ch in line.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }

        for (_, count) in &map {
            if *count == 2 {
                x2_count += 1;
                break;
            }
        }

        for (_, count) in &map {
            if *count == 3 {
                x3_count += 1;
                break;
            }
        }
    }

    println!("{}", x2_count * x3_count);
    Ok(())
}

fn part_2(input: &str) -> Result<()> {
    let lines = input
        .lines()
        .filter(|foo| !foo.is_empty())
        .collect::<Vec<&str>>();
    let lines_count = lines.len();
    let mut foo: (u32, usize, usize) = (u32::MAX, 0, 0);

    for i in 0..lines_count {
        for j in i..lines_count {
            if i == j {
                continue;
            }

            let diff = diff(lines[i], lines[j]);
            if diff <= foo.0 {
                foo.0 = diff;
                foo.1 = i;
                foo.2 = j;
            }
        }
    }

    let uncommon = lines[foo.1]
        .chars()
        .zip(lines[foo.2].chars())
        .filter(|(a, b)| *a == *b)
        .map(|(c, _)| c)
        .collect::<String>();
    println!("{}", uncommon);
    Ok(())
}

fn diff(first: &str, second: &str) -> u32 {
    let mut diff: u32 = 0;

    first.chars().zip(second.chars()).for_each(|(a, b)| {
        if a != b {
            diff += 1;
        }
    });
    // println!("{}", diff);
    diff
}
