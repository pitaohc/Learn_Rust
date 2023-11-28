struct ImportantExcerpt<'a> {
    part: &'a str, //part 需要比结构体的实例生命周期更长
}

fn main() {
    /*
    从函数返回引用时，返回类型的生命周期参数必须要与其中一个参数的生命周期匹配
    如果返回的引用没有指向任何参数，那么它智能引用函数内部创建的值
    这就会导致悬垂引用，该值在函数返回时被销毁
    */
    let s = create_string();
    println!("{}", s);

    /*
    Struct定义中的生命周期标注
    struct中可包含
    - 自持有类型，拥有变量的所有权，管理变量的生命周期
    - 引用：需要在每个引用上添加生命周期标注
    */
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);

    /*
    生命周期的省略
    - 每个引用都有生命周期
    - 需要为使用生命周期的函数或struct指定生命周期参数
    - 有些情况下，编译器可以推断出生命周期参数
        - 函数或struct只有一个引用参数
        - 在未来的某个版本中，编译器可能会更加智能
    */


    /*
    输入、输出生命周期
    生命周期在
    - 函数/方法的参数：输入生命周期
    - 函数/方法的返回值：输出生命周期
    */

    /*
    生命周期省略的三个规则
    - 规则1应用于输入生命周期
    - 规则2,3应用于输出生命周期

    规则1：每个引用的参数都有它自己的生命周期参数
    规则2：如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
    规则3：如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为&self或&mut self，那么self的生命周期被赋予所有输出生命周期参数
    */
    /*
    正确例子：
    fn first_word(s: &str) -> &str { s }
    应用规则1：
    fn first_word<'a>(s: &'a str) -> str { s }
    应用规则2：
    fn first_word<'a>(s: &'a str) -> &'a str { s }

    错误例子：
    fn longest(x: &str, y: &str) -> &str { ... }
    应用规则1：
    fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str { ... }
    由于存在两个参数，所以不能应用规则2
    由于是函数而不是方法，所以不能应用规则3
    报错
    */
}

/*
fn create_string<'a>() -> &'a str {
    let s = String::from("hello");
    s.as_str() // error 返回了一个被当前函数拥有的值的引用
}
*/

/*
如果需要返回本地变量
需要将所有权转移给调用者
*/
fn create_string<'a>() -> String {
    let s = String::from("hello");
    s
}