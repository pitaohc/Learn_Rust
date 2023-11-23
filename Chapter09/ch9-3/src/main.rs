use std::fs::File;
use std::io;
use std::io::Read;
use std::error::Error;
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // 返回Err，让调用者处理
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } //没有; 代表返回值
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; //?可以直接返回Err，让调用者处理
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s) //没有; 代表返回值
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s) //没有; 代表返回值
}

fn main() -> Result<(), Box<dyn Error>>{
    /*
    传播错误
    抛出Err，让调用者处理
    */
    let s = read_username_from_file();
    let s = match s {
        Ok(s) => s,
        Err(e) => panic!("Error: {:?}", e)
    };
    println!("s: {}", s);
    /*
    ?运算符
    简化传播错误的代码
    相当于在Err的match分支中，直接return错误
    - 如果是Ok，表达式的值就是Ok的值
    - 如果是Err, Err就是当前函数的返回值，就像使用了return
    */
    read_username_from_file2();
    /*
    ?与from
    Trait std::convert::From
    - 错误类型的转换
    - ?操作符会自动调用from函数处理
    - 当?调用from函数时，?得到的类型会自动转换成成函数返回类型中的错误类型
    - 用途：捕获不同的错误类型，返回同一种错误类型
    - 条件：每个错误类型实现了转换为所返回的错误类型的from函数
    */

    /*
    ?与链式调用
    */
    read_username_from_file3();

    /*
    如何在main函数中使用?
    将main的签名改为
    fn main() -> Result<(), Box<dyn Error>> {
        ...
    }
    Box<dyn Error>是一个trait对象，表示任何错误类型
    */
    let f = File::open("hello.txt")?;
    Ok(())
}
