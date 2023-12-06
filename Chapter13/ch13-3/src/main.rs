/*
使用泛型参数和fn trait存储闭包
*/

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

/*
缓存器实现的问题
- Cacher假设对于不同的参数，value总会得到同样的值
- Cacher仅设置了一个泛型参数，导致参数和返回值类型必须相同
可以使用hashmap代替单个缓存值
- key： arg参数
- value：闭包的返回值

*/
struct Cacher<T>
    where T: Fn(u32) -> u32 // 闭包类型约束，T必须实现Fn trait
{
    calculation: T, // 闭包
value: Option<u32>, //缓存的值，运行闭包前为None
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation:T)->Cacher<T>{ // 构造函数,传入闭包
        Cacher{
            calculation,
            value:None,
        }
    }

    fn value(&mut self,arg:u32)->u32{ // 闭包调用函数
        match self.value{
            Some(v)=>v, // 有值，直接返回
            None=>{
                let v = (self.calculation)(arg); // 调用闭包
                self.value = Some(v); // 缓存值
                v // 返回值
            }
        }
    }
}
fn generate_workout(intensity: u32, random_number: u32) {
    /*
    解决重复调用的问题

    创建一个结构体，它持有闭包及其调用结果
    - 只会在需要结果的时候才执行闭包
    - 可以缓存结果
    这个模式叫做记忆化或延迟计算
    */
    /*
    struct持有闭包
    struct的定义需要知道所有字段的类型
    - 需要指明闭包的类型
    每个闭包实例都有自己唯一的类型，即使两个闭包签名完全一致
    所以需要使用：泛型和Trait Bound
    */
    /*
    Fn Trait
    fn traits由标准库提供
    所有闭包都至少实现了以下trait之一：
    - Fn
    - FnMut
    - FnOnce
    */
    let mut expensive_closure = Cacher::new(|num|{
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });



    if intensity < 25 {
        println!("Today, do {:?} pushups!",
                 expensive_closure.value(intensity));
        println!("Next, do {:?} situps!",
                 expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break!");
        } else {
            println!("Today, run for {:?} minutes!", expensive_closure.value(intensity));
        }
    }
}

fn main() {
    generate_workout(3, 2);
    /*
    Result:
    calculating slowly ...
    Today, do 3 pushups!
    Next, do 3 situps!
    */
}

