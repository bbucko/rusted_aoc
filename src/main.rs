use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/input1.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut found_freqs = HashSet::new();
    let mut current_freq = 0;
    loop {
        for freq in contents.lines().map(process_line).into_iter() {
            current_freq += freq;
            if !found_freqs.contains(&current_freq) {
                found_freqs.insert(current_freq.clone());
            } else {
                println!("{}", current_freq);
                return Ok(());
            }
        }
    }
}

fn process_line(line: &str) -> i32 { line.trim().parse().unwrap() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_ones() {
        assert_eq!(1, process_line("+1"));
    }
}