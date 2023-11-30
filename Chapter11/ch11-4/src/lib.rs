/*
使用should_panic检查panic

验证错误处理的情况
测试除了验证代码的返回值是否正确，还需要验证代码是否如预取的处理了发生错误的情况
可验证代码在特定情况下是否发生了panic
should_panic属性(attribute):
- 函数panic时测试通过
- 函数没有panic时测试失败

should_panic属性可以接受一个可选的expected参数，用于测试代码panic的错误信息
- 如果测试代码panic了，但错误信息不符合预期，测试失败
*/

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn div(left: usize, right: usize) -> usize {
    if right == 0 {
        panic!("div by zero");
    }
    left / right
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
    fn test_div1() {
        let result = div(2, 2);
        assert_eq!(result, 1);
    }

    #[test]
    // #[should_panic = "div by zero"] // 验证panic的错误信息
    #[should_panic(expected = "div by zero")] // 与上一行写法效果一致
    fn test_div_zero(){
        div(2, 0); // 除数为0，会panic
    }
}
