# guessing_name

## 更新依赖

当Cargo.toml中的dependencies内容发生变化时可使用 `cargo update` 更新依赖，直接调用 `cargo build` 或者 `cargo run` 也会自动更新依赖

```bash
cargo update
```

## 查看文档

构建所有本地依赖提供的文档，并在浏览器中打开

```bash
cargo doc --open
```
