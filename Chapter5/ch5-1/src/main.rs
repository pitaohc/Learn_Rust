fn main() {
    /*
    结构体 Struct
    */
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool, // 末尾的逗号是可选的
    }

    /*
    实例化结构体
    - 必须为每个字段填写具体值
    - 无需按定义顺序填写
    */
    let user = User {
        email: String::from("pitao@123.com"),
        username: String::from("pitao"),
        active: true,
        sign_in_count: 1,
    };
    println!("user.email: {}", user.email);

    /*
    取得结构体的值
    - 使用点号
    */

    /*
    为结构体字段赋值
    - 使用点号
    - 结构体是mut的，所有字段都是mut的，不允许部分可变
    */

    /*
    字段初始化简写
    - 当字段名和变量名相同时，可以简写
    */
    let email = String::from("pitao@123.com");
    let username = String::from("pitao");
    let user = User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };

    /*
    struct更新语法
    - 使用..user语法
    - 当想从其他实例创建实例时，可以使用这种语法
    */
    let user2 = User {
        email: String::from("hello@123.com"),
        ..user
    };// user其他字段的值会被复制到user2中，如果数据在heap中，会被移动，如果在stack中，会被复制

    /*
    tuple struct
    - 没有具体的字段名，只有字段的类型
    - 适用于没有具体字段名，但是需要不同类型的数据
    - 访问：使用点号，或者解构
    */

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    /*
    unit-like struct (没有任何字段)
    可以定义没有任何字段的结构体，称为unit-like struct，用于泛型参数
    适用于需要在某个类型上实现trait，但是不需要在类型中存储数据的情况
    */
    struct UnitLikeStruct;

    /*
    struct数据所有权
    - struct中的字段可以拥有不同的所有权
    */
    /*
    这里的email是String类型，而不是&str类型，因为我们希望struct拥有所有权，这样struct在离开作用域时，可以drop email
    - 该实例拥有所有权，因此可以修改email的值
    - 只要struct实例是有效的，那么它的所有字段都是有效的
    - struct也可以存储引用，但是需要用上生命周期
    */
    struct User2 {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
}
