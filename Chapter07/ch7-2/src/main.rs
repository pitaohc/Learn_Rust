mod front_of_house {
    // 私有模块
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    mod serving {
        use crate::front_of_house::hosting::add_to_waitlist;

        //无法从front_of_house外部访问
        fn take_order() {
            add_to_waitlist();
        }

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // 绝对路径，因为与私有模块同处于一个文件，所以可以访问到
    front_of_house::hosting::add_to_waitlist(); // 相对路径
}

fn main() {
    /*
    路径(Path)
    为了在Rust的模块中找到某个条目，需要使用路径。
    路径的两种形式:
    - 绝对路径:从crate root开始, 使用crate名或字面值crate
    - 相对路径:从当前模块开始, 使用self, super或当前模块的标识符
    路径至少由一个标识符组成,标识符之间使用::
    */
    eat_at_restaurant();
    /*
    私有边界 private boundary
    - 模块可以定义私有边界
    - 如果希望函数或者struct私有，可以将其放入私有模块中
    - 所有条目默认私有
    - 使用pub关键字可以使得条目变为公有
    - 父模块无法使用子模块的私有条目
    - 子模块可以使用父模块的私有条目
    - 同级模块可以相互调用，无论是否私有
    */
}
