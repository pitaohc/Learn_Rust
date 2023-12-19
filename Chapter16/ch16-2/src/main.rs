/*
消息传递
Channel（标准库提供）
包含发送端和接收端
调用发送端方法发送数据
接收端检查和接收到达的数据
如果发送端或接收端中任意一个被drop了，Channel就会被关闭

创建Channel
使用mpsc::channel 创建Channel
- mpsc代表多个生产者，单个消费者（multiple producer, single consumer）
- 返回一个元组：发送端和接收端
*/
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn example_1() {
    println!("example_1");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        /*
        send
        - 参数：希望发送的数据
        - 返回值：Result<T,E>
        - 成功时返回Ok(())
        - 失败时返回错误
        */
        tx.send(val).unwrap();
    });

    /*
    recv: 从通道中接收值,返回Result<T,E>,成功时返回包含值的Ok，失败时返回错误,会阻塞主线程执行直到从通道中接收一个值
    try_recv: 不会阻塞，立即返回Result<T,E>,Ok值包含可用的值，Err值表示此时没有任何消息，通常用于循环接收
    */
    let received = rx.recv().unwrap(); // recv方法会阻塞主线程执行直到从通道中接收一个值
    println!("Got: {}", received);
}

fn example_2() {
    println!("example_2");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx { //接收端作为迭代器使用，不需要显示调用recv
        println!("Got: {}", received);
    }
}
/*
通过克隆创建多个发送者
*/
fn example_3() {
    println!("example_3");
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx); //克隆发送端
    thread::spawn(move || {
        let vals = vec![
            String::from("0. hi"),
            String::from("0. from"),
            String::from("0. the"),
            String::from("0. thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("1. hi"),
            String::from("1. from"),
            String::from("1. the"),
            String::from("1. thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx { //接收端作为迭代器使用，不需要显示调用recv
        println!("Got: {}", received);
    }
}

fn main() {
    example_1();
    example_2();
    example_3();
}
