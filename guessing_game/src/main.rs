use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // {} 一对空的大括号表示占位符，用于显示后面的参数值，可以使用多个括号对来依次捕捉多个参数值
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); // mut 表示变量可变（变量默认是不可变的）

    io::stdin().read_line(&mut guess) // &mut 表示可变引用（引用默认也是不可变的，如&guess就不可变）
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
