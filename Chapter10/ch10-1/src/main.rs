fn largest(list: &[i32]) -> i32 {
    let mut largest_num = list[0];
    /*
    这里的 &num 是什么意思？ 为什么不是 num
    因为 list 是一个引用，所以 list 中的元素也是引用， 使用&操作符来获取引用的值
    */
    for &num in list {
        if largest_num < num {
            largest_num = num;
        }
    }
    return largest_num;
}


fn main() {
    /*
    复用并抽象函数
    */
    let nums = [3, 2, 1, 5, 7, 4];
    let largest_num = largest(&nums);
    println!("The largest number in list [{:?}] is {}", nums, largest_num);
    let nums = [1, 0, 8, 4, 2, 1, 2];
    let largest_num = largest(&nums);
    println!("The largest number in list [{:?}] is {}", nums, largest_num);

    /*
    消除重复代码的步骤
    - 识别重复代码
    - 提取重复代码到函数体中，并在签名中指定参数和返回值
    */
}
