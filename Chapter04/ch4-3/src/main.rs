fn main() {
    /*
    切片 Slice
    指向字符串部分内容的引用
    */
    let mut s = String::from("hello world");
    let hello = &s[0..5]; //也可以写成&[0..5]
    let world = &s[6..11]; //也可以写成&[6..]
    let whole = &s[..]; //等价于&[0..11]
    println!("{} {}", hello, world);
    println!("whole: {}", whole);

    let first = first_word(&s);
    // 此时无法对s进行修改，因为first_word函数获取了s的引用，而引用默认是不可变的
    // s.clear(); //清空字符串，导致first的引用失效 //error E0502: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("first word: {}", first);
    //解除first的引用
    let first = "解除first对s的引用";
    println!("first: {}", first);
    s = String::from("new string"); //此时first已经不是s的引用，因此可以修改s
    println!("s: {}", s);

    /*
    字符串字面值就是切片 &str
    字符串字面值直接存储在二进制文件中

    将字符串切片作为参数传递
    fn first_word(s: &str) -> &str {...}
    这样写可以同时接收String和&str形式的参数
    这样定义会使API更加通用，且不会损失任何功能
    */

    /*其他形式的切片*/
    let nums = [1,2,3,4,5];
    let slice = &nums[1..3]; //类型为&[i32]
    println!("slice: {:?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){ //enumerate()方法返回一个元组，元组的第一个元素是索引，第二个元素是集合中元素的引用
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}