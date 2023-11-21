use std::collections::HashMap; // 引入HashMap

fn main() {
    /*
    HashMap
    HashMap<K,V>
    - 键值对的形式存储数据，一个键对应一个值
    - hash函数：决定如何在内存中存放K和V
    */

    /*
    创建
    */
    // let mut scores = HashMap::new(); // 空HashMap，无法推断类型
    let mut scores: HashMap<String, i32> = HashMap::new(); // 空HashMap，指定类型

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); // 插入键值对,自动判断类型
    println!("{:?}", scores);

    /*
    HashMap使用较少
    - 不在Prelude中，必须手动use
    - 标准库支持少，没有内置宏来创建HashMap
    - 数据存放在heap上
    - 同构的：所有key类型相同，所有value类型相同
    */

    /*
    使用collect方法创建hashmap
    在元素为tuple的vector上调用collect方法，可以创建HashMap
    - 要求tuple的第一个元素为key，第二个元素为value
    - collect方法可以把数据整合成很多集合类型，包括hashmap。所以需要显示指明类型
    */
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50]; // 两个vector
    /*
    iter(): 创建一个迭代器，遍历vector中的每一个元素
    zip(): 创建一个元组的vector，元组中的元素来自两个迭代器中的元素
    collect(): 创建一个集合，这里是HashMap
    */
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); // 两个vector合并成hashmap

    /*
    HashMap和所有权
    - 对于实现了Copy trait的类型，其值会被复制到hashmap中
    - 对于拥有所有权的值（如String），其值会被移动到hashmap中，所有权转移给hashmap
    - 如果将值的引用插入到hashmap中，值本身不会移动。但是引用指向的值必须在hashmap有效时也是有效的
    */

    /*
    访问HashMap中的值
    - get方法：参数k，返回Option<&V>
    */
    match scores.get(&String::from("Blue")) { // 返回Some(&10)
        Some(&n) => println!("{}", n),
        None => println!("None"),
    }

    /*
    遍历hashmap
    */
    for(k,v)in &scores{ // k,v是引用 ,scores是引用,为了使用完后还能继续使用
        println!("{}:{}",k,v);
    }
}

