use std::env;
use std::fs;
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
    let config = Config::new(&args);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file\n");
    /*
    read_to_string()返回一个Result<String, io::Error>类型
    如果Result是Err，expect()会导致程序崩溃，并显示expect()的参数
    如果Result是Ok，expect()会获取Ok中的值并返回
    */
    println!("With text:\n{}", contents);
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = &args[1];
        let filename = &args[2];
        /*
        不能夺取query和filename的所有权，因为parse_config()需要返回Config实例
        to_string()会从&str创建一个String
        */
        Config { query: query.to_string(), filename: filename.to_string() }
    }
}

struct Config {
    query: String,
    filename: String,
}