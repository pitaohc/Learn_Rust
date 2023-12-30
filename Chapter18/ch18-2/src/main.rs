/*
可辩驳性（可失败性）
模式是否会匹配失败

rufutable 模式（可失败模式）
不一定能匹配任何传递的值的模式，可能会失败的模式
if let Some(x) = a_value

unrufutable 模式（不可失败模式）
能匹配任何可能传递的值的模式
e.g.
let x = 5;
*/
/*
函数参数、let 语句、for 循环中的模式都是不可失败的

if let 和 while let 表达式接收的模式是可失败的
*/
fn example_1() {
    println!("example_1");
    let a: Option<i32> = Some(5);
    // let Some(x) = a; // error refutable pattern in local binding [E0005] pattern `None` not covered
}

fn main() {
    println!("Hello, world!");
}
