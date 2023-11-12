fn main() {
    /*
    引用和借用
    功能：允许使用值但不获取其所有权
    使用方式：&符号和*符号
    引用:引用某些值而不获取其所有权，&str

    与C++的对比
    C++默认传递是复制
    Rust默认传递是移动

    借用：把引用作为函数参数的行为
    借用时不允许修改
    */
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    fn calculate_length(s: &String) -> usize {
        //此处的s是一个指向调用者的引用
        s.len()
        //可以理解为函数结束时，s（一个指针）被销毁，而s指向的东西不会被销毁
    }
    println!("The length of '{}' is {}.", s1, len);


    /*
    可变引用
    限制：在特定作用域内，对同一块数据，只能有一个可变引用或多个不可变引用
    作用：在编译器层面防止数据竞争

    一下三种情况会发生数据竞争：
    1.两个或更多指针同时访问同一数据
    2.至少有一个指针被用来写入数据
    3.没有同步数据访问的机制

    可以通过创建新的作用域，来运行非同时的多个可变引用
    */
    let mut s2 = String::from("hello");
    println!("original s2 = {}", s2);
    change(&mut s2);
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    println!("changed s2 = {}", s2);
    let s22 = &mut s2;
    // let s23 = &mut s2; //error  cannot borrow `s2` as mutable more than once at a time
    println!("s22 = {}", s22);
    // println!("s23 = {}", s23);
    let s24 = &s2;
    println!("s24 = {}", s24);

    /*
    可变引用和不可变引用不能同时存在的例子
    */
    {
        let mut s3 = String::from("hello");
        let r1 = &s3;
        let r2 = &s3;
        // let r3 = &mut s3; // error cannot borrow `s3` as mutable because it is also borrowed as immutable
        // println!("{}, {}, and {}", r1, r2, r3); //error
        println!("{}, and {}", r1, r2);
    }
    {
        let mut s3 = String::from("hello");
        let r1 = &mut s3;
        // let r2 = &s3; // error cannot borrow `s3` as immutable because it is also borrowed as mutable
        // let r3 = &s3; // error cannot borrow `s3` as immutable because it is also borrowed as mutable
        // println!("{}, {}, and {}", r1, r2, r3); // error
        println!("{}", r1);
    }

    /*
    悬空引用 dangling reference
    指针引用了已经被释放的内存
    Rust编译器确保不会出现悬空引用
    e.g.
    r = dangle();
    fn dangle() -> &String{
        let s = String::from("hello");
        &s
    }
    */

    /*
    引用的规则：
    - 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
    - 引用必须总是有效的
    */
}
