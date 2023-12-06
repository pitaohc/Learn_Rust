use std::thread;
use std::time::Duration;
/*
迭代器和闭包

闭包：
可以捕获其所在环境的匿名函数
闭包的特点
- 匿名函数
- 保存为变量，可以作为参数传递给函数
- 可在一个地方创建闭包，但是在另一个上下文中调用闭包完成运算
- 可以从其定义的作用域捕获值

c++ lambda表达式就是闭包
*/

/*
例子——生成自定义运动计划的程序
- 算法中的计算过程需要较长时间
- 尽量少调用算法
*/


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2)); // 线程睡眠2秒
    intensity
}

fn generate_workout1(intensity: u32, random_number: u32) {
    if intensity < 25 { // 会调用两次耗时长的算法
        println!("Today, do {} pushups!",
                 simulated_expensive_calculation(intensity));
        println!("Next, do {} situps!",
                 simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break!");
        } else {
            println!("Today, run for {} minutes!", simulated_expensive_calculation(intensity));
        }
    }
}
/*
闭包写法
*/
fn generate_workout2(intensity: u32, random_number: u32) {
    let expensive_closure = |num|{
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2)); // 线程睡眠2秒
        num
    }; // 定义闭包
    if intensity < 25 { // 会调用两次耗时长的算法
        println!("Today, do {} pushups!",
                 expensive_closure(intensity));
        println!("Next, do {} situps!",
                 expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}



fn main() {
    generate_workout1(3, 2);
}
