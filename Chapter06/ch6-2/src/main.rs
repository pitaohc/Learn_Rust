fn main() {
    /*
    Option 枚举
    - 定义于标准库中
    - 在Prelude(预导入模块)中
    - 描述了一个值可能存在或不存在的情况
    */

    /*
    NULL
    Rust中没有NULL
    RUst提供类似NULL的功能的是Option<T>
    enum Option<T> {
        Some(T),
        None,
    }
    这样设计的优势
    Option<T>和T是不同的类型，不可以把Option<T>类型的值当成T类型的值使用
    如果想要使用Option<T>类型的值，必须显式地处理Some和None的情况
    在编译阶段就能检查出错误，而不用在运行时判断对象是否是NULL
    */

    let some_number = Some(5); // Some(5)是一个Option<i32>
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; // None必须声明类型,因为编译器无法推断出变量类型

    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);
}
