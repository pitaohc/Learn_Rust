
/*
不是所有类型都能比较大小
只有实现了std::cmp::PartialOrd的类型才能比较大小
在函数签名中添加对T的限制，确保T是实现了std::cmp::PartialOrd的类型
*/
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0]; // 不能使用let largest = list[0];，因为list[0]是一个引用
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/*
结构体泛型
*/
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Debug)]
struct Point2<T1,T2>{
    x: T1,
    y: T2,
}
/*
enum泛型
*/
enum MyOption<T> { // T是泛型参数
    Some(T), // Some是泛型枚举的一个成员，T是Some的泛型参数
    None, // None不是泛型枚举的成员
}
enum MyResult<T1, T2> {
    Ok(T1), // Ok是泛型枚举的一个成员，T1是Ok的泛型参数
    Err(T2), // Err是泛型枚举的一个成员，T2是Err的泛型参数
}
/*
方法的定义中使用泛型
*/
impl<T> Point<T> { //在impl后面的<T>中声明了泛型参数T, 这个T的实际类型与Point<T>中的T是一致的
    fn x(&self) -> &T { //getter x
        &self.x
    }
}
/*
泛型特化
*/
impl Point<f32> { //在impl后面的<f32>中声明了泛型参数T的特化类型为f32
    fn distance_from_origin(&self) -> f32 { //计算点到原点的距离
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
impl<T,U> Point2<T,U>{
    fn mixup<V,W>(self, other: Point2<V,W>) -> Point2<T,W> {
        /*
        此处有4个类型：T,U,V,W
        - self: Point2<T,U>
        - other: Point2<V,W>
        - return: Point2<T,W>
        */
        Point2 {
            x: self.x, //T
            y: other.y,//W
        }
    }
}
fn main() {
    /*
    泛型
    - 处理泛型数据类型，提高代码复用能力
    - 泛型是具体类型或其他属性的抽象代替
        - 函数体内使用泛型时，会在编译时根据具体使用的类型进行替换

    例子：
    fn largest<T>(list: &[T]) -> T{}
    T是泛型参数，表示任意类型
    类型参数命名：
    - 很短通常一个字母
    - 使用驼峰命名法
    - T: type的缩写
    */

    /*
    函数定义中的泛型
    - 泛型函数：
        - 参数类型
        - 返回类型
    */
    let nums = vec![1, 2, 3, 4, 5];
    let largest_num = largest(&nums);
    println!("list: {:?}, largest: {}", nums, largest_num);

    /*
    struct中的泛型
    */
    let p1 = Point { x: 1, y: 2 }; // x和y都是i32类型,且x和y必须是同类型的
    let p2 = Point { x: 1.1, y: 2.2 };
    /*
    如何定义一个结构体，使得x和y可以是不同类型的？
    使用两个类型别名
    如果出现太多的类型参数，需要考虑重组struct为多个更小的单元
    */
    let p3 = Point2 { x: 1, y: 2.2 };

    /*
    enum中的泛型
    */


    /*
    方法的定义中使用泛型
    */
    println!("p1.x = {}", p1.x());
    println!("p2.x = {}", p2.x());
    let p4 = Point2 { x: "abc", y: "def" };
    println!("mixed: {:?}", p3.mixup(p4));
    //p3和p4都无法继续使用，因为p3和p4的所有权已经被转移给了mixup函数

    /*
    泛型代码性能
    使用泛型的速度不会比使用具体类型慢
    单态化（monomorphization）：编译器将泛型代码转换为特定代码的过程
    */
}
