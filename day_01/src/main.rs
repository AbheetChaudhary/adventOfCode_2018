use std::{collections::HashSet, fs};

fn parser(log: &str) -> Vec<i32> {
    let count = log.len();
    let mut values: Vec<i32> = Vec::with_capacity(count);

    for line in log.lines() {
        match line.parse::<i32>() {
            Ok(val) => {
                values.push(val);
            }
            Err(_) => (),
        }
    }

    values
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("usage: {} <input_filename>", args[0]);
        std::process::exit(1);
    }

    let filename = args[1].as_str();

    let input = fs::read_to_string(filename).unwrap();

    part_1(input.as_str());
    part_2(input.as_str());
}

fn part_1(input: &str) {
    let logs = parser(&input[..]);

    let mut freq = 0;

    for log_entry in logs {
        freq += log_entry;
    }

    println!("{}", freq);
}

fn part_2(input: &str) {
    let logs = parser(&input[..]);
    let mut freq = 0;
    let mut seen_freqs: HashSet<i32> = HashSet::new();

    seen_freqs.insert(freq);

    loop {
        for log_entry in &logs {
            freq += log_entry;
            if seen_freqs.contains(&freq) {
                println!("{}", freq);
                return;
            } else {
                seen_freqs.insert(freq);
            }
        }
    }
}
