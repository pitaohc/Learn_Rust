/*
控制测试的运行方式
改变cargo test的默认行为：添加命令行参数
默认行为：
- 并行运行
- 所有测试
- 捕获(不显示)所有成功的测试输出，使阅读跟测试结果相关的输出更容易

命令行参数：
- 针对cargo test的参数，紧跟cargo test 后
- 针对被测试的可执行程序：放在 -- 后

e.g.
cargo test --help
cargo test -- --help
*/

/*
并行运行测试
运行多个测试：默认使用多个线程并行运行
- 运行快
需要确保测试之间
- 不会相互依赖
- 不依赖于某个共享状态(环境、工作目录、环境变量等)

使用--test-threads参数
- 传递给测试程序 放在 -- 后
- 以单线程运行或者精确控制线程数量
- e.g. cargo test -- --test-threads=1 以单线程运行
*/

/*
显示函数输出
默认情况：只显示失败的测试的输出
使用参数 --show-output显示所有输出
放在 -- 后
*/
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
