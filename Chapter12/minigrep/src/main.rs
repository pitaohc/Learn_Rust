use std::env; // 引入标准库中的env模块

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
    // println!("{:?}", args); // 打印命令行参数

    let query = &args[1];
    let filename = &args[2];
    println!("Searching for \"{}\"", query);
    println!("In file \"{}\"", filename);
}
