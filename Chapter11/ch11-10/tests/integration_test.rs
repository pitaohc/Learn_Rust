mod common; // 导入common模块

use ch11_10; // 导入被测试的crate

#[test] // 不需要cfg，tests目录下的内容只会在cargo test时编译
fn it_works() {
    common::setup(); // 调用common模块中的setup函数
    let result = ch11_10::add(2, 2);
    assert_eq!(result, 4);
}