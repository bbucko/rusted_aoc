#![feature(test)]

extern crate test;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

const SIZE: usize = 400;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/input.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut letters = "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz".chars();

    let coords: Vec<(char, (i32, i32))> = contents.lines()
        .map(parse_line)
        .map(|(x, y)| (letters.next().unwrap(), (x, y)))
        .collect();


    let mut board = [[(' ', i32::max_value()); SIZE]; SIZE];
    for (sign, (x, y)) in coords {
        for (i, row) in board.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                let dist = (x - i as i32).abs() + (y - j as i32).abs();

                if dist < cell.1 {
                    cell.0 = sign;
                    cell.1 = dist;
                } else if dist == cell.1 {
                    cell.0 = '.';
                }
            }
        }
    }

    for line in board.iter() {
        for char in line.iter() {
            print!("{}", char.0);
        }
        println!()
    }

    let mut result = HashMap::new();

    for (i, row) in board.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            let count = result.entry(cell.0).or_insert(0);

            if i == 0 || j == 0 || i == SIZE - 1 || j == SIZE - 1 {
                *count = -1;
            }

            if *count > -1 {
                *count += 1;
            }
        }
    }

    println!("Result: {:?}", result.into_iter().max_by_key(|(_, b)| *b).unwrap());

    Ok(())
}

fn parse_line(line: &str) -> (i32, i32) {
    single_step_per_map(line)
}

fn single_step_per_map(line: &str) -> (i32, i32) {
    let points: Vec<i32> = line.split(",")
        .map(|point| point.trim().parse::<i32>().unwrap())
        .collect();
    (points[0], points[1])
}

fn multiple_step_per_map(line: &str) -> (i32, i32) {
    let points: Vec<i32> = line
        .split(",")
        .map(|point| point.trim())
        .map(|point| point.parse::<i32>())
        .map(|point| point.unwrap())
        .collect();
    (points[0], points[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!((0, 0), parse_line("0,0"));
        assert_eq!((1, 2), parse_line("1, 2"));
        assert_eq!((3, 4), parse_line(" 3 , 4 "));
    }

    #[test]
    fn test_me() {}

    #[bench]
    fn test_single(b: &mut test::Bencher) {
        b.iter(|| single_step_per_map("0,0"));
    }

    #[bench]
    fn test_multi(b: &mut test::Bencher) {
        b.iter(|| multiple_step_per_map("0,0"));
    }
}
