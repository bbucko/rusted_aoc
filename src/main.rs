extern crate chrono;

use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::Read;

use chrono::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/input4.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let events: BTreeSet<(DateTime<Local>, String)> = contents
        .lines()
        .map(parse_line)
        .collect();

    let mut emp_id = 0;
    let mut start = 0;
    let mut slept_for = HashMap::new();
    let mut slept_count_by_emp_and_minutes = HashMap::new();

    for (time, msg) in &events {
        if msg.starts_with("Guard #") {
            emp_id = msg
                .replace("Guard #", "")
                .replace(" begins shift", "")
                .parse::<i32>()
                .unwrap_or(-1);
        } else if msg.starts_with("falls asleep") {
            start = time.timestamp_millis();
        } else if msg.starts_with("wakes up") {
            let duration = ((time.timestamp_millis() - start) / 1000 / 60) as u32;
            let slept_for_by_emp_id = slept_for.entry(emp_id).or_insert(0);
            *slept_for_by_emp_id += duration;

            let slept_count_by_emp_and_minutes_entry = slept_count_by_emp_and_minutes.entry(emp_id).or_insert([0; 60]);
            let end_minute = time.minute();
            let start_minute = end_minute - duration;

            for minute in start_minute..end_minute {
                slept_count_by_emp_and_minutes_entry[minute as usize] += 1;
            }

            start = 0;
        }
    }

    let slept_longest = slept_for.iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();
    let guard_id_who_slept_the_longest = slept_longest.0;
    let minute_when_he_slept_the_most = slept_count_by_emp_and_minutes[guard_id_who_slept_the_longest].iter().enumerate().max_by_key(|&(_, v)| v).unwrap().0 as i32;
    dbg!(guard_id_who_slept_the_longest * minute_when_he_slept_the_most);

    let slept_the_most = slept_count_by_emp_and_minutes.iter().max_by(|(_, sleep_count_per_minute1), (_, sleep_count_per_minute2)| sleep_count_per_minute1.iter().max().unwrap().cmp(sleep_count_per_minute2.iter().max().unwrap())).unwrap();

    let max_minutes_slept = slept_the_most.1.iter().max().unwrap();
    let slept_the_most_on = slept_the_most.1.iter().position(|x| x == max_minutes_slept).unwrap() as i32;
    dbg!(slept_the_most.0 * slept_the_most_on);


    Ok(())
}

fn parse_line(line: &str) -> (DateTime<Local>, String) {
//    [1518-11-01 00:00] Guard #10 begins shift
//    [1518-11-01 00:05] falls asleep
//    [1518-11-01 00:25] wakes up

    let date_as_string = &line[0..18];
    let msg = &line[19..];

    let date = Local.datetime_from_str(date_as_string, "[%Y-%m-%d %H:%M]").unwrap();
    (date, String::from(msg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_different() {}
}