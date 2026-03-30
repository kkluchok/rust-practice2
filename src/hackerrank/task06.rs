/// ПРАКТИЧНА 6: Number Line Jumps (Kangaroo)
/// Посилання: https://www.hackerrank.com/challenges/kangaroo

pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    // Якщо перший кенгуру позаду і рухається повільніше або з тією ж швидкістю,
    // він ніколи не наздожене другого.
    if v1 <= v2 {
        return "NO".to_string();
    }

    // Вони зустрінуться, якщо різниця початкових позицій ділиться без залишку
    // на різницю їхніх швидкостей.
    // Формула: (x2 - x1) % (v1 - v2) == 0
    if (x2 - x1) % (v1 - v2) == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_will_meet() {
        // Тест з умови: 0 3 4 2 -> YES
        assert_eq!(kangaroo(0, 3, 4, 2), "YES".to_string());
    }

    #[test]
    fn test_kangaroo_never_meet() {
        // Тест з умови: 0 2 5 3 -> NO
        assert_eq!(kangaroo(0, 2, 5, 3), "NO".to_string());
    }

    #[test]
    fn test_same_speed_different_pos() {
        assert_eq!(kangaroo(0, 2, 10, 2), "NO".to_string());
    }
}