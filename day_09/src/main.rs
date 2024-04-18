use std::cell::RefCell;

const MAGIC: usize = 23;

type Idx = usize;

#[derive(Debug, Clone)]
pub struct Node {
    next: Idx,  // next node index within memory buffer
    prev: Idx,  // previous node index within memory buffer
    val: usize, // the value of node
}

type Unit = RefCell<Option<Node>>;

#[derive(Debug, Clone)]
pub struct Memory {
    buffer: Vec<Unit>,  // memory buffer
    curr: Idx,  // index of current marble
    empty_slots: Vec<Idx>,  // emptied indexes in memory buffer for quick access
}

fn next_unit<'a>(unit: &Unit, memory: &'a Memory) -> &'a Unit {
    memory
        .buffer
        .get(unit.borrow().as_ref().unwrap().next)
        .unwrap()
}

fn prev_unit<'a>(unit: &Unit, memory: &'a Memory) -> &'a Unit {
    memory
        .buffer
        .get(unit.borrow().as_ref().unwrap().prev)
        .unwrap()
}

fn unit_value(unit: &Unit) -> usize {
    unit.borrow().as_ref().unwrap().val
}

impl Memory {
    fn new(capacity: usize) -> Self {
        let node = Node {
            next: 0,
            prev: 0,
            val: 0,
        };
        let mut buffer = Vec::with_capacity(capacity);
        buffer.push(RefCell::new(Some(node)));

        Memory {
            buffer,
            curr: 0,
            empty_slots: Vec::new(),
        }
    }

    fn insert(&mut self, val: usize) -> usize {
        if val % MAGIC != 0 {
            let pos = self.get_slot();
            let curr = self.buffer.get(self.curr).unwrap();

            // next from current marble
            let nxt = next_unit(curr, self);

            // next to next from current marble
            let nxt2nxt = next_unit(nxt, self);

            // the node that gets in between
            let node = Node {
                next: nxt.borrow().as_ref().unwrap().next,
                prev: nxt2nxt.borrow().as_ref().unwrap().prev,
                val,
            };

            // modify the surrounding two marbles to point to the new current marble
            nxt.borrow_mut().as_mut().unwrap().next = pos;
            nxt2nxt.borrow_mut().as_mut().unwrap().prev = pos;

            // insert the current marble in its position
            self.buffer.get_mut(pos).unwrap().borrow_mut().replace(node);

            // finally update the current marble index
            self.curr = pos;

            // return score 0
            0
        } else {
            let curr = self.buffer.get(self.curr).unwrap();
            let mut score = val;
            let mut to_remove = prev_unit(curr, self);
            for _ in 0..6 {
                to_remove = prev_unit(to_remove, self);
            }

            score += unit_value(to_remove);

            // previous node from to_remove
            let pre = prev_unit(to_remove, self);

            // next node from to_remove
            let nxt = next_unit(to_remove, self);

            // this will be pushed in self.empty_slots
            let index_to_remove = pre.borrow().as_ref().unwrap().next;

            let new_current_index = to_remove.borrow().as_ref().unwrap().next;

            // modify the surrounding two marbles of to_remove
            pre.borrow_mut().as_mut().unwrap().next = to_remove.borrow().as_ref().unwrap().next;
            nxt.borrow_mut().as_mut().unwrap().prev = to_remove.borrow().as_ref().unwrap().prev;

            // remove the marble
            let _ = to_remove.borrow_mut().take();

            // update available slots
            self.empty_slots.push(index_to_remove);

            // finally update the current marble index
            self.curr = new_current_index;

            score
        }
    }

    /// gets an index to store new node or creates one at the end if none found
    fn get_slot(&mut self) -> usize {
        if self.empty_slots.is_empty() {
            self.buffer.push(RefCell::new(None));
            return self.buffer.len() - 1;
        } else {
            self.empty_slots.pop().unwrap()
        }
    }

    /// there must be some non-None in buffer
    fn _print(&self) {
        let mut i = 0;
        let head = loop {
            if self.buffer.get(i).unwrap().borrow().is_some() {
                break self.buffer.get(i).unwrap();
            }

            i += 1;
        };

        print!("{}", head.borrow().as_ref().unwrap().val);
        let mut next = next_unit(head, self);

        while unit_value(&next) != unit_value(&head) {
            print!(" -> {}", next.borrow().as_ref().unwrap().val);
            next = next_unit(next, self);
        }
        println!();
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Input {
    players: usize,
    max_points: usize,
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("usage: {} <input_file>", &args[0]);
        std::process::exit(1);
    }

    let untrimmed = std::fs::read_to_string(&args[1]).unwrap();
    let data = untrimmed.trim();

    let mut input_array = Vec::new();

    for line in data.lines() {
        let numbers: Vec<&str> = line.split(';').collect();
        let input = Input {
            players: numbers[0].parse::<usize>().unwrap(),
            max_points: numbers[1].parse::<usize>().unwrap(),
        };
        input_array.push(input);
    }
    // println!("{:#?}", input_array);

    part1(&input_array).unwrap();
    part2(&input_array).unwrap();

    Ok(())
}

fn part1(inputs: &Vec<Input>) -> Result<()> {
    for input in inputs {
        let mut scores = vec![0; input.players];
        let mut memory = Memory::new(input.max_points);
        for turn in 1..input.max_points + 1 {
            scores[turn % input.players] += memory.insert(turn);
        }

        println!(
            "players: {}, marbles: {}, high score: {}",
            input.players,
            input.max_points,
            max_score(&scores)
        );
    }

    Ok(())
}

fn part2(inputs: &Vec<Input>) -> Result<()> {
    for input in inputs {
        let mut scores = vec![0; input.players];
        let mut memory = Memory::new(input.max_points);
        for turn in 1..(input.max_points + 1) * 100 {
            scores[turn % input.players] += memory.insert(turn);
        }

        println!(
            "players: {}, marbles: {}, high score: {}",
            input.players,
            input.max_points * 100,
            max_score(&scores)
        );
    }

    Ok(())
}

fn max_score(scores: &Vec<usize>) -> usize {
    let mut max = 0;
    for s in scores {
        if *s >= max {
            max = *s;
        }
    }

    max
}
