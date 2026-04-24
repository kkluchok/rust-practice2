// https://www.hackerrank.com/challenges/sock-merchant/problem
use std::collections::HashMap;

/// Solution for HackerRank: Sales by Match
///
/// There is a large pile of socks that must be paired by color. Given an array
/// of integers representing the color of each sock, determine how many pairs
/// of socks with matching colors there are.
pub fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut counts = HashMap::new();
    let mut pairs = 0;

    for &sock in ar {
        let count = counts.entry(sock).or_insert(0);
        *count += 1;

        // Якщо кількість стала парною, ми знайшли нову пару
        if *count % 2 == 0 {
            pairs += 1;
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        let n = 9;
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(n, &ar), 3);
    }

    #[test]
    fn test_sample_1() {
        let n = 10;
        let ar = vec![1, 1, 3, 1, 2, 1, 3, 3, 3, 3];
        assert_eq!(sock_merchant(n, &ar), 4);
    }

    #[test]
    fn test_no_pairs() {
        let n = 3;
        let ar = vec![1, 2, 3];
        assert_eq!(sock_merchant(n, &ar), 0);
    }

    #[test]
    fn test_all_pairs() {
        let n = 4;
        let ar = vec![10, 10, 10, 10];
        assert_eq!(sock_merchant(n, &ar), 2);
    }
}