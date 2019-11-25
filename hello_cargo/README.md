# hello_cargo

## 创建项目

```bash
cargo new hello_cargo
cd hello_cargo
```

## 编译并手动运行

```bash
cargo build
./target/debug/hello_cargo
```

## 编译并运行

```bash
cargo run
```

## 检查代码

```bash
cargo check
```

## 发布构建

会在 target/release 而不是 target/debug 下生成可执行文件

```bash
cargo build --release
```
