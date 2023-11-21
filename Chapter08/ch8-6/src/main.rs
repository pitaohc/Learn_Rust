use std::collections::HashMap;

fn main() {
    /*
    更新hashmap
    - hashmap大小可变
    - 每个k对应一个v

    更新时的情况：
    - k已经存在，对应一个v
        - 替换v
        - 保留原有v
        - 合并原有v和新v
    - k不存在
        - 添加新k,v
    */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 100);
    scores.insert(String::from("Yellow"), 23);
    println!("old hashmap: {:?}", scores);
    /*
    k已经存在
    */
    scores.insert(String::from("Blue"), 25);
    println!("new hashmap: {:?}", scores);

    /*
    k不存在
    entry方法：检查指定的k是否对应v
    - 参数k
    - 返回值：枚举类型Entry
        - Entry::Occupied(k,v) k存在，对应v
        - Entry::Vacant(k) k不存在
    */
    scores.insert(String::from("Green"), 25); // 不管k是否存在，添加新k,v

    /*
    or_insert方法
    - 返回
        - k存在，返回对应v的可变引用
        - k不存在，插入新k,v，返回v的可变引用
    */
    scores.entry(String::from("Green")).or_insert(50); // k不存在，添加新k,v，k如果存在则不更新
    println!("new hashmap: {:?}", scores);
    let mut green_score = scores.entry(String::from("Green")).or_insert(50);
    *green_score += 10;  // 更新v
    println!("new hashmap: {:?}", scores);

    /*
    例子：统计单词出现次数
    */
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() { // split_whitespace()返回迭代器
        let count = map.entry(word).or_insert(0); // or_insert返回v的可变引用
        *count += 1;
    }
    println!("word count: {:?}", map);
    /*
    hash函数
    默认的hash功能强大，可以抵抗dos攻击。安全性高，但不是最快的hash算法
    可以指定不同的hasher来切换到另一个函数
    - hasher是一个实现了BuildHasher trait的类型
    */

}
