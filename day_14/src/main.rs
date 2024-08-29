type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const LUT: [[u8; 2]; 9] = [
    [1, 0],
    [1, 1],
    [1, 2],
    [1, 3],
    [1, 4],
    [1, 5],
    [1, 6],
    [1, 7],
    [1, 8],
];

struct ScoreBoard {
    recipes: Vec<u8>,
    currents: [usize; 2],
}

impl ScoreBoard {
    fn new() -> Self {
        ScoreBoard {
            recipes: vec![3, 7],
            currents: [0, 1],
        }
    }

    fn update_current(&mut self) {
        for curr in self.currents.iter_mut() {
            *curr = (*curr + 1 + self.recipes[*curr] as usize) % self.recipes.len();
        }
    }

    fn create_new_recipes(&mut self) {
        let sum = (self.recipes[self.currents[0]] + self.recipes[self.currents[1]]) as usize;
        if sum < 10 {
            self.recipes.push(sum as u8);
        } else {
            self.recipes.extend_from_slice(&LUT[sum - 10]);
        }
        self.update_current();
    }
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("usage: {} <input-filename>", args[0]);
        std::process::exit(-1);
    }

    let input = args[1].parse::<usize>()?;

    part1(input)?;
    part2(input)?;

    Ok(())
}

fn part1(input: usize) -> Result<()> {
    const SIZE: usize = 10;
    let mut scoreboard = ScoreBoard::new();

    while scoreboard.recipes.len() < SIZE + input + 1 {
        scoreboard.create_new_recipes();
        // scoreboard.debug_dump();
    }

    for c in &scoreboard.recipes[input..input+SIZE] {
        print!("{}", c);
    }
    println!();

    Ok(())
}

fn part2(input: usize) -> Result<()> {
    let bytes = input.to_string().chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|&c| c as u8 - 48)
        .collect::<Vec<u8>>();

    let size: usize = bytes.len();

    let mut scoreboard = ScoreBoard::new();

    let mut start_from = 0;

    'outer: loop {
        while start_from + bytes.len() < scoreboard.recipes.len() {
            let mut matched = true;

            for (s, t) in scoreboard.recipes[start_from..start_from+size].iter().zip(bytes.iter()) {
                if s != t {
                    matched = false;
                    break;
                }
            }

            if matched {
                break 'outer;
            }

            start_from += 1;
        }

        scoreboard.create_new_recipes();
    }

    println!("{start_from}");

    Ok(())
}
