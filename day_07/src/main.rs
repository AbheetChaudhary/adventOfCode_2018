use std::collections::{HashMap, HashSet};
use std::{str::FromStr, string};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }
    let data = std::fs::read_to_string(&args[1])?;

    let mut conditions: Vec<Pre> = Vec::new();
    for line in data.lines() {
        conditions.push(Pre::from_str(line).unwrap());
    }

    let shift_size = 5;
    part1(&conditions).unwrap();
    part2(&conditions, shift_size).unwrap();

    Ok(())
}

fn part1(conditions: &Vec<Pre>) -> Result<()> {
    /* 
     * a hashmap b/w a characters(task) and all the tasks that need to be done before it that has
     * not been done yet.
     * */
    let mut conditions_map: HashMap<char, HashSet<char>> = HashMap::new();
    for condition in conditions {
        conditions_map
            .entry(condition.1)
            .or_insert(HashSet::new())
            .insert(condition.0);
        // make sure that all the inpur characters are present in conditions_map. Something mapping
        // to an empty hashset is just a task that has no pending dependencies
        conditions_map.entry(condition.0).or_insert(HashSet::new());
    }

    // gather every task that has all its pre-reqs completed
    let mut ready: HashSet<char> = HashSet::new();
    for (&node, requirements) in &conditions_map {
        if requirements.is_empty() {
            ready.insert(node);
        }
    }

    let mut order = String::new();

    loop {
        // until nothing remains to do
        if ready.is_empty() {
            break;
        }

        let first = get_first(&ready).unwrap();
        // complete a task...remove it from pending dependencies
        conditions_map.iter_mut().for_each(|(_, requirements)| {
            if requirements.contains(&first) {
                requirements.remove(&first);
            }
        });

        conditions_map.remove(&first).unwrap(); // remove completed nodes as its requirements are
                                                // done and it will interfere in dertemining what
                                                // else should get into the ready set.

        ready.remove(&first); // get rid of completed nodes

        // update 'ready'
        for (&node, requirements) in &conditions_map {
            if !ready.contains(&node) && requirements.is_empty() {
                ready.insert(node);
            }
        }

        order.push(first);
    }

    println!("{order}");

    Ok(())
}

struct Work {
    task: char,
    time_spent: i32,
}

fn part2(conditions: &Vec<Pre>, shift_size: usize) -> Result<()> {
    let mut conditions_map: HashMap<char, HashSet<char>> = HashMap::new();
    for condition in conditions {
        conditions_map
            .entry(condition.1)
            .or_insert(HashSet::new())
            .insert(condition.0);
        // make sure that all the inpur characters are present in conditions_map
        conditions_map.entry(condition.0).or_insert(HashSet::new());
    }

    let mut ready: HashSet<char> = HashSet::new();
    for (&node, requirements) in &conditions_map {
        if requirements.is_empty() {
            ready.insert(node);
        }
    }

    for task in &ready {
        conditions_map.remove(task).unwrap();
    }

    let mut time = 0;
    let mut factory: Vec<Option<Work>> = Vec::new();
    factory.resize_with(shift_size, || None);

    assign_work(&mut factory, &mut ready);

    loop {
        let mut is_working = false;
        for work in &factory {
            if work.is_some() {
                is_working = true;
            }
        }

        if !is_working {
            break;
        }

        time += 1;
        let completed = tick(&mut factory);
        if completed.is_some() {
            // something completed
            for task in &completed.unwrap() {
                // update requirements set for each task
                conditions_map.iter_mut().for_each(|(_, requirements)| {
                    requirements.remove(task);
                })
            }

            // collect newly ready tasks
            let mut ready_temp = HashSet::new();
            for (&task, requirements) in &conditions_map {
                if requirements.is_empty() {
                    ready_temp.insert(task);
                }
            }

            // remove ready tasks from collections_map so that they dont appear in next search of
            // ready tasks, and put them in ready set
            for &task in &ready_temp {
                conditions_map.remove(&task).unwrap();
                ready.insert(task);
            }

            // assign work to idle workers
            assign_work(&mut factory, &mut ready);
        }
    }

    println!("{time}");

    Ok(())
}

/* 
 * Run one step of the simulation. If any worker has completed its job then mark it as idle and
 * return the list of completed jobs.
 * */
fn tick(factory: &mut Vec<Option<Work>>) -> Option<Vec<char>> {
    let mut completed = Vec::new();
    for work in factory {
        let mut task_completed = false;
        work.as_mut().map(|w| {
            w.time_spent += 1;

            // nodes should be uppercase
            if w.time_spent == (60 + (w.task as u8 - 'A' as u8 + 1u8) as i32) {
                task_completed = true;
            }

            // for testinput
            /* if w.time_spent == (w.task as u8 - 'A' as u8 + 1u8) as i32 {
                task_completed = true;
            } */
        });

        if task_completed {
            completed.push(work.as_mut().unwrap().task);
            *work = None;
        }
    }

    if completed.is_empty() {
        None
    } else {
        Some(completed)
    }
}

/* 
 * go through every worker and if any is idle then look for any ready pending work an assign it to that
 * worker. Remove the assigned tasks from the set of ready pending works
 * */
fn assign_work(factory: &mut Vec<Option<Work>>, ready: &mut HashSet<char>) {
    for work in factory {
        if work.is_none() {
            // someone is idle
            let task = get_first(&ready);
            if task.is_some() {
                // ...we got something to do
                ready.remove(&task.unwrap());
                *work = Some(Work {
                    // initialize the worker
                    task: task.unwrap(),
                    time_spent: 0,
                });
            }
        }
    }
}

fn get_first(set: &HashSet<char>) -> Option<char> {
    if set.is_empty() {
        None
    } else {
        set.into_iter()
            .reduce(|a, b| if *a < *b { a } else { b })
            .copied()
    }
}

#[derive(Debug)]
struct Pre(char, char);

impl FromStr for Pre {
    type Err = string::ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let character_substrings = s
            .split(" ")
            .filter(|&token| token.len() == 1)
            .map(|sub_str| char::from_str(sub_str).unwrap())
            .collect::<Vec<char>>();
        let first = character_substrings
            .get(0)
            .expect("could not get substring containing first character");

        let second = character_substrings
            .get(1)
            .expect("could not get substring containing second character");

        Ok(Pre(*first, *second))
    }
}
