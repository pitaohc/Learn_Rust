/*
匹配字面值
*/
fn example_1() {
    println!("example_1");
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }
}
/*
匹配命名变量
*/
fn example_2() {
    println!("example_2");
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // 命名变量作为匹配模式, 会覆盖掉外部的y
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
}
/*
多重模式
| 运算符可以将多个模式组合成一个分支
*/
fn example_3() {
    println!("example_3");
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("other"),
    }
}
/*
使用..=匹配值的范围
*/
fn example_4() {
    println!("example_4");
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("other"),
    }
    let y = 'c';
    match y {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("other"),
    }
}
/*
解构以分解值
*/
fn example_5() {
    println!("example_5");
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // 解构结构体
    assert_eq!(0, a);
    assert_eq!(7, b);
    // assert_eq!(0, x); // 无法访问x, 因为x是p的字段
    let Point { x, y } = p; // 解构结构体
    assert_eq!(0, x);
    assert_eq!(7, y);
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({},{})", x, y),
    }
}
/*
解构枚举 enum
*/
fn example_6() {
    println!("example_6");
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}
/*
解构嵌套的结构体和枚举
*/
fn example_7() {
    println!("example_7");
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color), // 嵌套枚举
    }
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        }
        _ => (),
    }
}
/*
解构结构体和元组
*/
fn example_8() {
    println!("example_8");
    struct Point {
        x: i32,
        y: i32,
    }
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    // 外部是一个大元组 类型为((i32, i32), Point)
    // 第一个元素是一个小元组 类型为(i32, i32)
    // 第二个元素是一个结构体 类型为Point
    println!("feet = {}, inches = {}, x = {}, y = {}", feet, inches, x, y);
}
/*
在模式中忽略值
- _
- .. 忽略值的剩余部分
*/
fn example_9() {
    println!("example_9");
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => { // 忽略了Some中的值
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => { // 忽略了second和fourth
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

fn example_10() {
    println!("example_10");
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    match black {
        Color(r, ..) => println!("r = {}", r),
    }

    match black{
        Color(.., b) => println!("b = {}", b),
    }

    // match black{
    //     Color(..,g,..) => println!("g = {}", g), // error 无法确定位置
    // }
}
/*
使用match守卫提供额外的条件
match守卫是match分支模式后额外的if条件，它们用于指定应该匹配的值的更多细节
*/
fn example_11(){
    println!("example_11");
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), // match守卫
        Some(x) => println!("{}", x),
        None => (),
    }

}
/*
@绑定
功能：创建一个变量来保存match值的一部分，同时测试匹配的值是否符合绑定的模式
*/
fn example_12(){
    println!("example_12");
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => { // @绑定, 将值存入id_variable
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
    example_5();
    example_6();
    example_7();
    example_8();
    example_9();
    example_10();
    example_11();
}
