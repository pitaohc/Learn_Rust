/*
Rc<T>引用计数智能指针
一个值有多个所有者
如图结构
如果引用计数为0，就会被自动清理

使用场景：
需要在heap上分配数据，这些数据被程序的多个部分读取，但在编译时无法确定哪个用户最后使用完这些数据

注意：
Rc<T>只能用于单线程
Rc<T>不是原子操作，不能用于并发环境
需要手动导入
Rc:clone()方法，增加引用计数
Rc::strong_count()方法，获取引用计数
Rc::weak_count()方法，获取弱引用计数

Rc<T>通过不可变引用，使用户可以在程序的不同部分共享只读数据
不可变原因：如果可变会导致数据竞争
*/
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn example_1() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn main() {
    example_1();
}
