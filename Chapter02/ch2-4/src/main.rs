use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数！！！");
    println!("请输入你猜的数：");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //loop关键字表示无限循环
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //Ok(num)是模式匹配，如果parse()函数返回Ok，将Ok(num)的num赋值给guess
            Err(_) => {
                println!("输入不是数字，请重新输入！");
                continue;
            } //_ 是通配符，表示忽略所有的错误， continue表示跳过本次循环
        };
        println!("你猜测的数是：{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("太小了！") }
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
            Ordering::Greater => { println!("太大了！") }
        };
    }
}
