use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/input.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;


    let result = "abcdefghijklmnopqrstuvwxyz".chars()
        .map(|check| react_polymer(contents.clone(), check))
        .map(|polymer| polymer.len())
        .min()
        .unwrap_or(0);

    dbg!(result);

    Ok(())
}

fn react_polymer(contents: String, check: char) -> String {
    contents.chars()
        .filter(|a| !check.eq_ignore_ascii_case(a))
        .fold(String::new(), fold_polymer)
}

fn fold_polymer(polymer: String, unit: char) -> String {
    let last_polymer = polymer.chars().last().unwrap_or_else(|| unit);
    if units_react(unit, last_polymer) {
        let (first, _) = polymer.split_at(polymer.len() - 1);
        return String::from(first);
    }

    let mut result = String::from(polymer);
    result.push(unit);
    result
}

fn units_react(char_a: char, char_b: char) -> bool {
    return char_a != char_b && char_a.to_uppercase().eq(char_b.to_uppercase());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_fold() {
        assert_eq!("aBaa", fold_polymer("aBa".to_string(), 'a'));
        assert_eq!("aB", fold_polymer("aBa".to_string(), 'A'));
    }

    #[test]
    fn test_reaction() {
        assert!(!units_react('a', 'a'));
        assert!(!units_react('a', 'b'));

        assert!(units_react('a', 'A'));
    }
}
