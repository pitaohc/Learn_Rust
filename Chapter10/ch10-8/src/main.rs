fn main() {
    /*
    方法定义中的生命周期标注
    - 在struct上使用生命周期实现方法，语法和泛型参数的语法一样
    - 在什么位置声明和使用生命周期参数，依赖于：生命周期参数是否和字段、方法的参数、方法的返回值有关

    struct字段的生命周期名：
    - 在impl后
    - 在struct名后
    - 这些生命周期是struct的一部分

    impl块内的方法签名中：
    - 引用必须绑定于struct字段引用的生命周期，或者引用是独立的也可以
    - 生命周期省略规则经常是的方法中的生命周期标注不是必须的
    */

    /*
    静态生命周期
    - 'static，整个程序运行期间都有效
    - 所有的字符串字面值都拥有'static生命周期
    - 为引用指定'static生命周期前，先确保引用的数据确实是静态生命周期
    */

    /*
    综合例子：包含泛型生命周期参数、泛型类型参数、trait bound的函数

    功能：返回x和y中较长的那个字符串的引用
    指定了泛型生命周期'a和泛型类型T
    由于有两个引用参数，所以需要显示指定生命周期
    x是字符串切片，y是字符串切片，返回值是字符串切片
    ann是泛型类型T，根据where的约束T必须实现Display trait
    */
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: std::fmt::Display
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

struct ImportantExcerpt<'a> {
    // 生命周期标注
    part: &'a str, // part 是外部的引用，需要保证在struct实例有效时，part也有效
}

impl<'a> ImportantExcerpt<'a> { // 生命周期标注

    fn level(&self) -> i32 { // self不需要生命周期标注
        3
    }
    /*
    fn announce_and_return_part(&self, announcement: &str) -> &str
    fn announce_and_return_part<'a, 'b>(&'a self, announcement: &'b str) -> & str
    fn announce_and_return_part<'a, 'b>(&'a self, announcement: &'b str) -> &'a str
    */
    fn announce_and_return_part(&self, announcement: &str) -> &str { // announcement是独立的引用，不需要生命周期标注
        println!("Attention please: {}", announcement);
        self.part
    }
}
