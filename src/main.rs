mod calculation;

use std::io::{self, Write};
use std::time::{Duration, Instant};
use crate::calculation::calculate_pi;

fn main() {
    loop {
        let mut input: String = String::new();

        println!("何度計算を繰り返しますか？以下の選択肢から選択してください。");
        println!("1: 1万回");
        println!("2: 100万回");
        println!("3: 1億回");
        println!("4: 100億回");
        println!("5: 1兆回");
        println!("6: 100兆回");
        println!("7: 自分で入力");

        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        let selection_limb: &str = input.trim();
        let repetition: i64 = match selection_limb {
            "1" => 10000,
            "2" => 1000000,
            "3" => 100000000,
            "4" => 10000000000,
            "5" => 1000000000000,
            "6" => 100000000000000,
            "7" => {
                println!("計算回数を入力してください");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                input.trim().parse().unwrap()
            }
            _ => {
                println!("1から7の数字を入力してください");
                continue;
            }
        };

        println!("計算中...");

        let start_time: Instant = Instant::now();

        let result: f64 = calculate_pi(repetition);

        let elapsed_time: Duration = start_time.elapsed();

        println!("Results : {}", result);
        println!("Repetition: {}", repetition);
        println!("Calculation time: {:.2} seconds", elapsed_time.as_secs_f64());
    }
}