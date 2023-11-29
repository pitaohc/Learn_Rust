/*
添加自定义的错误消息
可以向assert, assert_eq, assert_ne添加可选的自定义消息
- 自定义消息和失败消息都会被打印
- 自定义消息参数会被传递给format!宏，可以使用{}占位符
*/
pub fn add(left: usize, right: usize) -> usize {
    left + right + 1 // bug
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4, "2 + 2 = {}", result);
    }
}
