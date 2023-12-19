/*
Rust支持通过共享状态来实现并发
Channel类似单所有权，一旦将值发送，发送端就无法再使用这个值
共享内存类似多所有权，多个线程可以同时访问同一块内存

使用Mutex来允许单个线程访问数据
在同一时刻Mutex只允许一个线程访问指定的数据

访问数据流程：
- 获得锁
- 访问数据
- 释放锁
*/
/*
Mutex<T> API
创建锁：Mutex:new(数据)
获取锁：lock() -> Result<MutexGuard<T>, PoisonError<MutexGuard<T>>>
    会阻塞当前线程
    lock可能会失败
解锁：drop(MutexGuard<T>)
*/
use std::sync::{Arc, Mutex};
use std::thread;

fn example_1() {
    println!("example_1");
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
        //num走出作用域，自动解锁
    }
    println!("m = {:?}", m);
}
/*
多线程共享的例子
*/
/*fn example_2(){
    println!("example_2");
    let counter = Mutex::new(0);
    let mut handles = vec![];
    for _ in 0..10{
        let handle = thread::spawn(move || { // error 所有权已经被前一个线程move了
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap()); //error 所有权已经被子线程move
}*/
/*
使用Arc<T>来在多个线程间共享所有权
Rc<T>无法在多线程环境下使用，因为无法保证线程安全，没有实现send trait
A代表Atomic，原子类型，可以跨线程安全共享
Arc需要额外开销
*/
fn example_3(){
    println!("example_3");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || { // error 所有权已经被前一个线程move了

            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap()); //error 所有权已经被子线程move
}
/*
RefCell/Rc和Mutex/Arc的区别
- Mutex提供了内部可变性，和Cell系列类似
- 使用RefCell来改变Rc里面的内容
- 使用Mutex来改变Arc里面的内容
- Mutex有死锁的风险
*/
fn main() {
    example_1();
    //example_2();
    example_3();
}
