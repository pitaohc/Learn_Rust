/*
模式
用于匹配简单和复杂类型的结构
将模式与匹配表达式和其他构造结合使用，可以更好地控制程序

模式的组成：
- 字面值
- 解构的数组、枚举、结构体或者元组
- 变量
- 通配符
- 占位符

模式的使用：
将模式与值进行比较
- 匹配：在代码中使用这个值的相应部分
*/
/*
模式引用场景
- match 分支
- if let 条件表达式
*/
fn example_1() {
    println!("example_1");
    let favorite_color: Option<&str> = Some("hello");
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    // 使用模式匹配
    if let Some(color) = favorite_color { //将favorite_color的值与Some(color)模式进行比较
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // 使用模式匹配
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        // 使用模式匹配
        println!("Using blue as the background color");
    }
}
/*
while let
- 只要模式匹配就一直执行代码块
*/
fn example_2() {
    println!("example_2");
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // 使用while let
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
/*
for 循环
for循环中，模式就是紧随for关键字后的值
*/
fn example_3() {
    println!("example_3");
    let v = vec!['a', 'b', 'c'];
    // 使用for循环
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
/*
let 语句
let Pattern = Expression;
*/
fn example_4() {
    println!("example_4");
    let (x, y, z) = (1, 2, 3);
    println!("x = {}, y = {}, z = {}", x, y, z);
    // let (a,b) = (1,2,3); // 编译报错，左右两边的元素数量不一致
}
/*
函数参数
*/
fn example_5() {
    println!("example_5");
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);
}

fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
    example_5();
}
