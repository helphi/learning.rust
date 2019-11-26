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

fn hello() -> &'static str { // 由于函数返回的是一个静态字符串，所以这里要用 `&'static str` 类型返回，意味着这个字符串从程序开始到结束一直在内存中，也就是您一直可以访问到它，而不用担心作用域的问题。
    "hello" // 结尾没有分号，则它是一个表达式，直接返回值，相当于语句 `return "hello";`
}