/// Solution for HackerRank: Breaking the Records
///
/// Counts how many times a player breaks their highest and lowest score records
/// throughout a season.
pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }

    let mut min_score = scores[0];
    let mut max_score = scores[0];
    let mut min_count = 0;
    let mut max_count = 0;

    for &score in &scores[1..] {
        if score > max_score {
            max_score = score;
            max_count += 1;
        } else if score < min_score {
            min_score = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        let result = breaking_records(&scores);
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn test_sample_1() {
        let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        let result = breaking_records(&scores);
        assert_eq!(result, vec![4, 0]);
    }

    #[test]
    fn test_empty_scores() {
        let scores = vec![];
        let result = breaking_records(&scores);
        assert_eq!(result, vec![0, 0]);
    }

    #[test]
    fn test_single_score() {
        let scores = vec![10];
        let result = breaking_records(&scores);
        assert_eq!(result, vec![0, 0]);
    }
}