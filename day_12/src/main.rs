use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type X = i32;

#[derive(Debug, Clone)]
struct LinearFarm {
    fertile: HashSet<X>,
    infertile: HashSet<X>,
    current_state: BTreeSet<X>,
    generation: usize,
    rules: [bool; 32],
}

impl LinearFarm {
    fn new(initial_state: &str, rules: [bool; 32]) -> Self {
        let len = initial_state.len();
        let mut linear_farm = LinearFarm {
            fertile: HashSet::with_capacity(len),
            infertile: HashSet::with_capacity(len),
            current_state: BTreeSet::new(),
            generation: 0,
            rules: rules,
        };
        for (idx, c) in initial_state.trim().chars().enumerate() {
            if c == '#' {
                linear_farm.current_state.insert(idx as i32);
            }
        }

        linear_farm
    }

    /*
     * Populates self.fertile and self.infertile hashmaps to use them 
     * for creating next generation line farm.
     */
    fn prepare_next(&mut self) {
        let (min, max) = self.current_min_max();

        for pot_x in (min - 2)..=(max + 2) {
            let left_to_left = self.has_plant(pot_x - 2);
            let left = self.has_plant(pot_x - 1);
            let current = self.has_plant(pot_x);
            let right = self.has_plant(pot_x + 1);
            let right_to_right = self.has_plant(pot_x + 2);

            // create an index in rules using these 5 values
            let array = [left_to_left, left, current, right, right_to_right];
            let mut rules_index = 0;
            for (idx, &b) in array.iter().rev().enumerate() {
                rules_index += b as usize * 2usize.pow(idx as u32);
            }

            if self.rules[rules_index] {
                self.fertile.insert(pot_x);
            } else {
                self.infertile.insert(pot_x);
            }
        }
    }

    /*
     * Uses self.fertile and self.infertile hashmaps to add and remove plants from 
     * current gen pots and create the next gen. Drais these hashmaps in the process.
     */
    fn goto_next_gen(&mut self) {
        // Remove infertile pots from self.current_state.
        self.infertile.drain().for_each(|x| {
            self.current_state.remove(&x);
        });

        // Adds fertile pots to self.current_state.
        self.fertile.drain().for_each(|x| {
            self.current_state.insert(x);
        });

        self.generation += 1;
    }

    #[inline]
    // Checks if pot at index pot_x has plant in current state or not.
    fn has_plant(&self, pot_x: X) -> u8 {
        match self.current_state.contains(&pot_x) {
            true => 1u8,
            false => 0u8,
        }
    }

    // min and max indexes for pots with plants in current gen.
    fn current_min_max(&self) -> (X, X) {
        assert!(self.current_state.len() != 0);

        let mut min = X::MAX;
        let mut max = X::MIN;

        for &x in &self.current_state {
            if x < min {
                min = x;
            }

            if x > max {
                max = x;
            }
        }

        (min, max)
    }

    // Hash function to capture the relative arrangement of different plants. The 
    // idea is to do a product of differences between the indexes of consecutive
    // plants. Also incorporate the index at which the difference occurs to save 
    // against collisions.
    fn current_state_hash(&self) -> usize {
        self.current_state
            .iter()
            .map(|x| (x - self.current_state.first().unwrap()) as usize /* consecutive differences */)
            .skip(1) /* coz the first one is 0 */
            .zip(0..)
            .fold(1usize, |hash, e| hash.wrapping_mul(e.0).wrapping_sub(e.1))
    }

    // Restore self.current_state to the generation 0 value.
    fn restore_initial(&mut self, initial: &BTreeSet<X>) {
        self.fertile.clear();
        self.infertile.clear();
        self.current_state.clear();
        self.current_state = initial.clone();
        self.generation = 0;
    }

    // Iterate for next n generations
    fn speedrun_gen_n(&mut self, gen: usize) {
        for _ in 0..gen {
            self.prepare_next();
            self.goto_next_gen();
        }
    }
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("usage: {} <input-filename>", args[0]);
        return Err("No input file.".into());
    }

    let input_string = std::fs::read_to_string(&args[1])?;

    let (upper, lower) = input_string
        .split_once("\n\n")
        .ok_or("Could not split in upper an lower")?;

    let initial_state = upper
        .split_once(':')
        .ok_or("cannot get initial state string")?
        .1;

    let mut rules = [false; 32];

    for line in lower.lines() {
        let (pattern, will_grow) = line
            .split_once("=>")
            .ok_or("could not get pattern and future")?;

        let mut index = 0;
        for (idx, c) in pattern.trim().chars().rev().enumerate() {
            if c == '#' {
                index += 2usize.pow(idx as u32);
            }
        }

        if will_grow.trim() == "#" {
            rules[index] = true;
        }
    }

    let linear_farm = LinearFarm::new(initial_state, rules);

    part1(linear_farm.clone())?;
    part2(linear_farm.clone())?;

    Ok(())
}

fn part1(mut linear_farm: LinearFarm) -> Result<()> {
    const MAX_GEN: usize = 20;

    linear_farm.speedrun_gen_n(MAX_GEN);

    let mut sum = 0;
    linear_farm.current_state.iter().for_each(|x| sum += x);
    println!("{}", sum);

    Ok(())
}

fn part2(mut linear_farm: LinearFarm) -> Result<()> {
    const MAX_GEN: usize = 50_000_000_000;

    let original_state = linear_farm.current_state.clone();

    // HashMap with current_state hash as key and its first generation of occurance
    // as value. Store hash and generation number of each generation.
    let mut state_count: HashMap<usize /* hash */, usize /* gen */> = HashMap::new();

    state_count.insert(linear_farm.current_state_hash(), 0);

    // Loop until there is a generation which has a reoccuring hash. Break the 
    // loop with the original and repeat generation numbers.
    let (original_gen, repeat_gen) = loop {
        linear_farm.prepare_next();
        linear_farm.goto_next_gen();

        let hash = linear_farm.current_state_hash();
        if state_count.contains_key(&hash) {
            break (state_count.get(&hash).map(|&x| x).unwrap(), linear_farm.generation);
        } else {
            state_count.insert(hash, linear_farm.generation);
        }
    };

    // Period of repetition for generations with same relative spatial arrangement
    let period = repeat_gen - original_gen;

    // Calculating min index of plant in original_gen and repeat_gen
    linear_farm.restore_initial(&original_state);
    linear_farm.speedrun_gen_n(original_gen);

    let (original_gen_min, _) = linear_farm.current_min_max();

    linear_farm.restore_initial(&original_state);
    linear_farm.speedrun_gen_n(repeat_gen);

    let (repeat_gen_min, _) = linear_farm.current_min_max();

    // Shift of pot with min index between original and repeat gen.
    let shift = repeat_gen_min - original_gen_min;

    // shadow is what MAX_GEN's pot arrangement corresponds in [original_gen, repeat_gen)
    let shadow = MAX_GEN % period + original_gen as usize;

    // Number of cycles of original_gen and repeat_gen before reaching MAX_GEN.
    let repeat_count = (MAX_GEN - original_gen) / period;

    linear_farm.restore_initial(&original_state);
    linear_farm.speedrun_gen_n(shadow);

    let (shadow_min, _) = linear_farm.current_min_max();

    let final_shift: i64 =
        original_gen_min as i64
        + repeat_count as i64 * shift as i64
        + shadow_min as i64 - original_gen_min as i64;
    let sum: i64 = linear_farm
        .current_state
        .iter()
        .map(|&x| x as i64 + final_shift as i64 - shadow_min as i64)
        .fold(0, |sum, x| sum + x);

    println!("{}", sum);

    Ok(())
}
