/*
Send和Sync

Rust语言的并发特性少，目前的并发特性都来自于标准库
其实无需局限于标准库，可以自己实现并发特性

Send
允许线程间转移所有权
实现Send的类型可以在线程间转移所有权
几乎所有类型都实现了Send，除了裸指针（*const T, *mut T）和部分同步类型（Rc<T>, RefCell<T>）
任何完全由Send类型组成的类型也被标记为Send

Sync
允许多线程同时访问
实现Sync的类型可以安全的在多个线程中拥有其值的引用
如果T是Sync,那么&T（T的引用）就是Send
引用可以被安全的送往另一个线程
基础类型都是Sync
完全由Sync类型组成的类型也是Sync
- Rc<T>非Sync
- RefCell<T>和Cell<T>非Sync
- Mutex<T>是Sync

手动实现Send和Sync是不安全的
*/


fn main() {
    println!("Hello, world!");
}
