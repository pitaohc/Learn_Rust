/*
使用闭包捕获环境
filter方法：筛选元素
接收一个闭包
这个闭包在遍历迭代器的每个元素时，返回bool类型
如果闭包返回true，当前元素将会包含在filter产生的迭代器中，否则不会被包含
*/
fn example_1() {
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let v2: Vec<i32> = v1.into_iter().filter(|x| x % 2 == 0).collect();// 收集偶数
    println!("{:?}", v2);
}

fn main() {
    example_1();
}
