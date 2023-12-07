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
    /*
    修改为迭代器访问
    */
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next(); // 跳过第一个参数, 因为它是程序名
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next()
        {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

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

    // let mut result: Vec<&str> = vec![];
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    // result

    /*
    使用迭代器
    filter 返回闭包返回true的所有元素
    */
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
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

    // let mut result: Vec<&str> = vec![];
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line);
    //     }
    // }
    // result


    /*
    此处query不同于search中的形式的原因
    search中的query 是&str字符串切片
    而此处为String
    contains需要一个字符串切片类型
    */
    contents.lines()
        .filter(|line| line.contains(&query))
        .collect()
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