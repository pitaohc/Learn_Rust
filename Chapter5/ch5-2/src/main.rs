#[derive(Debug)] // 为了能够使用println!打印结构体,#表示注解,derive表示从其他类型派生，Debug表示Debug类
struct Rectangle(u32,u32);
fn main() {
    let w = 30;
    let h  = 50;
    println!("area: {}",area0(w,h)); //问题：w和h不相关，但是必须一起传递
    let rect = (30,50);
    //改进方法：使用元组
    //问题：rect是什么意思？不清楚
    println!("get area with tuple: {}",area1(rect));

    //改进方法：使用结构体

    let rect2 = Rectangle(30,50);
    println!("get area with struct: {}",area2(&rect2));
    /*
    std::fmt::Display trait用于更加用户友好的输出
    std::fmt::Debug trait用于更加**调试**友好的输出
    #[derive(Debug)] 注解可以自动实现Debug trait
    {:?} 功能：打印调试信息
    {:#?} 功能：打印调试信息，带有缩进和换行
    */
    println!("{:?}",rect2);
    println!("{:#?}",rect2);
}

fn area0(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dim:(u32,u32))->u32{
    dim.0 * dim.1
}

fn area2(rect:&Rectangle)->u32{
    rect.0 * rect.1
}