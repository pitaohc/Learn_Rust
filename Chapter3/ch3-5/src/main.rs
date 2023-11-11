fn main() {
    /*
    if表达式
    允许根据条件执行不同的代码分支，这个条件必须是bool类型
    表达式中，与条件相关联的代码块叫做分支(arm)
    */

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /*
    多分支：else if
    */
    if number == 0 {
        println!("number was zero");
    } else if number == 1 {
        println!("number was one");
    } else {
        println!("number was something else");
    }

    /*
    如果使用了超过3个分支的if，则最好使用match替换
    */
    match number {
        0 => println!("number was zero"),
        1 => println!("number was one"),
        2 => println!("number was two"),
        _ => println!("number was something else"),
    }
    /*
    if作为表达式可以给变量赋值
    分支的表达式的类型必须相同
    */
    let condition = if number < 5 { "number < 5" } else { "number >= 5" };
    println!("The value of condition is: {}", condition);
}
