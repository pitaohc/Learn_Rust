/*
从crates.io安装二进制crate

cargo install <crate-name>
来源: https://crates.io
限制：只能安装具有二进制目标的crate
二进制目标crate是一个可运行的程序
拥有src/main.rs或者其他被指定为二进制文件的crate

默认安装到~/.cargo/bin
*/
/*
使用自定义命令扩展cargo
cargo被设计为可使用子命令扩展

如：
$PATH下有个二进制文件cargo-hello
则可运行cargo hello命令来执行该文件
可以使用cargo --list查看所有可用的子命令
*/
fn main() {
    println!("Hello, world!");
}
