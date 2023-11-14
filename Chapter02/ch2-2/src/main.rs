use std::io;
//use导入库 std标准库 io库
use rand::Rng;
//导入rand库的Rng trait

fn main() { // fn 函数体  main主函数 ()无参数
    println!("猜数！！！"); //带！表示println是一个宏
    println!("请输入你猜的数：");

    //mut 修饰的对象是可变的 string 默认utf-8
    //String::new() 构造一个String类型的对象， '::'表示是类函数而不是对象函数，或者称作静态方法
    let mut guess = String::new();

    //thread_rng()返回一个随机数生成器
    //gen_range()函数是Rng trait的方法，gen_range()函数的参数是一个范围，返回一个范围内的随机数
    let secret_number = rand::thread_rng().gen_range(1..101);

    //调用io库的stdin()函数，stdin()返回一个Stdin类型的对象
    //调用Stdin对象的read_line()函数读取一行
    //read_line()传入参数buffer
    //&mut 表示引用，引用是不可变的，但是可以通过mut修饰的引用来改变引用的值
    //read_line()返回io::Result类型，Result是枚举类型，Ok表示成功，Err表示失败
    //expect()函数是io::Result类型方法之一，如果read_line()函数返回Err，expect()函数会抛出一个异常
    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数是：{}", guess);
    println!("随机数是：{}", secret_number);
}
