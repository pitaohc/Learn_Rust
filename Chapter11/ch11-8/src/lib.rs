/*
忽略某些测试，运行剩余测试
使用ignore属性(attribute)标记
使用--ignored参数单独运行被忽略的测试
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
    #[ignore]
    fn it_works_2() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
