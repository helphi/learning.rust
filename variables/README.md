# variables

- Rust 有四种基本的标量（scalar）类型：整型、浮点型、布尔类型和字符类型。
- Rust 有两个原生的复合类型（Compound types）：元组（tuple）和数组（array）。

## 整型

长度 | 有符号 | 无符号
----|-------|-------
8-bit | i8 | u8
16-bit | i16 | u16
32-bit | i32 | u32
64-bit | i64 | u64
128-bit | i128 | u128
arch | isize | usize

数字字面值 | 例子
---------|-----
Decimal | 98_222
Hex | 0xff
Octal | 0o77
Binary | 0b1111_0000
Byte (u8 only) | b'A'

- 数字类型默认是 `i32`。
- `isize` 和 `usize` 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的，主要作为某些集合的索引，比如数组索引。

## 浮点型

Rust 的浮点数类型是 `f32` 和 `f64`，分别占 32 位和 64 位。默认类型是 `f64`。

## 布尔型

Rust 中的布尔类型有两个可能的值：`true` 和 `false`。

## 字符类型

Rust 的 char 类型的大小为四个字节(four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容。在 Rust 中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。Unicode 标量值包含从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值。
