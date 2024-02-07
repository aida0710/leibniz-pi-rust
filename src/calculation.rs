pub fn calculate_pi(repetition: i64) -> f64 {
    let mut pi: f64 = 0.0;
    let mut operator: i32 = 1;

    for i in 0..repetition {
        pi += operator as f64 / (2 * i + 1) as f64;
        operator *= -1;
    }

    4.0 * pi
}