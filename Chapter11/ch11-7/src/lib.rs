
/*
按名称运行测试
指定需要运行的测试函数
将测试的名称(一个或多个)作为参数传递给 cargo test 命令
- 运行单个测试
cargo test test_name
- 运行多个测试
通过指定测试名的一部分
cargo test it
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
    #[test]
    fn it_works2() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
