use regex::Regex;
use std::{collections::HashMap, fs};
use time::{Date, Month, PrimitiveDateTime, Time};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("usage: {} <input_filename>", args[0]);
        std::process::exit(0);
    }

    let input = fs::read_to_string(&args[1])?;
    let mut logs: Vec<Log> = vec![];
    let re_date_time = Regex::new(r"\[(\d+{4})-(\d+{2})-(\d+{2}) (\d+{2}):(\d+{2})\]").unwrap();
    for line in input.lines() {
        let (_, [year, month, day, hour, minute]) =
            re_date_time.captures(line).map(|c| c.extract()).unwrap();
        let date = Date::from_calendar_date(
            year.parse().unwrap(),
            Month::January.nth_next(month.parse::<u8>().unwrap() - 1),
            day.parse().unwrap(),
        )?;

        let time = Time::from_hms(hour.parse().unwrap(), minute.parse().unwrap(), 0)?;
        let log_entry: Log;
        if line.contains("asleep") {
            log_entry = Log {
                time: PrimitiveDateTime::new(date, time),
                data: LogData::SleepStart(time.minute()),
            }
        } else if line.contains("wake") {
            log_entry = Log {
                time: PrimitiveDateTime::new(date, time),
                data: LogData::WakeUp(time.minute()),
            }
        } else {
            let start_idx = line.find('#').unwrap() + 1;
            let end_idx = line[start_idx..].find(' ').unwrap();
            log_entry = Log {
                time: PrimitiveDateTime::new(date, time),
                data: LogData::Guard(line[start_idx..start_idx + end_idx].parse().unwrap()),
            }
        }

        logs.push(log_entry);
    }

    logs.sort_by(|log1, log2| log1.time.partial_cmp(&log2.time).unwrap());

    let mut guards: HashMap<u32, [u16; 60]> = HashMap::new();

    let mut logs_iter = logs.iter();

    let mut current_id = 0;
    while let Some(log_entry) = logs_iter.next() {
        match log_entry.data {
            LogData::Guard(id) => {
                current_id = id;
                guards.entry(id).or_insert([0; 60]);
            }
            LogData::SleepStart(s) => {
                let wakeup_minute = logs_iter.next().unwrap().data.inner_num();
                guards.entry(current_id).and_modify(|freq_count| {
                    for i in s as u32..wakeup_minute {
                        freq_count[i as usize] += 1;
                    }
                });
            }
            LogData::WakeUp(_) => unreachable!(),
        }
    }

    part_1(&guards)?;
    part_2(&guards)?;

    Ok(())
}

fn part_1(guards: &HashMap<u32, [u16; 60]>) -> Result<()> {
    let mut most_sleepy_id = 0;
    let mut most_sleepy_duration = 0;
    for (&guard_id, &freq_count) in guards {
        let duration = freq_count.iter().sum();
        if duration >= most_sleepy_duration {
            most_sleepy_duration = duration;
            most_sleepy_id = guard_id;
        }
    }
    let max_min = guards
        .get(&most_sleepy_id)
        .unwrap()
        .iter()
        .enumerate()
        .max_by_key(|(_, &x)| x)
        .map(|(idx, _)| idx)
        .unwrap();
    println!("{}", most_sleepy_id * max_min as u32);

    Ok(())
}

fn part_2(guards: &HashMap<u32, [u16; 60]>) -> Result<()> {
    let mut guard_id = 0;
    let mut amount = 0;
    let mut minute = 0;

    for (id, freqs) in guards {
        let temp_minute = guards
            .get(id)
            .unwrap()
            .iter()
            .enumerate()
            .max_by_key(|(_, &x)| x)
            .map(|(idx, _)| idx)
            .unwrap();

        if freqs[temp_minute] >= amount {
            amount = freqs[temp_minute];
            minute = temp_minute;
            guard_id = *id;
        }
    }

    println!("{}", guard_id * minute as u32);

    Ok(())
}

#[derive(Debug)]
enum LogData {
    Guard(u32),
    SleepStart(u8),
    WakeUp(u8),
}

impl LogData {
    fn inner_num(&self) -> u32 {
        match self {
            LogData::Guard(id) => *id,
            LogData::SleepStart(s) => *s as u32,
            LogData::WakeUp(w) => *w as u32,
        }
    }
}

#[derive(Debug)]
struct Log {
    time: PrimitiveDateTime,
    data: LogData,
}
