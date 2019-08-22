use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/input2.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents
        .lines()
        .collect();

    let result = lines.clone()
        .into_iter()
        .flat_map(|line_a| cartesian_with_line(lines.clone(), line_a))
        .filter(|(a, b)| only_one_different_character(a, b))
        .map(|(a, b)| same_characters(a, b))
        .max()
        .unwrap();

    println!("{}", result);

    Ok(())
}

fn cartesian_with_line<'a, 'b>(lines: Vec<&'a str>, line: &'b str) -> Vec<(&'b str, &'a str)> {
    lines
        .into_iter()
        .filter_map(|line_b| {
            if line.ne(line_b) {
                Some((line, line_b))
            } else {
                None
            }
        })
        .collect()
}

fn same_characters(line_a: &str, line_b: &str) -> String {
    line_a
        .chars()
        .zip(line_b.chars())
        .flat_map(|(a, b)| {
            if a.eq(&b) {
                Some(a)
            } else {
                None
            }
        })
        .collect()
}

fn only_one_different_character(line_a: &str, line_b: &str) -> bool {
    line_a
        .chars()
        .zip(line_b.chars())
        .filter(|(char_a, char_b)| char_a.ne(&char_b))
        .count() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartesian() {
        assert_eq!(cartesian_with_line(vec!["b"], "a"), vec![("a", "b")]);
        assert_eq!(cartesian_with_line(vec!["b", "c"], "a"), vec![("a", "b"), ("a", "c")]);
        assert_eq!(cartesian_with_line(vec!["a", "b", "c"], "a"), vec![("a", "b"), ("a", "c")]);
        assert_eq!(cartesian_with_line(vec!["a", "b", "c"], "b"), vec![("b", "a"), ("b", "c")]);
    }

    #[test]
    fn test_process_line() {
        assert_eq!(only_one_different_character("abcdef", "abddef"), true);
        assert_eq!(only_one_different_character("abcdef", "ijkxyz"), false);
    }

    #[test]
    fn test_different() {
        assert_eq!(same_characters("abcdef", "abddef"), "abdef");
        assert_eq!(same_characters("abc", "def"), "");
    }
}