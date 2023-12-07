/*
release profile
预定义的发型配置方案
可以自定义：使用不同的配置，对代码编译拥有更多的控制
每个profile都独立于其他配置

主要的两个profile
- dev profile: 适用于开发 cargo build 命令调用
- release profile: 适用于生产环境 cargo build --release 命令调用
*/
/*
自定义profile
在Cargo.toml中添加如下内容
[profile.xxx]
在里面覆盖默认配置的子集
*/
fn main() {
    println!("Hello, world!");
}
