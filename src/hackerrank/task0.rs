/// ПРАКТИЧНА 4: Grading Students
/// Опис: Алгоритм округлення оцінок згідно з правилами університету Sam's

pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut final_results = Vec::new();

    for &score in grades {
        if score >= 38 {
            // Знаходимо наступне число, кратне 5
            let next_five = score + (5 - (score % 5));

            // Якщо різниця менша за 3, округлюємо
            if next_five - score < 3 {
                final_results.push(next_five);
            } else {
                final_results.push(score);
            }
        } else {
            // Якщо оцінка менша за 38, залишаємо як є
            final_results.push(score);
        }
    }
    final_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_grading_logic() {
        let test_data = vec![73, 67, 38, 33];
        let result = grading_students(&test_data);
        assert_eq!(result, vec![75, 67, 40, 33]);
    }

    #[test]
    fn check_no_rounding() {
        assert_eq!(grading_students(&[29]), vec![29]);
        assert_eq!(grading_students(&[57]), vec![57]);
    }
}
