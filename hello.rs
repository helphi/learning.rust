/// 以三个斜杠开头的注释为文档注释，支持 Markdown，会生成 HTML 文档
/// 使用以下指令运行程序
///
/// ```bash
/// rustc main.rs
/// ./main
/// ```
fn main() {
    let x = hello();

    let y = {
        "word" // 结尾没有分号，则它是一个表达式，会返回值给变量 y
    };

    println!("{} {}", x, y);
}

fn hello() -> &'static str { // 此处使用 `'static` 指明 `&str` 类型的返回值的生命周期为永久，因为返回的字符串字面值是在函数里面定义的，不指定生命周期则会在函数返回后失效
    let x = "hello";
    println!("{}", x);

    // "Hello" 是一个字符串字面值，字符串值被硬编码进程序里
    // 字符串字面值的类型为 `&str`，可以不用显示指定，编译器可以自动推导
    let x: &str = "Hello";
    
    x // 结尾没有分号，则它是一个表达式，直接返回值，相当于语句 `return x;`
}