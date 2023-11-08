use std::io;
use rand::Rng;
use std::cmp::Ordering; //Ordering是枚举类型，Less、Greater、Equal是枚举值

fn main() {
    println!("猜数！！！");
    println!("请输入你猜的数：");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..101); //整形的默认是i32
    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数是：{}", guess);
    println!("随机数是：{}", secret_number);

    //trim()函数是String类型的方法，去掉字符串首尾的空白字符
    //parse()函数是String类型的方法，将字符串转换成数字，返回Result类型
    //expect()函数是Result类型的方法之一，如果parse()函数返回Err，expect()函数会抛出一个异常
    //guess:u32 表示guess是一个u32类型的变量，显式声明
    //此处复用guess变量，将String类型的guess变量转换成u32类型的guess变量
    let guess: u32 = guess.trim().parse().expect("输入非法，请输入一个数字");

    //match表达式，类似于switch语句
    //cmp()函数是String类型的方法，返回一个Ordering类型的值
    //Ordering是枚举类型，Less、Greater、Equal是枚举值
    //Ordering::Less表示guess小于secret_number
    //Ordering::Greater表示guess大于secret_number
    //Ordering::Equal表示guess等于secret_number
    // => 表示执行后面的语句
    match guess.cmp(&secret_number) {
        Ordering::Less => { println!("太小了！") }
        Ordering::Equal => { println!("猜对了！") }
        Ordering::Greater => { println!("太大了！") }
    };
}
