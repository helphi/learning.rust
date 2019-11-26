use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // `thread_rng` 是一个
    // `gen_range` 方法是在 `rand::Rng` 这个 `trait`（类似接口interface） 中定义的，所以需要在使用 `use rand::Rng;` 将其引入
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // `{}` 一对空的大括号表示占位符，用于显示后面的参数值，可以使用多个括号对来依次捕捉多个参数值
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // `mut` 表示变量可变（变量默认是不可变的）
        // `new` 是 `String` 类的关联函数（associated function），一些语言中把它称为静态方法（static method）
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // `&mut` 表示可变引用（引用默认也是不可变的，如 `&guess` 就不可变）
            .expect("Failed to read line"); // `expect` 是 `read_line` 返回的 `io::Result` 类型的工具方法，如果结果中有错误则使用传递的参数抛出错误，如果没有错误则返回结果中包含的值

        // 新的变量 `guess` 与前面定义的变量 `guess` 同名，则前面的变量将被隐藏（shadow）
        // `guess` 后面的 `: u32` 表示为变量指定类型为无符号的 32 位整型
        // `trim` 方法可以去掉末尾的 `\n` 换行符
        // `parse` 方法和上面的 `read_line` 类似，不过是直接返回的枚举类型 `Result`，但是此处使用 `match` 表达式处理返回结果而没有用 `except` 方法，因为需要在遇到错误时继续执行程序而非抛出错误
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // `_` 是一个通配符，可以匹配所有值
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // 由于 `guess` 的类型为 `u32`，所以在这里使用 `cpm` 比较时，编译器可以推断出 `secret_number` 也是 `u32`
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
