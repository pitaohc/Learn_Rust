use ch7_1::front_of_house;
fn main() {
    /*
    Package, Crate, Module
    */
    /*
    Rust代码组织
    - 暴露或私有哪些细节
    - 作用域内哪些命名有效
    模块系统：自顶向下
    - Package(包):Cargo特性，构建、测试、共享crate
    - Crate(单元包): 一个模块树，生成一个库或可执行文件
    - Module(模块):控制代码的组织、作用域、私有路径等
    - Path(路径):为struct、function、module命名的方式
    */

    /*
    Package和Crate
    Crate类型：
    - binary
    - library
    Crate Root:
    - 源代码文件，Rust编译器从这里开始编译crate
    一个Package:
    - 包含一个Cargo.toml文件,描述如何构建crate
    - 只能包含0-1个library crate
    - 可以有任意数量的binary crate
    - 但必须至少包含一个crate
    */

    /*
    命令：cargo new ch7-1
     Created binary (application) `ch7-1` package
    */

    /*
    Cargo惯例
    - src/main.rs: binary crate的crate root
        - crate名与package名相同
    - src/lib.rs: library crate的crate root
        - crate名与package名相同
    Cargo把crate root文件交给rustc来构建library或binary
    一个package可以同时包含src/main.rs和src/lib.rs

    理解：package相当于解决方案，crate相当于项目，module相当于类
    */

    /*
    Crate作用
    将相关功能组合到一个作用域内，便于在项目间共享
    - 作用域内的代码可以访问crate的任何内容
    例如：rand crate，提供随机数生成器，访问它需要使用use语句，use rand
    */

    /*
    定义module控制作用域和私有性
    Module:
    - 在一个crate内，将代码进行分组
    - 增加可读性，易于复用
    - 控制项目item的私有性
    建立module:
    - 使用mod关键字
    - 可嵌套
    */
    //调用lib.rs中的front_of_house
    front_of_house::hosting::add_to_waitlist();
}
