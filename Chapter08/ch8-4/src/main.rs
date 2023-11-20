use unicode_segmentation::UnicodeSegmentation;
fn main() {
    /*
    对string按索引方式访问
    会报错
    */
    let s1 = String::from("hello");
    // println!("{}", s1[0]); //error `String` cannot be indexed by `{integer}`

    /*
    String类型的内部表示
    是对Vec<u8>的封装
    - len: 字符串长度, 单位是字节
    下标并不总能对应到字符，因此设计上无法支持索引访问
    */
    println!("s1.len() = {}", s1.len());

    /*
    字节、标量值、字形簇
    是Rust中三种不同的字符概念
    */
    /*
    输出字节
    */
    let s2 = String::from("नमस्ते");
    print!("s2.len() = {}, value = ", s2.len()); //输出的是字节长度
    for byte in s2.bytes(){ //输出每个字节的值
        print!("{}, ", byte);
    }
    println!();
    /*
    输出标量值
    */
    for ch in s2.chars(){
        print!("{}, ", ch); //输出每个字符
    }
    println!();

    /*
    输出字形簇
    */
    for cluster in s2.grapheme_indices(true) {
        let (index, grapheme) = cluster;
        println!("Index: {}, Grapheme: {}", index, grapheme);
    }
    /*
    切割字符串
    */
    let s3 = String::from("Привет");
    // let sub_s3 = &s3[0..3]; //error byte index 3 is not a char boundary; it is inside 'р' (bytes 2..4) of `Привет`
    let sub_s3 = &s3[0..2];
    //以字节为单位切割，但是必须要沿着字符边界切割，字符边界以chars()方法返回的字符为准
    println!("sub_s3 = {}", sub_s3);
    /*
    Rust字符串的思想：将正确处理String数据作为所有Rust程序的默认行为
    */
}
