/*
在测试中使用Result<T, E>
无需panic,可使用Result<T, E>作为测试函数的返回类型
- OK(value) => 测试通过
- Err(err) => 测试失败

注意：不要在使用#[should_panic]的同时使用Result<T, E>
返回值是Err的情况下，测试会失败，但是不会panic
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
    fn it_works2() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
