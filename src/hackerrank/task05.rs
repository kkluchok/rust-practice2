/// ПРАКТИЧНА 5: Apple and Orange
/// Автор: vviktoriiaaa

pub fn count_apples_and_oranges(
    s: i32, // початок будинку
    t: i32, // кінець будинку
    a: i32, // позиція яблуні
    b: i32, // позиція апельсинового дерева
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    // Рахуємо яблука, що впали на будинок
    let apple_count = apples
        .iter()
        .filter(|&&d| {
            let pos = a + d;
            pos >= s && pos <= t
        })
        .count() as i32;

    // Рахуємо апельсини, що впали на будинок
    let orange_count = oranges
        .iter()
        .filter(|&&d| {
            let pos = b + d;
            pos >= s && pos <= t
        })
        .count() as i32;

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_logic() {
        // Тестові дані з HackerRank: s=7, t=11, a=5, b=15
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        let (res_a, res_o) = count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges);

        assert_eq!(res_a, 1);
        assert_eq!(res_o, 1);
    }
}
