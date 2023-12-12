/*
隐式解引用转换 Deref Coercion

在编译时完成
*/
use std::ops::Deref;

fn print(info: &str) {
    println!("hello {}", info);
}
struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> std::ops::Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn example_1() {
    let x = "&str"; // &str OK
    print(&x);
    let y = String::from("String"); // String OK
    print(&y);
    let z = MyBox::new(String::from("MyBox")); // MyBox OK
    /*
    z MyBox<String>
    &z 调用MyBox的Deref Trait 得到 &MyBox<String>
    &MyBox<String> 调用Deref Trait 得到 &String
    &String 调用Deref Trait 得到 &str
    */
    print(&z);
    print(&(*z)[..]); // 等价于 print(&z);
    /*
    *z 调用MyBox的Deref Trait 得到 String
    &(*z) 调用String的Deref Trait 得到 &String
    &String 调用Deref Trait 得到 &str
    */
}
/*
解引用与可变性
使用DerefMut trait重载可变引用的*运算符
在类型和trait为下列三种情况时Rust会执行deref coercion
- 当T: Deref<Target=U>时从&T到&U, 不可变引用转换为不可变引用
- 当T: DerefMut<Target=U>时从&mut T到&mut U, 可变引用转换为可变引用
- 当T: Deref<Target=U>时从&mut T到&U, 可变引用转换为不可变引用

不允许不可变引用转换为可变引用
*/
fn main() {
    example_1();
}
