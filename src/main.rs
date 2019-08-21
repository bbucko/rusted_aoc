use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/input1.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let sum: i32 = contents.lines().map(|str| str.trim().parse::<i32>().unwrap()).sum();
    println!("{}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_ones() {}
}