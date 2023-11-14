fn main() {
    /*
    if let 一个简单的模式匹配
    只有一个分支的match
    */
    let _v = Some(3);

    if let Some(5) = _v  { // _v = Some(5) 的结果和 Some(5) = _v不一致
        println!("v = {:?}", _v);
    }
    else { //可以搭配 else 使用
        println!("v is not 5, v = {:?}", _v);
    }
}
