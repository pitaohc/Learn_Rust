fn example() {
    let r;
    // let b = r; // error r还没有被初始化，生命周期还没有开始
    {
        let x = 5;
        // r = &x; // error x的生命周期在这里结束，r引用的内存可能已经被释放
    }
    r = 10;
    println!("r: {}", r);
}

fn main() {
    /*
    声明周期
    Rust中每个引用都有自己的生命周期
    生命周期：让引用保持有效的作用域
    大多数情况：生命周期是隐式的、可以被推断的
    当引用的生命周期可能以不同的方式相互关联时：需要手动标注生命周期
    */

    /*
    生命周期的作用
    避免悬垂引用：引用指向的内存可能已经被释放
    */
    example();

    /*
    借用检查器
    功能：比较作用域来判断所有的借用是否合法
    */

    /*
    函数返回值中的泛型生命周期
    */
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 'a: 生命周期参数
    // x: &'a str: 参数x是一个字符串切片，它的生命周期至少与参数y的生命周期一样长
    // 'a作用：提示编译器，返回值的生命周期与参数的生命周期有关
    // 返回值的生命周期与参数的生命周期有关
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
