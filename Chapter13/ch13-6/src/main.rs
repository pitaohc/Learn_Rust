/*
消耗迭代器的方法
在标准库中，Iterator trait有一些带默认实现的方法
其中有一些方法会调用next方法
- 实现自定义Iterator trait时必须实现next方法
调用next的那些方法被称为“消耗性适配器”

例子
sum方法
- 用于获取迭代器中所有元素的总和
- 取得迭代器的所有权
- 反复调用next，遍历所有元素
- 每次迭代，把当前元素添加到一个总和里，迭代结束返回总和
collect方法
- 可以把迭代器转换为一个集合
*/
fn example_1() {
    println!("example_1");
    let v1 = vec![1, 2, 3];
    let total: i32 = v1.iter().sum();
    println!("{}", total);
    // println!("{:?}", v1_iter); // error
}
/*
产生其他迭代器的方法
定义在Iterator trait上的另外一些方法叫做“迭代器适配器”
- 把迭代器转换为不同种类的迭代器
可以通过链式调用使用多个迭代器适配器来执行复杂操作，这种调用可读性高

例子
map方法
- 接受一个闭包作为参数，闭包作用于每个元素
*/
fn example_2() {
    println!("example_2");
    let v1: Vec<i32> = vec![1, 2, 3];

    for num in v1.iter().map(|x| x + 1) {
        print!("{}, ", num); // 并不会修改原有的值，因为是iter返回不可变引用
    }
    println!();
    for num in v1.iter() {
        print!("{}, ", num);
    }
    println!();
    /*
    v1.iter() 返回不可变引用的迭代器
    map方法获取每个元素的不可变引用，并调用闭包
    该闭包获取元素值并+1返回
    collect方法消耗迭代器并把结果收集到一个数据结构中
    vec<_> 让编译器推断类型
    */
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
}

fn main() {
    example_1();
    example_2()
}
