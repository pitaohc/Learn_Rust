use std::fmt::Display;

trait Draw {
    fn draw(&self);
}

trait Size {
    fn size(&self) -> u32;
}

struct Rectangle {
    width: u32,
    height: u32,
}

struct Circle {
    radius: u32,
}

// 为Rectangle实现Draw trait
impl Draw for Rectangle {
    fn draw(&self) {
        println!("Rectangle");
    }
}

impl Size for Rectangle {
    fn size(&self) -> u32 {
        self.width * self.height
    }
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Circle");
    }
}

impl Size for Circle {
    fn size(&self) -> u32 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

pub fn draw(item: impl Draw) { // 传入任何实现了Draw trait的类型
    item.draw();
}

pub fn draw2<T: Draw>(item: T) { // 传入任何实现了Draw trait的类型
    item.draw();
}

// 如果是impl trait语法，签名为 fn compare_size(item1:impl Size,item2:impl Size) -> bool
pub fn compare_size<T: Size>(item1: T, item2: T) -> bool {
    item1.size() > item2.size()
}

pub fn display<T: Draw + Size>(item: T) {
    item.draw();
    println!("size: {}", item.size());
}

fn main() {
    /*
    trait作为参数
    impl trait语法：适用于简单情况
    e.g. draw(item:impl Draw)
    */

    /*
    trait bound语法：适用于复杂情况
    e.g. draw2<T:Draw>(item:T)
    在参数少的时候和impl trait语法区别不大，但是当参数数量多时，trait bound更加简洁
    e.g. pub fn compare_size<T: Size>(item1: T, item2: T) -> bool

    实际上impl trait是trait bound的语法糖

    使用+指定多个trait bound
    e.g. pub fn display<T: Draw + Size>(item: T)
    e.g. pub fn display(item: Draw + Size)
    */

    /*
    使用where从句简化trait bound
    e.g. fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32
    可以改为
    e.g. fn some_function<T, U>(t: T, u: U) -> i32
        where T: Display + Clone,
              U: Clone + Debug
    */

    /*
    使用trait作为返回类型
    必须保证该函数返回的类型是确定的，否则编译器无法确定返回类型
    */
    pub fn return_a_rectangle() -> impl Draw + Size {
        Rectangle {
            width: 10,
            height: 10,
        }
    }

    /*
    使用trait bound有条件地实现方法
    在使用泛型类型参数的impl块上使用trait bound，可以有条件地为实现了特定trait的类型实现方法
    同理，可以有条件地实现某个trait
    为满足trait bound的所有类型上实现trait的行为叫做覆盖实现(blanket implementations)
    */
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) { // 只有当T实现了Display和PartialOrd trait时才实现cmp_display方法
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    impl<T: Display + PartialOrd> Size for Pair<T>{ // 只有当T实现了Display和PartialOrd trait时才实现Size trait
        fn size(&self) -> u32 {
            1
        }
    }
}
