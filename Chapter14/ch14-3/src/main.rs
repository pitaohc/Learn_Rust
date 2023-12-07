
// // 三层引入
// use ch14_3::kinds::PrimaryColor;
// use ch14_3::utils::mix;
// fn main() {
//     let red = PrimaryColor::Red;
//     let yellow = PrimaryColor::Yellow;
//     mix(red, yellow);
// }

use ch14_3::PrimaryColor; // pub use后，可以直接使用PrimaryColor
use ch14_3::mix;
fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}