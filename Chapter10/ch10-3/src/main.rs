pub trait Summary {
    fn summarize(&self) -> String;
    fn author(&self) -> String{ // 默认实现
        format!("no author")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn author(&self) -> String { // 覆盖默认实现
        format!("{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    // fn author(&self) -> String {
    //     format!("{}", self.username)
    // }
}

fn main() {
    /*
    Trait
    告诉编译器某个特定类型拥有可能与其他类型共享的功能。
    trait：抽象的定义共享行为
    trait bounds(约束): 泛型类型参数指定为实现了特定行为的类型
    类似c++纯虚函数或java接口
    */

    /*
    定义一个trait
    把方法签名放在一起，来定义实现某种目的所必须的一组行为
    - 关键字：trait
    - 只有方法签名，没有具体实现。与interface概念类似
    - trait可以有多个方法，每个方法签名占1行
    - 实现该trait的类型必须提供具体的方法实现
    */

    /*
    在类型上实现trait
    与类型方法类似
    impl trait_name for type_name {}
    */

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.author());
    let news = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: "Pitao".to_string(),
        content: "hello".to_string(),
    };
    println!("New article available! {}", news.summarize());
    println!("New article available! {}", news.author());

    /*
    实现trait的约束
    类型实现trait的条件：
    - 类型或trait位于当前crate中。这个限制是程序属性的一部分，为了防止其他人在你的类型上实现某个外部trait
    */

    /*
    默认实现
    类似C++中的虚函数
    trait中的方法可以提供默认实现，而不是在每个类型的每个实现中都定义自己的实现。
    见上面的author方法
    */

}
