fn main() {
    /*
    Rust错误处理
    - 大部分情况：编译期提示错误，并处理
    错误分类：
    - 可恢复
        - 文件路径不存在, 网络连接中断等, 可再次尝试, 不用终止程序
    - 不可恢复(bug)
        - 索引越界等

    Rust没有类似异常的处理机制
    - 可恢复错误：返回Result<T, E>
    - 不可恢复错误：panic!
    */

    /*
    panic!宏 恐慌
    - 会打印错误信息，
    - 开始回溯，清理栈数据，
    - 退出程序

   当panic发生时：
    - 程序展开调用栈(工作量大)(默认操作)
        - Rust沿着调用栈返回
        - 清理每个遇到的函数中的数据
    - 立即终止调用栈
        - 不清理，直接停止程序
        - 内存需要OS清理

    立即终止的程序二进制文件更小
    - 在cargo.toml中，[profile.release]下，panic = 'abort'
    */
    println!("Hello, world!");
    // panic!("crash and burn"); // 立即终止

    let v = vec![1, 2, 3];
    v[99]; // 索引越界，立即终止

    /*
    如果需要显示调试信息，在构建时添加--release
    cargo run --release
    */
}
