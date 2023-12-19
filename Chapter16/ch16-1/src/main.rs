/*
并发
Concurrent: 程序的不同部分独立的执行
Parallel: 程序的不同部分同时执行

Rust无畏并发：允许编写没有细微Bug的代码，并在不引入新Bug的情况下易于重构

本课程的并发：泛指Concurrent和Parallel
*/
/*
进程和线程
OS管理进程
进程是操作系统分配资源的基本单位
一个进程中可以有多个线程
线程是OS调度的基本单位
线程可以同时执行
*/
/*
多线程可能导致的问题
- 竞争状态：多个线程同时访问同一数据
- 死锁：两个线程相互等待对方停止使用其所拥有的资源，导致两个线程都无法继续执行
只在特定情况下产生Bug，且很难定位和修复
*/
/*
线程的实现方式：
- 调用OS的API来创建线程：1:1模型
    需要较小的运行时
- 在运行时实现线程：M:N模型
    更灵活，但需要较大的运行时

Rust：需要权衡运行时支持
Rust仅提供1:1模型
如果需要M:N模型，可以使用第三方库
*/
/*
通过spawn创建新线程
thread::spawn函数来创建新线程
- 参数：一个闭包
*/
/*
通过join handle等待所有线程结束
thread::spawn返回一个JoinHandle
JoinHandle是一个拥有所有权的值，当对其调用join方法时，会阻塞当前线程直到被调用线程结束
join方法返回一个Result，线程正常结束时，其值为Ok，包含线程的返回值
*/
use std::thread;
use std::time::Duration;

fn example_1() {
    println!("example_1");
    let task = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    task.join().unwrap(); // 等待线程结束
}

/*
使用move闭包
move闭包通常和thread::spawn一起使用，以便将数据移动到新线程中
move会将闭包所使用的变量的所有权转移给新线程，主线程就无法再使用这些变量
*/
fn example_2() {
    println!("example_2");
    let v = vec![1, 2, 3];
    // 此处借用的v的生命周期可能比线程短，所以编译器报错
    // let task = thread::spawn( || {
    //     println!("Here's a vector: {:?}", v);
    // });
    let task = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v); // error,所有权已经移动到闭包内，此处无法释放v的所有权
    task.join().unwrap();
}

fn main() {
    example_1();
    example_2();
}
