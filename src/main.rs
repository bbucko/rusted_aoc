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


    let mut board = [[(' ', 0); SIZE]; SIZE];

    for (i, row) in board.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            for (sign, (x, y)) in coords.clone() {
                let dist = (x - i as i32).abs() + (y - j as i32).abs();
                cell.1 += dist;
            }
        }
    }

    for line in board.iter() {
        for char in line.iter() {
            if char.1 < 10000 {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!()
    }

    let mut result = 0;

    for (i, row) in board.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if cell.1 < 10000 {
                result += 1;
            }
        }
    }

    println!("Result: {:?}", result);

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
