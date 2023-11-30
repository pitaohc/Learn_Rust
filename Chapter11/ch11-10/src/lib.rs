/*
集成测试
- 继承测试位于被测试库的外部
- 目的：测试库的多个部分是否能正常协同工作
- 集成测试的覆盖率很重要

创建tests目录
与src目录平级
tests目录下的每个测试文件都是单独的一个crate
- 需要导入被测试库
- 不需要cfg

运行指定的集成测试
- 运行特定的集成测试：cargo test 函数名
- 运行某个测试文件内的所有测试：cargo test --test 文件名
*/

/*
集成测试中的子模块
tests目录下每个文件被编译为单独的crate
- 这些文件不共享行为（与src下的文件规则不同）
- 子目录不会被单独编译
*/

/*
针对binary crate的集成测试
如果项目是binary crate，只含有src/main.rs而没有src/lib.rs
- 不能在tests目录下创建集成测试
- 无法把main.rs的函数导入作用域

只有library crate才能暴露函数给其他crate
binary crate意味着独立运行

因此一般main.rs中的代码很少
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
