fn main() {
    println!("Hello, world!");
    f411_stack_heap();
    f412_owner_system();
    f413_owner_function();
}

fn f411_stack_heap() {
    println!("f411_stack_heap");
    /*
    所有权
    所有权是Rust的特性，它可以让Rust编译器在编译时检查内存的使用情况，从而避免内存泄漏和数据竞争，保证了内存安全
    */


    /*
    什么是所有权
    所有程序在运行时必须管理他们使用内存的方式
    - gc：JAVA的JVM可以回收垃圾内存，python的解释器也可以回收垃圾内存
    - 手动管理内存：C/C++需要手动管理内存，需要程序员自己去申请内存和释放内存
    - Rust：内存通过一个所有权系统管理，其中包含一组编译器在编译时检查的规则
    */

    /*
    Stack & Heap

    Stack:
    - LIFO（后进先出），按值的接收顺序存储
    - 添加数据，压栈
    - 移除数据，出栈
    - 所有数据必须有已知的固定大小

    Heap:
    - 当放入数据到Heap时，需要请求一定数量的空间
    - 操作系统会在Heap找到一块足够大的空间，标记为已用，并返回一个指针

    把数据存放到Stack上比存放到Heap上要快
    因为操作系统不需要寻找可用空间
    访问Stack的数据同样也比Heap快
    因为需要通过指针才能访问heap上的数据
    */

    /*
    所有权功能：管理Heap
    - 跟踪代码的哪些部分正在使用heap的哪些数据
    - 最小化heap上的重复数据的数量
    - 清理heap上未使用的数据以避免占用过多的空间
    */
}

fn f412_owner_system()
{
    println!("f412_owner_system");
    /*
    所有权规则：
    每个值都有一个变量，这个变量是它的所有者
    一次只能有一个所有者
    当所有者超出作用域时，这个值将被丢弃
    */

    /*
    变量作用域Scope
    */

    //变量S不可用
    {
        let s = "hello"; //s 可用
        println!("{}", s); //s 可用
    } //s 不可用

    let s1 = "hello";
    let s2 = String::from("hello");
    //s1的类型为&str，s2的类型为String
    //&str是一个固定长度的字符串，它是一个指向二进制程序特定位置的引用
    //String是一个可变长度的字符串，它是一个指向堆上的内存的指针，它还包含了长度和容量的元数据
    println!("s1: {}, s2: {}", s1, s2);
    let mut s2 = s2;
    s2.push('!');
    println!("s2: {}", s2);

    /*
    内存分配和释放
    当变量离开作用域时，Rust自动调用一个特殊的函数drop，内存就会被释放
    */

    /*
    变量和数据交互的方式：移动(Move)
    e.g. 1 i32
    let x = 5;
    let y = x;
    整数是已知且固定大小的简单的值，这两个5被压到stack中

    e.g. 2 String
    let s1 = String::from("hello");
    let s2 = s1;
    String是一个三部分的值:指向堆的指针ptr，长度len，容量capacity
    这三个值会被压入栈中，而具体的字符串数据会被放入堆中

    s1 -> heap
    s2 -> heap

    把s2赋值给s1时，Rust会把s1的指针ptr，长度len，容量capacity复制到s2中，但是不会复制堆中的数据
    当变量离开作用域时，Rust会调用drop函数，释放s1和s2的内存
    这会导致s2和s1指向同一块内存，当s2离开作用域时，会尝试释放同一块内存两次，这会导致内存错误
    Rust的解决方案是，当s1被赋值给s2时，Rust会使s1无效，这样当s1离开作用域时，不会释放内存
    类似c++的移动语义

    与浅拷贝的区别：浅拷贝之前的所有者还能正常访问数据，但是移动操作会让之前的所有者不能访问数据
    */
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1: {}", s1); //error, value borrowed here after move
    println!("s2: {}", s2);

    /*
    变量和数据交互的方式：克隆(Clone)
    如果需要深拷贝，可以使用clone方法
    针对heap上的数据
    */
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: \"{}\" was cloned to s2: \"{}\"", s1, s2); //s1和s2都可以正常访问数据

    /*
    Stack上的数据：Copy trait
    - 用于完全在Stack上的数据，如整数，浮点数，布尔，字符，元组
    - 如果一个类型实现了Copy trait，那么旧的变量在赋值后仍然可用
    - 如果一个类型或者该类型的一部分实现了Drop trait，那么它就不能实现Copy trait
    */
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y); // x和y都可以正常访问数据

    /*
    拥有Copy trait的类型
    - 简单标量的组合类型
    - 需要分配资源的类型都不是可以Copy的

    例子：
    - 所有整数类型，如u32
    - 布尔类型，如bool
    - 所有浮点数类型，如f64
    - 字符类型，如char
    - 元组，当且仅当其包含的类型也都是Copy的时候，如(i32, i32)是Copy的，但是(i32, String)不是
    */
}

fn f413_owner_function() {
    println!("f413_owner_function");
    /*
    所有权与函数
    在语义上，将值传递给函数和将值赋值给变量是类似的
    将值传递给函数时，会发生移动或者复制
    返回值同样也会发生移动或者复制
    */
    let s = String::from("hello");
    takes_ownership(s);
    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }
    // println!("{}", s); //error, value borrowed here after move

    let x = 5;
    makes_copy(x);
    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }
    println!("{}", x); //x仍然可用

    /*
    如果让函数获得某个对象，但不获得它的所有权？
    Reference
    */
}