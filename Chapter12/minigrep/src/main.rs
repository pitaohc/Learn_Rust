use std::{env, process};
use minigrep::Config;
/*
二进制程序关注点分离的指导性原则
将程序拆分为main.rs和lib.rs并将程序的逻辑放入lib.rs中
当命令行解析逻辑较少时，可以将解析逻辑放入main.rs中
当命令行解析逻辑较多时，需要将解析逻辑放入lib.rs中
*/
/*
经过上述拆分，留在main.rs中的代码有：
- 使用参数值调用命令行解析逻辑
- 调用lib.rs中的run函数
- 处理run函数可能出现的错误
*/
fn main() {
    let args: Vec<String> = env::args().collect(); // 获取命令行参数
    /*
    args()返回一个迭代器
    collect()将迭代器转换为一个集合
    */
    /*
    args 无法处理非法的Unicode字符
    如果需要处理非法的Unicode字符，可以使用std::env::args_os // OsString
    */

    /*
    unwrap_or_else()是Result类型的一个方法
    如果Result是Ok，unwrap_or_else()会获取Ok中的值并返回
    如果Result是Err，unwrap_or_else()会调用闭包并将Err中的值作为参数传递给闭包
    */
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1); // 退出程序并返回状态码1
    });

    if let Err(e) = minigrep::run(config) {
        /*
        if let 功能类似于match，但是只匹配一个模式
        此处这么使用因为run如果成功返回的是一个空元组，如果失败返回的是一个Box<dyn Error>
        */
        println!("Application error: {}", e);
        process::exit(1);
    }
}



