#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 此时在Rectangle的上下文中定义了area方法
    // 方法
    fn area(&self) -> u32 { //self 代表调用该方法的实例，被推定为Rectangle
        self.width * self.height
    }

}

fn main() {
    /*
    struct方法
    - 与函数的相同点:fn关键字、名字、参数、返回值
    - 与函数的不同点:
        - 方法是在struct(enum, trait)的上下文中定义的函数
        - 方法的第一个参数是self,代表调用该方法的实例
    */

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect is {:?}", rect);

    /*
    定义方法
    在impl块中定义方法
    方法的第一个参数可以是&self,也可以获得其所有权或可变借用
    */
    println!("rect area is {}", rect.area());

    /*
    方法调用的运算符
    在调用方法时，Rust会自动引用或解引用
    会根据情况添加&、&mut或*,以便使得调用方法的实例符合方法的签名
    下面两行代码等价
    p1.distance(&p2);
    (&p1).distance(&p2); //此处可以理解为p1是方法的第一个参数，使用引用传入，而p2是第二个参数，使用引用传入
    */

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("rect can hold rect2? {}", rect.can_hold(&rect2));

    /*
    关联函数
    在impl中定义不把self作为第一个参数的函数，称为关联函数。可以理解为静态方法
    例如String::from()
    关联函数通常用于构造器（创建类实例）

    ::符号
    - 用于关联函数
    - 用于模块创建的命名空间
    */
    let sq = Rectangle::square(3);
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle { //创建正方形
        Rectangle {
            width: size,
            height: size,
        }
    }
}