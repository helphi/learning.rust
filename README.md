# rust

## 安装

> - <https://forge.rust-lang.org/infra/other-installation-methods.html>
> - <http://mirrors.ustc.edu.cn/help/crates.io-index.html>
> - <http://mirrors.ustc.edu.cn/help/rust-static.html>

on Ubuntu 18.04

```bash
#设置rustup安装其他组件的源，此处使用科大源
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

#安装rustup
wget http://mirrors.ustc.edu.cn/rust-static/rustup/dist/x86_64-unknown-linux-gnu/rustup-init
chmod +x rustup-init
./rustup-init -y #会自动安装其他组件，如rustc、cargo等

#设置cargo下载包的源
cat <<EOF > ~/.cargo/config
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
EOF
```
