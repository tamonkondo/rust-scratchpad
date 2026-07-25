use rand::RngExt;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the Number!");

    // 0..100は0以上100未満、0..=100は0以上100以下。複数の乱数条件が欲しい場合はif文でbool判定を使う
    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("文字がないよ");
        // parseは型定義した値に変換してくれる。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数値を入力してください。");
                continue;
            },
        };
        println!("You guessd: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
