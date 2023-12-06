/*
使用闭包捕获环境
闭包可以访问定义它的作用域内的变量，而普通函数不可以
闭包捕获环境会产生额外的内存开销

闭包捕获环境的方式
与函数获得参数的三种方式相同
1. 取得所有权：FnOnce
2. 可变借用：FnMut
3. 不可变借用：Fn

创建闭包时，通过闭包堆环境值的使用，Rust自动推断出具体使用哪个trait
- 所有的闭包都实现了FnOnce
- 没有移动被捕获变量的闭包实现了FnMut
- 没有修改被捕获变量的闭包实现了Fn
范围： FnOnce > FnMut > Fn
*/
fn example_1() {
    let x = 4;
    let equal_to_x = |z| z == x; // x被捕获了
    let y = 4;
    assert!(equal_to_x(y));
}

/*
move关键字
在参数列表前使用move关键字，可以强制闭包取得它所使用的环境值的所有权
将闭包传递给新线程以移动数据所有权给新线程时，需要使用move
*/
fn example_2() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x; // x被移动了
    // println!("can't use x here: {:?}", x); // x已经被移动了，不能再使用
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
/*
最佳实践
当指定fn trait bound时，首先使用fn，基于闭包体内部的情况，如果需要fnonce或fnmut，编译器会提示
*/
fn main() {
    example_2();
}
