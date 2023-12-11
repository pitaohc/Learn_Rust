/*
使用Box<T>指向堆内存数据

Box<T>
允许在堆上存储数据
stack上是指向heap的指针
没有其他性能开销，也没有其他功能
*/

/*
Box<T>的使用场景
- 当有一个在编译时未知大小的类型，而又需要再确切大小的上下文中使用这个类型值的时候
- 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
- 当希望拥有一个值并只关心它的类型是否实现了特定trait而不是其具体类型时
*/
///
/// Box使用示例
fn example_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

/*
使用Box赋能递归类型（树、链表等）
在编译时，Rust需要知道所有类型的大小。
而递归类型无法在编译时确定大小
*/
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn example_box_list() {
    let list = List::Cons(1,
       Box::new(List::Cons(2,
                                              Box::new(List::Nil))));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_box() {
        example_box();
    }
    #[test]
    fn test_example_box_list() {
        example_box_list();
    }
}
