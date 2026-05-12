/// Solution for HackerRank: Bill Division
/// 
/// Determines if the bill was split fairly between two friends.
/// k: the index of the item Anna didn't eat
/// b: the amount of money that Anna contributed to the bill
pub fn bon_appetit_check(bill: &[i32], k: i32, b: i32) -> String {
    let k = k as usize;
    
    // Calculate sum of all items except the one Anna didn't eat
    let total_anna_ate: i32 = bill.iter()
        .enumerate()
        .filter(|&(i, _)| i != k)
        .map(|(_, &val)| val)
        .sum();

    let fair_share = total_anna_ate / 2;

    if b == fair_share {
        "Bon Appetit".to_string()
    } else {
        (b - fair_share).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bill_fair_split() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit_check(&bill, 1, 7), "Bon Appetit");
    }

    #[test]
    fn test_bill_overcharged() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit_check(&bill, 1, 12), "5");
    }

    #[test]
    fn test_bill_another_case() {
        let bill = vec![5, 5, 5];
        assert_eq!(bon_appetit_check(&bill, 0, 5), "Bon Appetit");
    }
}