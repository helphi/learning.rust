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

fn hello() -> &'static str {
    let x = "hello";
    println!("{}", x);

    // "Hello" 是一个字符串字面值，字符串值被硬编码进程序里。
    // 字符串字面值的类型为 `&'static str`，可以不用显示指定，编译器可以自动推导，
    // 这个类型表示这个字符串从程序开始到结束一直在内存中，也就是您一直可以访问到它，而不用担心作用域的问题，因为它的作用域被指定为 `'static`
    let x: &'static str = "Hello";
    
    x // 结尾没有分号，则它是一个表达式，直接返回值，相当于语句 `return x;`
}