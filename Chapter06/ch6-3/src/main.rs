#[derive(Debug)] // 为了可以打印
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin { // match 与 switch 类似
        Coin::Penny => { // 多行代码需要{}
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    } //匹配到的值会执行对应的代码，在此处会将表达式的值传递出来
}

fn main() {
    /*
    match 强大的控制流运算符
    允许一个值与一系列的模式相比较并根据匹配执行相应代码
    模式可以是字面值、变量、通配符、分解结构体、分解枚举、或者范围
    */
    let coin = Coin::Penny;
    println!("The value of {:?} is {}", coin, value_in_cents(&coin));
    let coin = Coin::Nickel;
    println!("The value of {:?} is {}", coin, value_in_cents(&coin));
    let coin = Coin::Dime;
    println!("The value of {:?} is {}", coin, value_in_cents(&coin));
    let coin = Coin::Quarter;
    println!("The value of {:?} is {}", coin, value_in_cents(&coin));
    /*
    绑定值的类型
    匹配的分支可以绑定到被匹配对象的部分值
    因此可以从enum中提取值
    */
    case2();
    /*
    匹配Option<T>
    */
    case3();
    /*
    match匹配必须穷举所有的可能性，强制程序员处理所有的可能，但是也可以通过'_'匹配剩余的所有情况
    思想：不让默认情况下什么都不做，而是要求程序员显式地处理所有可能的情况
    */
}
#[derive(Debug)]
enum ChState {
    Zhejiang,
    Jiangsu,
}
#[derive(Debug)]
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(ChState), // Quarter 包含了一个 ChState 类型的值, 详情见ch6-1
}
fn case2()
{

    fn value_in_cents(coin: &Coin2) -> u32 {
        match coin { // match 与 switch 类似
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("State quarter from {:?}!", &state);
                25
            }
        } //匹配到的值会执行对应的代码，在此处会将表达式的值传递出来
    }
    let coin = Coin2::Quarter(ChState::Zhejiang);
    println!("The value of {:?} is {}", &coin, value_in_cents(&coin));
}


fn case3() {
    fn plus_one(num:Option<i32>) -> Option<i32> {
        match num {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five = {:?}", five);
    println!("six = {:?}", six);
    println!("none = {:?}", none);
}