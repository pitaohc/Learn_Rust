use add_one;

/*
在工作空间中依赖外部的crate

工作空间中只有一个cargo.lock文件，在工作空间的顶层目录
这保证了工作空间中所有crate都使用相同版本的依赖
即使在子crate中使用了不同的版本，也会被cargo解析为相同的版本
*/
fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
