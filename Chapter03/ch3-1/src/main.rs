fn main() {
    // 声明变量使用let关键字
    // 默认变量是不可变的
    // 可以使用mut关键字来声明变量是可变的
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; //cannot assign twice to immutable variable
    let mut x = 5;
    x = 6;
    println!("The value of x is: {}", x);

    // 常量使用const关键字
    // 常量必须注明类型
    // 常量在绑定值以后也是不可变的，但是它与不可变的变量有很多区别：
    // 1. 常量只能绑定到常量表达式，而不能是函数调用的结果或任何其他只在运行时计算出的值
    // 2. 不可以使用mut修饰，常量永远都是不可变的 error: const mut x
    // 3. 常量可以在任何作用域中声明，包括全局作用域，这在全局作用域中声明常量是很有用的，因为常量需要在程序的任何地方都有效
    // 在程序运行期间，常量在作用域内总是有效
    // RUST命名规范：常量使用全大写，每个单词之间使用下划线分割
    const MAX_POINTS: u32 = 100_000;

    //Shadowing
    // 可以使用相同名字声明新的变量，新的变量会覆盖之前的变量
    let y = 5;
    // y = y+1; //error: cannot assign twice to immutable variable
    let y = y + 1;//可以理解为let只是声明了一个指针指向一块内存区域，这个指针是不可变的，let让这个指针指向了新的区域

    let mut spaces = "   ";
    // spaces = spaces.len(); //error: mismatched types
    let spaces = spaces.len();
    // spaces = spaces + 1; //error: cannot assign twice to immutable variable
    let mut spaces = spaces; // 允许可变变量覆盖不可变变量
}
