/// Solution for HackerRank: Birthday Cake Candles
/// 
/// Counts the number of candles that have the maximum height.
/// Efficient single-pass implementation.
pub fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let mut tallest = 0;
    let mut count = 0;

    for &candle in candles {
        if candle > tallest {
            tallest = candle;
            count = 1;
        } else if candle == tallest {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birthday_candles_standard() {
        let candles = vec![3, 2, 1, 3];
        assert_eq!(birthday_cake_candles(&candles), 2);
    }

    #[test]
    fn test_birthday_candles_all_max() {
        let candles = vec![5, 5, 5, 5, 5];
        assert_eq!(birthday_cake_candles(&candles), 5);
    }

    #[test]
    fn test_birthday_candles_unique_max() {
        let candles = vec![1, 2, 8, 4, 1];
        assert_eq!(birthday_cake_candles(&candles), 1);
    }

    #[test]
    fn test_birthday_candles_empty() {
        assert_eq!(birthday_cake_candles(&[]), 0);
    }
}