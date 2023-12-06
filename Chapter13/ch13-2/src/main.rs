use std::thread;
use std::time::Duration;
/*
闭包的类型推断
闭包不要求标注参数和返回值的类型
函数的大家调用的接口，标注参数和返回值类型有助于代码可读性
闭包并不是暴露的接口，基本只是临时使用
闭包通常短小，只在狭小的上下文中工作，编译器通常能推断出类型

函数和闭包的定义语法比较
- fn  add_one_v1   (x: u32) -> u32 {x+1}
- let add_one_v2 = |x: u32| -> u32 {x+1}; // 闭包
- let add_one_v2 = |x|      -> u32 {x+1}; // 省略类型标注
- let add_one_v2 = |x|      -> u32  x+1 ; // 单行语句不需要写{}
*/
/*
闭包的类型推断
闭包的定义最终只会为参数/返回值推断出唯一的具体类型
无法使用泛型功能
*/


fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello")); // 将类型绑定到闭包
    // let n = example_closure(5); // error[E0308]: mismatched types
}
