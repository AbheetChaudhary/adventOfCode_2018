use std::fs;
use std::str;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1])?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let stack = Stack::try_from(input.trim())?;
    println!("{}", stack.next);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut lengths: Vec<usize> = Vec::with_capacity(26);
    for c in 'a'..='z' {
        let stack = Stack::try_from_but_c(input.trim(), c)?;
        lengths.push(stack.next);
    }

    let min = lengths
        .iter()
        .enumerate()
        .min_by_key(|(_, &len)| len)
        .map(|(_, len)| len)
        .unwrap();

    println!("{}", min);
    Ok(())
}

const MAXDEPTH: usize = 50000;
struct Stack {
    slots: [u8; MAXDEPTH],
    next: usize,
}

impl Stack {
    fn push(&mut self, x: u8) -> Result<()> {
        if self.next >= MAXDEPTH {
            return Err("stack overflow".into());
        } else {
            self.slots[self.next] = x;
            self.next += 1;
            Ok(())
        }
    }

    fn pop(&mut self) -> Option<u8> {
        if self.next != 0 {
            self.next -= 1;
            Some(self.slots[self.next])
        } else {
            None
        }
    }

    fn polarity(&self) -> bool {
        if self.next > 1 {
            return self.slots[self.next - 1].abs_diff(self.slots[self.next - 2]) == 32;
        }

        false
    }

    fn try_from_but_c(
        value: &str,
        c: char,
    ) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let mut stack = Stack {
            slots: [0; MAXDEPTH],
            next: 0,
        };

        for ch in value.bytes().filter(|&ch| {
            let diff = (c as u8).abs_diff(ch);
            diff != 0 && diff != 32
        }) {
            stack.push(ch)?;
            while let true = stack.polarity() {
                let _ = stack.pop();
                let _ = stack.pop();
            }
        }

        Ok(stack)
    }
}

impl TryFrom<&str> for Stack {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut stack = Stack {
            slots: [0; MAXDEPTH],
            next: 0,
        };

        for c in value.bytes() {
            stack.push(c)?;
            while let true = stack.polarity() {
                let _ = stack.pop();
                let _ = stack.pop();
            }
        }

        Ok(stack)
    }
}
