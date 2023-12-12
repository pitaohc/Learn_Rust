/*
Drop Trait
自定义当值离开作用域时执行的动作
如：文件、网络资源的释放
*/
struct MyBox {
    data: String,
}

impl Drop for MyBox {
    fn drop(&mut self) {
        println!("MyBox is dropped with data '{}'", self.data);
    }
}

fn example_1() {
    let c = MyBox { data: String::from("hello") };
    let d = MyBox { data: String::from("d") };
    println!("MyBox c and d are created");
    /*
    先Drop d
    再Drop c
    因为局部变量存储在栈上，先进后出
    */
}
/*
使用std::mem::drop主动调用drop
Rust不允许我们主动调用Drop trait的drop方法

drop 会使用移动语义修改其参数的所有权
调用过drop后，不能再使用该变量
而drop trait由用户自定义，可能会有不同的实现，所以不能保证drop后变量的状态

*/
fn example_2() {
    let a = MyBox { data: String::from("a") };
    // a.drop();
    drop(a);
    println!("MyBox a is dropped");
    // drop(a); // error E0382: borrow of moved value: `a`
}

fn main() {
    example_1();
    example_2();
}
