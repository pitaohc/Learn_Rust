fn main() {
    /*
    使用fn关键字声明函数
    对于函数和变量名，Rust使用snake case命名规范：
    - 所有字母都是小写的
    - 单词之间使用下划线分隔
    Rust不关心函数声明的位置
    */
    another_function();

    /*
    函数的参数
    parameters 形参
    arguments 实参
    函数签名中必须声明参数类型
    */
    print_number(100);

    /*
    函数体中的语句(statement)和表达式(expression)
    语句：执行一些操作，但不返回值
    表达式：计算并产生一个值
    */
    let y = {
        let x = 3;
        x + 1 // 表达式不需要分号结尾
    }; //y = 4
    println!("y is: {}", y);
    // let y = {
    //     let x = 3;
    //     x + 1; // 表达式不需要分号结尾
    // }; //y = ()
    // println!("y is: {}", y);

    /*
    函数的返回值
    使用->指定返回值类型
    返回值是函数体中最后一个表达式的值
    如果想要提前返回，需要使用return关键字
    */
    let result = add_one(5);
    println!("result is: {}", result);
}

fn another_function() {
    println!("Another function.");
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}