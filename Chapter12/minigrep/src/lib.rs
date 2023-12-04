use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    /*
    Box<dyn Error>是一个trait对象，它可以表示任何实现Error trait的类型
    */
    let contents = fs::read_to_string(config.filename)?;
    /*
    ?运算符会将Result中的值取出并返回，如果Result是Err，?运算符会提前返回整个函数
    */
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

impl Config {
    /*
    使用Result来处理错误
    &‘static str是字符串字面值的类型，它的生命周期是静态的
    */
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        /*
        不能夺取query和filename的所有权，因为parse_config()需要返回Config实例
        to_string()会从&str创建一个String
        */
        Ok(Config { query: query.to_string(), filename: filename.to_string() })
    }
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];
    for line in contents.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        /*
        先编写测试，测试会失败。
        再修改函数功能，让函数符合测试，类似算法题的思路。
        */
        assert_eq!(vec!["safe, fast, productive."],
                   search(query, contents));
    }
}