enum SpreadsheetCell { // 单元格类型，可以存放三种类型的数据
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    /*
    使用enum存储多种数据类型
    - Enum的变体可以附加不同类型的数据
    - Enum的变体定义在同一个enum类型下
    为什么vec需要知道所有元素的类型？
    为了在堆内存上分配内存，需要知道所有元素的大小

    如果不知道所有元素的类型，需要使用trait对象
    */
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    for elem in &row{
        match elem {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }
}
