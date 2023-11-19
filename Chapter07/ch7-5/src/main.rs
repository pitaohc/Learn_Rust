use rand::Rng;
use std::io::{self,Write};
fn main() {
    /*
    Cargo.toml 添加依赖的包
    use关键字将特定条目引入作用域

    std标准库也被当做外部包
    - 默认包含
    - 不需要修改cargo.toml
    - 需要使用use关键字引入std中的特定条目
    */

    /*
    使用嵌套路径引入多个条目
    use std::io::{self,Write};
    */

    /*
    通配符
    use std::collections::*;
    引入collections中的所有条目

    注意：谨慎使用通配符导入
    - 可能引入多个同名的条目

    应用场景：
    - 测试代码，引入所有的测试函数
    - 用于预导入模块
    */
}
