fn main() {
    let x = 5;
    println!("{}", x);
    let y = x;
    println!("{} {}", x, y); // 此处变量 `x` 仍然有效，因为 `x` 是整型，值存储在栈上，语句 `y = x` 仍然是在栈上创建了变量 `y` 并将 `x` 的值复制（Copy）给 `y`，而 `x` 不会被影响

    let s = String::from("hello");
    println!("{}", s);

    let s2 = s;
    // println!("{}", s); // 此处变量 `s` 已经无效了，因为 `String` 类型存储在堆上，`s2 = s` 语句将 `s` 移动（Move）到了 `s2` 中，`s` 无效了
    println!("{}", s2);

    let s3 = s2.clone();
    println!("{} {}", s2, s3); // 此处变量 `s2` 仍然有效，因为 `s3` 获取的是 `s2` 的拷贝，即 `clone` 方法在堆上重新生成的一块内存，`s2` 不会被影响

    let mut s4 = world(s3);
    // println!("{}", s3); // `s3` 被移动到 `world` 函数中，此处不再有效
    println!("{}", s4);

    let len = length(&s4);
    println!("{},{}", s4, len); // 此处 `s4` 仍然有效，`length` 获取的是 `s4` 的引用，它的所有权不会被移动到函数里面

    let s5 = world2(&mut s4); // 传递可变引用给函数
    println!("{} {}", s4, s5);

    let s6 = &mut s4;
    // let s7 = &mut s4; // 不能获取多个可变引用
    // println!("{} {}", s6, s7);

    let s8 = &s4;
    // let s9 = &mut s4; // 不能在拥有不可变引用的同时拥有可变引用
    // println!("{} {}", s8, s9);

    let s10 = &s4;
    println!("{}", s10); // 所有权进入函数，这句代码之后 `s10` 就无效了
    let s11 = &mut s4; // 此处可以获取可变引用了，因为不可变引用 `s10` 在这里已经无效，也就不会冲突了
    println!("{}", s11);

    let s12 = &s4;
    // s4 = String::from("123"); // `s4` 的值在此处不能改变，因为 `s12` 获取了它的引用
    println!("{}", s12);
}

fn world(mut hello: String) -> String {
    // `hello` 进入作用域
    hello.push_str(", world!"); // `push_str()` 在字符串后追加字面值
    hello // 返回 `hello` 并移出给调用的函数
}

fn world2(hello: &mut String) -> String {
    hello.push_str(", world!");
    (*hello).clone() // 这里需要返回 `*hello` 的拷贝，不能使用 `*hello` 直接返回，因为函数并没有获取到 `hello` 的所有权，这里也可以直接简写为 `hello.clone()`
}

fn length(s: &String) -> usize {
    s.len()
}
