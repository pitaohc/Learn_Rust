/*
迭代器
对一系列项执行某个操作
迭代器功能
- 遍历每个项
- 确定遍历何时完成
*/

/*
Rust迭代器
懒惰的：除非调用消费迭代器的方法，否则迭代器本身没有任何效果
*/
fn example_1() {
    println!("example_1");
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // 此时没有使用，不会有任何效果
    for val in v1_iter { // 此时才会使用
        println!("Got: {}", val);
    }
}

/*
Iterator trait
所有迭代器都实现了Iterator trait
定义于标准库
pub trait Iterator{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
type Item 和 Self::Item 定义了与该trait关联的类型
实现Iterator trait的类型都必须指定Item类型，且next方法返回Option<Self::Item>

仅要求实现next方法
每次调用都会返回迭代器的一个元素，返回值包裹在Some里，迭代结束返回None
可以直接在迭代器上调用next方法
*/
fn example_2(){
    println!("example_2");
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1)); // 返回Some(&1)
    assert_eq!(v1_iter.next(), Some(&2)); // 返回Some(&2)
    assert_eq!(v1_iter.next(), Some(&3)); // 返回Some(&3)
    assert_eq!(v1_iter.next(), None); // 返回None
}
/*
迭代方法
iter：获取不可变引用
into_iter：获取所有权
iter_mut：获取可变引用
*/
fn main() {
    example_1();
    example_2();
}
