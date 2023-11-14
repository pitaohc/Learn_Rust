fn main() {
    /*
    标量和复合类型
    Rust是静态编译语言，在编译时就必须知道所有变量的类型。
    - 基于使用的值，Rust可以推断出大部分变量的类型。
    - 有时候，我们需要显式地指定类型，这样可以增加可读性。
    - 但如果可能的类型太多，Rust就会报错，这时候就需要显式地指定类型。如String转为整形的parse方法。
    */
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    /*
    标量类型
    一个标量类型代表一个单独的值。Rust有四种基本的标量类型：整形、浮点型、布尔型和字符型。
    */

    /*
    整数类型
    整数类型有两种：有符号和无符号。有符号的可以表示正数、负数和0，无符号的只能表示正数和0。
    无符号：u8、u16、u32、u64、u128、usize
    有符号：i8、i16、i32、i64、i128、isize
    usize和isize由程序运行的计算机决定，64位计算机是64位，32位计算机是32位。
    usize和isize常见于对集合的索引操作
    */
    let a: u128 = 1;
    println!("a: {}", a);

    /*
    整数字面值
    十进制:        98_222   一个下划线分隔符，可以增加可读性
    十六进制:       0xff
    八进制:        0o77
    二进制:        0b1111_0000
    字节（u8类型）: b'A'
    */

    /*
    整数溢出
    Debug模式: Rust会检查溢出，如果溢出，程序会panic（报错）
    Release模式: Rust不会检查溢出，如果溢出，程序会继续执行。溢出的结果和C/C++类似，会从最小值开始。
    */
    // let numu32: u32 = 65536;
    // let num: u16 = numu32;

    /*
    浮点类型
    Rust有两种浮点类型：f32和f64，默认是f64。使用IEEE754标准表示。
    */

    /*
    布尔类型
    布尔类型有两个值：true和false。
    大小为1Byte
    符号是bool
    */
    let t = true;
    let t2: bool = false;
    println!("t: {}, t2: {}", t, t2);

    /*
    字符类型
    Rust的char类型是Unicode标量值，不是单个字节，描述单个字符。占用4Bytes。
    字符类型使用单引号，字符串类型使用双引号。
    */
    let message = "你好😘";
    println!("message: {}", message);
    let ch: char = '😘';
    println!("ch: {}", ch);
}
