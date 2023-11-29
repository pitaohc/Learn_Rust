/*
测试分类
- 单元测试
- 集成测试

单元测试：
- 小，专注
- 一次对一个模块进行隔离测试
- 可测试private接口

集成测试：
- 在库外部，和其他外部代码一样使用我们编写的代码
- 只能访问public接口
- 会访问多个模块


#[cfg(test)]：只有在测试时才编译
集成测试在不同的目录，不需要#[cfg(test)]


cfg configuration
- 告诉Rust，下面的条目只有在指定的配置选项下才被包含
- test配置：由rust提供，用于编译和运行测试
    - 只有cargo test才会编译代码，包括模块中的helper函数和#[test]标记的代码
*/

/*
测试私有函数
与普通测试一样
*/
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn internal_adder(left: usize, right: usize) -> usize {
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
    fn test_internal() {
        assert_eq!(internal_adder(2, 2), 4);
    }
}
