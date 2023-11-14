fn main() {
    /*
    控制流：循环
    loop, while, for
    */

    /*
    loop 会无限循环，直到遇到 break
    */
    let mut counter = 10;
    loop {
        println!("{} again!", counter);
        counter -= 1;
        if counter == 0 {
            break;
        }
    }
    /*
    while 循环前判断条件
    */
    while counter != 10 {
        println!("{} again!", counter);
        counter += 1;
    }

    /*
    for循环遍历集合
    */
    let nums = [1, 2, 3, 4, 5];
    for num in nums.iter() {
        print!("{}, ", num);
    }
    println!();
    /*
    使用下标遍历
    */
    for num in 0..nums.len() {
        println!("nums[{}] = {}", num, nums[num]);
    }
    /*
    Range
    标准库中的Range类型，它是一个范围的迭代器（iterator）。
    指定一个范围，然后可以对这个范围进行迭代。
    rev() 反转Range
    */
    for num in (1..4).rev() { // [1, 4)
        println!("{}!", num);
    }
}
