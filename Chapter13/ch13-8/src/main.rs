/*
创建自定义迭代器
需要实现next方法
*/
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
#[test]
fn calling_next_directly(){
    let mut counter = Counter::new();
    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);
}
#[test]
fn using_other_iterator_trait_methods(){
    /*
    Counter::new() 创建一个新的Counter实例，内部的count为0
    zip() 创建一个新的迭代器，它将两个迭代器作为参数，并将两个迭代器的元素配对为一个元组
    skip() 创建一个新的迭代器，它将一个迭代器作为参数，并跳过前n个元素
    zip当两个迭代器长度不一致时，会自动停止
    map() 创建一个新的迭代器，它接收一个闭包作为参数，该闭包会调用每个元素，并返回一个新的迭代器
    当前的这个闭包返回的是两个元素的乘积 (a,b)是一个元组
    filter() 创建一个新的迭代器，它接收一个闭包作为参数，该闭包会调用每个元素，并返回一个布尔值
    sum() 对迭代器中的每个元素求和
    */
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a,b)| a*b)
        .filter(|x| x%3 == 0)
        .sum();
    assert_eq!(18,sum);
}

fn main() {
    println!("Hello, world!");
}
