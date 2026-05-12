/// Solution for HackerRank: Divisible Sum Pairs
/// 
/// Finds the number of pairs (i, j) where i < j and (ar[i] + ar[j]) % k == 0.
pub fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let n = n as usize;
    let mut count = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_pairs_basic() {
        let ar = vec![1, 3, 2, 6, 1, 2];
        assert_eq!(divisible_sum_pairs(6, 3, &ar), 5);
    }

    #[test]
    fn test_divisible_pairs_no_matches() {
        let ar = vec![1, 2, 3];
        assert_eq!(divisible_sum_pairs(3, 10, &ar), 0);
    }

    #[test]
    fn test_divisible_pairs_all_match() {
        let ar = vec![2, 2, 2];
        assert_eq!(divisible_sum_pairs(3, 2, &ar), 3);
    }
}