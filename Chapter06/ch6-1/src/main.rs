#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}
fn main() {
    let loopback = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback2 = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("loopback: {:?}", loopback);
    println!("loopback2: {:?}", loopback2);
    /*
    允许将数据附加到枚举的变体中
    enum IpAddr{
        V4(String),
        V6(String),
    }
    优点：
    - 不需要额外使用struct存放数据
    - 每个变体可以拥有不同的类型以及关联的数据量
    */
    #[derive(Debug)]
    enum NewIpAddr{
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let new_loopback = NewIpAddr::V4(127, 0, 0, 1);
    let new_loopback2 = NewIpAddr::V6(String::from("::1"));
    println!("new_loopback: {:?}", new_loopback);
    println!("new_loopback2: {:?}", new_loopback2);
    /*
    为枚举定义方法
    使用impl关键字为枚举定义方法
    */
}

fn route(ip_kind: IpAddrKind){
    println!("ip_kind: {:?}", ip_kind);
}