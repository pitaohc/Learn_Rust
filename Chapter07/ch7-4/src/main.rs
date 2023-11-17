use std::io;
use std::fmt;
fn main() {
    /*
    use关键字
    使用use将路径导入到作用域内
    仍然遵循私有性规则
    */
    /*
    例子见lib.rs
    */

    /*
    同名条目：指定到父级
    */

    fn function1() -> fmt::Result {
        // --snip--
        Ok(())
    }
    fn function2() -> io::Result<()> {
        // --snip--
        Ok(())
    }
    /*
    as关键字
    指定本地别名
    和python类似
    use std::io::Result as IoResult;
    */

    /*
    7.5
    使用use将路径导入到作用域后，该名称在此作用域是私有的
    即使该作用域被其他作用域包含，也无法访问
    同样可以使用pub关键字，使得该名称在此作用域是公有的
    */

}
