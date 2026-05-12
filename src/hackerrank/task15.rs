/// Solution for HackerRank: Drawing Book
/// 
/// Calculates the minimum number of pages to turn to reach page p in an n-page book.
/// Turning can start from either the front or the back.
pub fn page_count(n: i32, p: i32) -> i32 {
    // Turning from the front: each flip covers 2 pages (0-1, 2-3, etc.)
    let from_front = p / 2;
    
    // Turning from the back: total flips minus flips to p
    let from_back = (n / 2) - (p / 2);

    std::cmp::min(from_front, from_back)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_count_from_front() {
        assert_eq!(page_count(6, 2), 1);
    }

    #[test]
    fn test_page_count_from_back() {
        assert_eq!(page_count(5, 4), 0);
    }

    #[test]
    fn test_page_count_large_book() {
        assert_eq!(page_count(10, 5), 2);
    }

    #[test]
    fn test_page_count_last_page_even() {
        assert_eq!(page_count(2, 1), 0);
    }
}