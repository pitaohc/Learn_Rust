/*
循环引用导致内存泄漏
例子：
使用Rc和RefCell可能创造出循环引用，从而产生内存泄漏：
- 每个项的引用数量不会变为0
*/
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    // List是一个枚举类型，它有两个变体，要么是Cons，要么是Nil
    Cons(i32, RefCell<Rc<List>>),
    // Cons包含一个i32和一个指向另一个List的Rc
    Nil, // Nil代表一个空列表
}

impl List {
    ///
    /// 获得列表的尾部
    ///
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

fn example_1() {
    println!("example_1");
    //a: 5 -> Nil
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
    // a的引用计数为1
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    // a的引用计数为2
    // b: 10 -> a
    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // b的引用计数为1
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    // b的引用计数为2
    if let Some(link) = a.tail() {
        // a的引用计数为3
        // a和b相互引用 5<->10
        *link.borrow_mut() = Rc::clone(&b);
    }
    // a和b的引用计数都为2
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // 下一行的注释将会导致栈溢出
    // println!("a next item = {:?}", a.tail());
}
/*
防止内存泄漏的方法
- 依靠开发者来保证，不依赖Rust
- 重新组织数据结构，一些引用来表达所有权，一些引用不表达所有权
类似c++的weak_ptr
Weak<T>

Rc::clone 为Rc实例的strong_count加1，Rc实例仅当strong_count为0时才会被drop
Rc::downgrade 为Rc实例创建一个Weak实例，Weak实例不会增加strong_count的值
返回类型为Weak<T>
调用Rc:downgrade 会增加weak_count的值
weak_count记录了有多少个Weak<T>指向了相同的值
weak_count的值不会影响Rc<T>指向的值是否被清理
*/
/*
强弱引用比较
强引用表达了所有权
弱引用只有对象的使用权
使用Weak Ref 不会创建循环引用
在使用Weak Ref时，需要检查对象是否存在
在Weak<T>实例上调用upgrade方法，返回Option<Rc<T>>
*/
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    /*
    Node 代表一个节点，它有一个值和一个指向其子节点的Vec
    Rc<Node> 代表一个节点的所有权
    Vec<Rc<Node>> 代表当前节点拥有的所有子节点的所有权数组
    RefCell<Vec<Rc<Node>>> 代表当前节点拥有的所有子节点的所有权数组，可以在运行时修改
    */
    parent: RefCell<Weak<Node>>,
}

fn example_2() {
    println!("example_2");
    let leaf = Rc::new(Node {
        value: 10,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });
    // leaf的引用计数为1
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    let branch = Rc::new(Node {
        value: 20,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // leaf的引用计数为2
    // branch的引用计数为1
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    // leaf的父节点是branch
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    //输出weak<T>时，不会递归输出，不用担心栈溢出
}

fn main() {
    example_1();
    example_2();
}
