use std::fs::File;
use std::io::ErrorKind;

fn main() {
    /*
    Result枚举与可恢复的错误
    */
    /*
    Result定义
    Result是一个枚举类型，
    它的两个枚举量Ok和Err分别表示操作成功和操作失败，
    并且Ok枚举量中包含了一个泛型参数T，表示操作成功时返回的值的类型，
    Err枚举量中包含了一个泛型参数E，表示操作失败时返回的错误类型。

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){ // 不存在则创建
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },

    };
    /*
    可以看出使用match表达式来处理Result类型的值是非常繁琐的，
    因为我们需要处理各种可能的情况，而且还需要处理嵌套的match表达式，
    这样的代码是非常难以阅读和维护的。
    可以使用闭包(closure)来简化代码，闭包是一种可以保存进变量或作为参数传递给其他函数的匿名函数。
    */
    /*
    unwrap：match表达式的一个快捷方法
    */
    let f = File::open("hello.txt").unwrap(); //Err时会自动调用panic!，缺点是无法定制错误信息

    /*
    expect
    是unwrap的改进版，可以自定义错误信息
    */
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); //Err时会自动调用panic!，缺点是无法定制错误信息
}
