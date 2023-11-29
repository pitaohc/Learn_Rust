/*
断言 assert
assert!
确定某个状态是否为true
- true -> 继续执行
- false -> panic!

assert_eq!
确定两个值是否相等
相当于 ==
断言失败时会输出两个参数的值
- 使用debug格式打印参数
- 要求参数实现了PartialEq和Debug trait，基本类型和大部分的标准库已经实现了

assert_ne!
确定两个值是否不相等
相当于 !=
断言失败时会输出两个参数的值
*/


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn less(left: usize, right: usize) -> bool {
    left < right
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
        let result = less(2, 3);
        assert!(result);
    }
}
