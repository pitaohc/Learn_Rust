fn main() {
    /*
    常用的集合
    动态分配内存
    - Vector
    - String
    - HashMap
    */

    /*
    Vector
    Vec<T>
    - 由标准库提供
    - 可以存储多个相同类型的值
    - 连续存放
    */

    /*
    创建
    let v:Vec<i32> = Vec::new();
    */
    let v: Vec<i32> = Vec::new(); //指名类型的原因：没有初始值，编译器无法推断类型
    let mut v = vec![1, 2, 3]; //使用宏创建，编译器可以推断类型
    println!("{:?}", v);
    //使用上下文推导判定类型
    let mut v = Vec::new();
    v.push('1');
    v.push('2');
    println!("{:?}", v);

    /*
    添加元素
    vec.push()
    */
    v.push('4');
    println!("pushed vector: {:?}", v);

    /*
    删除Vector
    当Vector离开作用域时
    - 所有元素被丢弃
    - 当前Vec对象被丢弃
    */

    /*
    读取Vec元素
    - 索引
    - get方法
    */
    let v = vec![1, 2, 3, 4, 5];
    println!("v[0] = {}", v[0]);
    match v.get(0) {
        Some(first) => println!("The first element is {}", first),
        None => println!("There is no first element."),
    }
    // println!("v[100] = {}", v[100]); // error index out of bounds 越界时会panic
    match v.get(100) { //越界时返回None
        Some(first) => println!("The first element is {}", first),
        None => println!("There is no 100th element."),
    }

    /*
    所有权和借用规则
    - 不能在同一作用域中同时存在可变和不可变引用
    */
    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // error 对v的修改，有可能导致索引失效
    println!("The first element is: {}", first);

    /*
    遍历vec
    */
    let v = vec![100, 32, 57];
    for i in &v{ //i是不可变引用
        print!("{}, ", i);
    }

    /*
    可变引用遍历
    */
    let mut v = vec![100, 32, 57];
    for i in &mut v{ //i是可变引用
        *i += 50; //*i 解引用
        print!("{}, ", i);
    }
}
