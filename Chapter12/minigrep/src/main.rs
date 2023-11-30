use std::env;
use std::fs;

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
    println!("{:?}", args); // 打印命令行参数

    let query = &args[1];
    let filename = &args[2];
    println!("Searching for \"{}\"", query);
    println!("In file \"{}\"", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file\n");
    /*
    read_to_string()返回一个Result<String, io::Error>类型
    如果Result是Err，expect()会导致程序崩溃，并显示expect()的参数
    如果Result是Ok，expect()会获取Ok中的值并返回
    */
    println!("With text:\n{}", contents);
}
