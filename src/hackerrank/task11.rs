/// Solution for HackerRank: Diagonal Difference
///
/// Given a square matrix, calculate the absolute difference between
/// the sums of its diagonals.
pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += arr[i][i];
        secondary_diagonal_sum += arr[i][n - 1 - i];
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        let matrix = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];
        // Primary: 11 + 5 - 12 = 4
        // Secondary: 4 + 5 + 10 = 19
        // |4 - 19| = 15
        assert_eq!(diagonal_difference(&matrix), 15);
    }

    #[test]
    fn test_2x2_matrix() {
        let matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        // (1+4) - (2+3) = 5 - 5 = 0
        assert_eq!(diagonal_difference(&matrix), 0);
    }

    #[test]
    fn test_single_element() {
        let matrix = vec![vec![10]];
        assert_eq!(diagonal_difference(&matrix), 0);
    }
}