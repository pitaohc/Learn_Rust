fn main() {
    /*
    String类型

    Rust开发者常见String疑惑
    - Rust倾向于暴露可能的错误
    - 字符串数据结构复杂
    - UTF-8
    */

    /*
    字符串是什么
    Byte集合+一些将byte解析为文本的方法
    */

    /*
    字符串是什么
    - Rust的核心语言层，只有一个字符串类型：字符串切片str或&str
    - 字符串切片：对存储在其他地方、UTF8编码的字符串的引用
    - 字符串字面值：存储在二进制文件中，也是字符串切片
    */

    /*
    String类型：
    - 来自标准库
    - 可增长、可修改、可拥有
    - UTF8
    */

    /*
    其他类型的字符串：OsString、CString、CStr、OsStr
    - String vs Str : 拥有或借用的变体
    - 可存储不同编码的文本或在内存中以不同的形式展现
    */

    /*
    创建字符串
    - String::new()
    - String::from()
    - to_string()
    */
    let mut s = String::new(); // 创建空字符串
    s.push('H');
    s.push_str("ello");
    println!("{}", s);

    let s = String::from("Hello"); // 从字符串字面值创建
    println!("{}", s);
    let s = "hello".to_string(); // 从字符串字面值创建

    /*
    修改string
    push
    push_str 把一个字符串切片附加到string后
    */

    /*
    拼接字符串
    - 使用+操作符 类似 fn add(self, s: &str) -> String { ... }
    - 使用format!宏
    */
    let s1 = String::from("Hello ");
    let s2 = String::from("World");
    // let s3 = s1 + s2; //error 不能直接使用+操作符
    // let s3 = &s1 + &s2; // error cannot be used to concatenate two `&str` strings
    let s3 = s1 + &s2; // 使用&强制转换为&str 问题 s1丢失所有权，后续报错
    println!("{}", s3);

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = format!("{} {}", s1, s2); // format!宏 s1 s2保持所有权
    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
}
