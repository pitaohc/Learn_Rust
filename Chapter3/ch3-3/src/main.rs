fn main() {
    /*
    Rust复合类型：元组（Tuple）和数组（Array）
    */

    /*
    Tuple:
    1. 长度固定，一旦声明无法改变
    2. 每个元素类型可以不同

    创建Tuple:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {:?}", tup);

    /*
    获取Tuple元素值：
    可以使用模式匹配（pattern matching）来解构（destructure）一个元组值，从而将元组中的值绑定给一些变量。类似python的解包
    */
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    /*
    访问Tuple元素值：
    可以使用索引来访问Tuple元素值
    */
    println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);

    /*
    数组（Array）：
    1. 长度固定，一旦声明无法改变
    2. 每个元素类型相同
    */
    let arr = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);
    /*
    数组的数据存放在栈上
    */
    /*
    数组的类型: [类型; 长度]
    */
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr = [3; 5];
    /*
    访问数组元素
    使用索引访问数组元素
    如果超出索引范围
    */
    // let num = arr[8]; //index out of bounds: the length is 5 but the index is 8
    // println!("num: {}", num);
    println!("arr[0]: {}", arr[0]);
}
