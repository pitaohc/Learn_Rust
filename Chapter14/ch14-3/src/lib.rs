//!
//! # 使用pub use 导出方便调用的公共API
//!
//! - 问题：crate的程序结构在开发时对于开发者很合理，但是对调用者不够方便。
//! 开发者会把程序结构分为很多层，使用者想找到这种深层结构中的某个类型很费劲
//!
//! - 例子：标准库中的std::io::ErrorKind::NotFound
//!
//! - 解决：使用pub use导出方便调用的公共API,可以重新导出，
//! 创建一个与内部私有结构不同的对外公共结构
//!

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// 一级颜色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    /// 二级颜色
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}
pub mod utils{
    use crate::kinds::*;
    /// 混合颜色
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}