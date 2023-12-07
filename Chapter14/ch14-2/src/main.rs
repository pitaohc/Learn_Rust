/*
可以通过crates.io发布包来共享代码
crate的注册表在https://crates.io/
- 分发已注册的包的源代码
- 托管开源代码
*/
/*
文档注释
/// 用于文档注释
html文档
显示公共API的文档注释：告诉用户如何使用API
支持Markdown格式
放置在被说明条目之前

可以使用cargo doc生成文档
会运行rustdoc工具
文档会放在target/doc目录下
--open参数会自动打开浏览器
*/
/*
常用章节
# Examples
# Panics
    描述可能会发生panic的场景
# Errors
    描述可能会返回Result::Err的场景，以及可能会导致错误的条件
# Safety
    描述unsafe代码的不变量。如果函数处于unsafe调用，就应该解释函数unsafe的原因，以及调用者确保的使用前提
*/
/*
文档注释作为测试
运行cargo test会运行文档注释中的代码
*/
/*
为包含注释的项添加文档注释
为上一层级的项添加文档注释
//!
见lib.rs
*/
fn main() {
    println!("Hello, world!");
}
