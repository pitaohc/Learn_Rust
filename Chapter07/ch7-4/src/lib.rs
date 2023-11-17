mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting; //引入hosting
// use self::front_of_house::hosting; //引入hosting
// use front_of_house::hosting; //使用相对路径，引入hosting
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); //可以直接访问hosting
    hosting::add_to_waitlist(); //对于函数，通常引用父模块，使用父模块调用函数
    hosting::add_to_waitlist();
}

use self::front_of_house::Appetizer; //引入Appetizer，对于struct、enum、trait，通常引用本身

