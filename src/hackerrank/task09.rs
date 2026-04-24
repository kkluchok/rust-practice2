// https://www.hackerrank.com/challenges/migratory-birds/problem
/// Solution for HackerRank: Migratory Birds
///
/// Given an array of bird sightings where every element represents a bird type id,
/// determine the id of the most frequently sighted type. If more than 1 type
/// has been spotted that maximum amount, return the smallest of their ids.
pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6]; // Типи птахів від 1 до 5

    for &bird_type in arr {
        if bird_type >= 1 && bird_type <= 5 {
            counts[bird_type as usize] += 1;
        }
    }

    let mut max_count = 0;
    let mut result_id = 0;

    for i in 1..=5 {
        if counts[i] > max_count {
            max_count = counts[i];
            result_id = i as i32;
        }
    }

    result_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_sample_1() {
        let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
        assert_eq!(migratory_birds(&arr), 3);
    }

    #[test]
    fn test_all_same() {
        let arr = vec![2, 2, 2];
        assert_eq!(migratory_birds(&arr), 2);
    }

    #[test]
    fn test_tie_pick_smallest() {
        let arr = vec![1, 1, 2, 2];
        assert_eq!(migratory_birds(&arr), 1);
    }
}