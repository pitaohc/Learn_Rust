/*
测试(函数)
验证函数的功能是否和预期的一致

测试函数体通常的三个操作：
- 准备数据/状态
- 调用被测试代码
- 断言(Assert)结果是否符合预期
*/

/*
解剖测试函数
测试函数需要使用test属性(attribute)进行标注
- #[test]标注的函数是测试函数
- attribute是一段rust代码的元数据(metadata)
*/

/*
运行测试
cargo test
rust会构建一个test runner来运行测试
会运行标注了test的函数，并报告是否成功

当使用cargo创建library项目时，会生成一个test module，里面有一个默认的test函数
当然可以添加任意数量的test module或test函数

cargo new adder --lib // 创建一个library项目
*/


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)] // 只有在测试时才编译 cfg代变为configuration
mod tests {
    use super::*;

    #[test] // 标注为测试函数
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_fail() {
        /*
        测试函数panic表示测试失败
        每个测试函数运行在一个单独的线程
        当主线程检测某个测试线程挂掉了，就会认为测试失败
        */
        let result = add(2, 2);
        assert_eq!(result, 1); // 会报错
    }
}
