//! # ch14_2 crate文档注释
//! 此处是对lib.rs的文档注释
//!
//! Rust是世界上最好的语言

///
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = ch14_2::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}
