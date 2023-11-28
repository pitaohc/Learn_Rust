fn main() {
    /*
    生命周期标注语法
    - 生命周期的标注不会改变引用的生命周期长度
    - 当指定了泛型生命周期参数，函数可以接收带有任何生命周期的引用
    - 生命周期标注描述了多个引用生命周期相互的关系，而不影响其生命周期

    生命周期参数名称：
    - 以'开头
    - 通常全是小写，且非常短
    - 常见的名称：'a, 'b, 'c

    生命周期标注的位置：
    - 在引用符号&后
    - 使用空格将标注和引用类型分开
    - 例如：
        - &'a i32 表示一个指向i32类型的引用，其生命周期至少与'a一样长
        - &'a mut i32 表示一个指向i32类型的可变引用，其生命周期至少与'a一样长
    单个生命周期的标注没有意义，生命周期标注的主要目的是描述**多个**引用生命周期相互的**关系**
    */
    example1();
}

fn example1() {
    /*
    生命周期标注的例子
    */
    let string1 = String::from("abcd"); // string1的生命周期到函数结束
    let result;
    {
        let string2 = "xyz"; // string2的生命周期是静态生命周期，即整个程序的生命周期，因为它是字符串字面量
        result = longest(string1.as_str(), string2);
        //此时string1的生命周期更短，因此result的生命周期与string1的生命周期相同，到函数结束
    }
    println!("The longest string is {}", result); // 此处result仍然有效
}

fn example2()
{
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        // 此处string2的生命周期更短，因此result的生命周期与string2的生命周期相同，到代码块结束
        println!("The longest string is {}", result); // 此处result仍然有效
    }
    // println!("The longest string is {}", result); // 此处result已经无效，因为string2已经被销毁
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    /*
    此处有个泛型生命周期的标注'a
    参数x,y和返回值都使用了'a标注
    表示参数x,y和返回值的生命周期是相同的

    'a标注的生命周期是x和y的生命周期中重叠的部分
    */
    if x.len() > y.len() {
        x
    } else {
        y
    }
}