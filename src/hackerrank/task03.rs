/// ПРАКТИЧНА 3: Staircase
/// https://www.hackerrank.com/challenges/staircase

pub fn staircase(n: i32) -> String {
    let mut result = String::new();
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        result.push_str(&format!("{}{}\n", spaces, hashes));
    }
    print!("{}", result); // Вивід для консолі
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase() {
        let expected = "  #\n ##\n###\n";
        assert_eq!(staircase(3), expected);
    }
}