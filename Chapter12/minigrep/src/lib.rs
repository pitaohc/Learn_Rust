use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        /*
        env::var 返回一个Result, 如果环境变量存在，返回Ok，否则返回Err
        is err 检查Result是否是Err, 如果是Err返回true，否则返回false
        CASE_INSENSITIVE 不区分大小写的环境变量
        */
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query: query.to_string(), filename: filename.to_string(), case_sensitive })
    }
}


/// Config结构体
///
/// query: String, 查询字符串
/// filename: String, 文件名
/// case_sensitive: bool, 是否区分大小写
///
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

/// 大小写敏感的搜索字符串函数
///
/// # Arguments
///
/// * `query`: 查询字符串
/// * `contents`: 文件内容
///
/// returns: Vec<&str, Global> 包含查询字符串的行
///
pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

/// 大小写不敏感的搜索字符串函数
///
/// # Arguments
///
/// * `query`: 查询字符串
/// * `contents`: 文件内容
///
/// returns: Vec<&str, Global> 包含查询字符串的行
///
pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    /*
    将query转换为小写,新query拥有所有权
    不能在此调用as_str, 因为to_lowercase()返回的String会在此行调用结束后被删除
    */
    let query = query.to_lowercase();

    let mut result: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct";

        assert_eq!(vec!["safe, fast, productive."],
                   search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct";

        assert_eq!(vec!["safe, fast, productive.", "Duct"],
                   search_case_insensitive(query, contents));
    }
}