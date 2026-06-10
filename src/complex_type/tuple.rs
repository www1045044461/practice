/// 元组类型测试
/// 类型 (T1, T2, T3, ...)，每个元素可以是不同的类型
/// 

fn question1() {
    let _t0: (u8,i16) = (0, -1);
    // 元组的成员还可以是一个元组
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // 填空让代码工作
    // let t: (u8, __, i64, __, __) = (1u8, 2u16, 3i64, "hello", String::from(", world")); //原题
    let t: (u8,u16, i64,&str,String) = (1u8, 2u16, 3i64, "hello", String::from(", world")); //原题

    println!("t: {:?}", t);
    dbg!(t);
}


// 🌟 可以使用索引来获取元组的成员
// 修改合适的地方，让代码工作
fn question2() {
    let t = ("i", "am", "sunface");
    // assert_eq!(t.1, "sunface");
    assert_eq!(t.2, "sunface");
}


// 修复代码错误
fn question3() {
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9);
    // println!("too long tuple: {:?}", too_long_tuple); 
    println!("too long tuple: {:?}", too_long_tuple);
}


/// 🌟 使用模式匹配来解构元组
fn question4() {
    let tup = (1, 6.4, "hello");

    // 填空
    // let __ = tup; //原题
    let (x,z,y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
}

/// 🌟🌟 解构式赋值
fn question5() {
    let (x, y, z);

    // 填空
    // __ = (1, 2, 3);
    (y, z, x) = (1, 2, 3);
    
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2); 

    let tups;
    tups = (2,3,4,6);
    println!("tups: {:?}", tups);
}


/// 🌟🌟 元组可以用于函数的参数和返回值
fn question6() {
    // 填空，需要稍微计算下
    // let (x, y) = sum_multiply(__);
    let (x, y) = sum_multiply((2,3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question1() {
        // question1();
        // question2();
        // question3();
        // question4();
        // question5();
        question6();
    }
}