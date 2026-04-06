fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    fn lcm(a: i32, b: i32) -> i32 {
        a * b / gcd(a, b)
    }

    // Знаходимо НСК для масиву a
    let mut lcm_a = a[0];
    for &num in a.iter() {
        lcm_a = lcm(lcm_a, num);
    }

    // Знаходимо НСД для масиву b
    let mut gcd_b = b[0];
    for &num in b.iter() {
        gcd_b = gcd(gcd_b, num);
    }

    // Рахуємо кількість чисел між ними
    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let a = vec![2, 4];
    let b = vec![16, 32, 96];

    let result = get_total_x(&a, &b);
    println!("{}", result);
}