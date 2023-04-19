fn main() {
    let x: u16 = 5;
    let mut x: u32 = (x + 5).into(); // 此处的变量 `x` 隐藏（shadow）了上面的变量 `x`，且使用 `mut` 关键字将其变成了可变的
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MIN_POINTS: u32 = 10_000;
    println!("{} {}", MIN_POINTS, MAX_POINTS);

    // 数字字面值允许使用 `_` 做为分隔符以方便读数
    let x = 1_000;
    println!("{}", x);

    // 除 `byte` 以外的所有数字字面值允许使用类型后缀
    let x = 1_000u32;
    println!("{}", x);

    let x = 'z';
    println!("{}", x);

    let x = 'ℤ';
    println!("{}", x);

    // 将表情存储在 char 中
    let x = '😻';
    println!("{}", x);

    // 将中文存储在 char 中
    let x = '中';
    println!("{}", x);

    // 元组长度固定：一旦声明，其长度不会增大或缩小
    // 元组中的每一个位置都有一个类型，而且这些类型也不必是相同的
    let x = (500, 6.4, 1);
    println!("The tup values is: {}, {}, {}", x.0, x.1, x.2);

    let x: (i32, f64, u8) = (500, 6.4, 2);
    println!("The tup values is: {}, {}, {}", x.0, x.1, x.2);

    let (x, y, z) = x; // 解构（destructure）元组
    println!("The tup values is: {}, {}, {}", x, y, z);

    //  Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小
    let x = [1, 2, 3, 4, 5];
    println!("{}", x[0]);

    let x: [i32; 5] = [1, 2, 3, 1, 1]; // `i32` 表示类型，`5` 表示长度
    // 遍历数组
    for v in x.iter().rev() { // `iter` 获取一个迭代器，`rev` 倒序
        println!("{}", v);
    }

    let x = [1; 5]; // `1` 表示数组里面存储的值，`5` 表示存储5个1
    println!("{}", x[0]);
}

// 常量必须指定类型
// 常量可以在任何作用域中声明，包括全局作用域
// 常量的命名规范是使用下划线分隔的大写字母单词
// 常量只能被设置为常量表达式，而不能是函数调用的结果，或任何其他只能在运行时计算出的值
const MAX_POINTS: u32 = 100_000;
