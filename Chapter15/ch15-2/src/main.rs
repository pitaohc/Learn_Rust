/*
Deref Trait
实现了 Deref trait 的类型可以被当作指针来解引用，通过使用*运算符。
*/
/*
解引用运算符*
*/
fn example_1() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn example_2() {
    let x = 5;
    let mut y = Box::new(x); // 拷贝x到堆上，y指向堆上的5，而不是x
    *y = 6;
    assert_eq!(5, x);
    assert_eq!(6, *y);
}
/*
自定义智能指针
Box<T>被定义成拥有一个元素的tuple struct
*/
struct MyBox<T>(T);

//该元组仅有1个元素
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T; // 定义关联类型，用于指定Deref trait的关联类型
    fn deref(&self) -> &T {
        &self.0 //返回元组中的唯一元素的引用
    }
}
fn example_3() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); //*y 等价于 *(y.deref())
}
fn main() {
    example_1();
    example_2();
    example_3();
}
