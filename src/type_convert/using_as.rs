/// 1. Rust 并没有为基本类型提供隐式的类型转换( coercion )，但是我们可以通过 as 来进行显式地转换。

// 修复错误，填空
// 不要移除任何代码
fn question1() {
    // let decimal = 97.123_f32;

    // let integer: __ = decimal as u8;

    // let c1: char = decimal as char;
    // let c2 = integer as char;

    // assert_eq!(integer, 'b' as u8);
    // println!("Success!")
}

fn answer1() {
    let decimal = 97.123_f32;
    let integer: u8 = decimal as u8;

    let c1: char = decimal as u8 as char; // 通过多重转换
    let c2 = integer as char;

    assert_eq!(integer, 'a' as u8);
    println!("Success!")
}

/// 允许溢出字面量
#[allow(overflowing_literals)]
fn question2() {
    assert_eq!(u8::MAX, 255);
    // 如上所示，u8 类型允许的最大值是 255.
    // 因此以下代码会报溢出的错误： literal out of range for `u8`.
    // **请仔细查看相应的编译错误，从中寻找到解决的办法**
    // **不要修改 main 中的任何代码**
    let v = 1000 as u8;
    println!("Success!")
}

// 当将任何数值转换成无符号整型 T 时，如果当前的数值不在新类型的范围内，
// 我们可以对当前数值进行加值或减值操作( 增加或减少 T::MAX + 1 )，直到最新的值在新类型的范围内，
// 假设我们要将 300 转成 u8 类型，由于u8 最大值是 255，因此 300 不在新类型的范围内并且大于新类型的最大值，
// 因此我们需要减去 T::MAX + 1，也就是 300 - 256 = 44。
fn question3() {
    // assert_eq!(1000 as u16, __);

    // assert_eq!(1000 as u8, __);

    // // 事实上，之前说的规则对于正整数而言，就是如下的取模
    // println!("1000 mod 256 is : {}", 1000 % 256);

    // assert_eq!(-1_i8 as u8, __);
    

    // // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
    // assert_eq!(300.1_f32 as u8, __);
    // assert_eq!(-100.1_f32 as u8, __);
    

    // // 上面的浮点数转换有一点性能损耗，如果大家对于某段代码有极致的性能要求，
    // // 可以考虑下面的方法，但是这些方法的结果可能会溢出并且返回一些无意义的值
    // // 总之，请小心使用
    // unsafe {
    //     // 300.0 is 44
    //     println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
    //     // -100.0 as u8 is 156
    //     println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
    //     // nan as u8 is 0
    //     println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    // }
}
#[allow(overflowing_literals)]
fn answer3() {
    assert_eq!(1000 as u16, 1000);
    assert_eq!(1000 as u8, 1000-256);
    // 事实上，之前说的规则对于正整数而言，就是如下的取模
    println!("1000 mod 256 is : {}", 1000 % 256);
    assert_eq!(-1_i8 as u8, 255);
    

    // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
    // assert_eq!(300.1_f32 as u8, (300-256) as u8); //个人理解错误
    assert_eq!(300.1_f32 as u8, u8::MAX); // 超出范围取最大值
    // assert_eq!(-100.1_f32 as u8, (256-100) as u8);
    assert_eq!(-100.1_f32 as u8, u8::MIN); // 超出范围取最小值
    

    // 上面的浮点数转换有一点性能损耗，如果大家对于某段代码有极致的性能要求，
    // 可以考虑下面的方法，但是这些方法的结果可能会溢出并且返回一些无意义的值
    // 总之，请小心使用
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}


// 填空
/// 裸指针可以和代表内存地址的整数互相转换
fn question4() {
    // let mut values: [i32; 2] = [1, 2];
    // let p1: *mut i32 = values.as_mut_ptr();
    // let first_address: usize = p1 __; 
    // let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    // let p2: *mut i32 = second_address __; // p2 指向 values 数组中的第二个元素
    // unsafe {
    //     // 将第二个元素加 1
    //     __
    // }
    
    // assert_eq!(values[1], 3);

    // println!("Success!")
}

fn answer4() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize; 
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // p2 指向 values 数组中的第二个元素
    unsafe {
        // 将第二个元素加 1
        *p2 += 1;
    }
    
    assert_eq!(values[1], 3);

    //打印修改前后的值
    println!("After modification: {}", values[1]);

    println!("Success!")
}

fn question5() {
    // let arr :[u64; 13] = [0; 13];
    // assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    // let a: *const [u64] = &arr;
    // let b = a as *const [u8];
    // unsafe {
    //     assert_eq!(std::mem::size_of_val(&*b), __)
    // }
}

fn answer5() {
    let arr :[u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13); 
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        // assert_eq!(std::mem::size_of_val(&*b),8*13);
        // 错误: 胖指针copy后无法正确获取大小
        assert_eq!(std::mem::size_of_val(&*b), 13);
    }
}

enum Value{
    Bool(bool),
    Int(i32),
    Tuple(i32,i32),
}

fn matches()
{
    let vec = vec![
        Value::Bool(true),
        Value::Int(42),
        Value::Tuple(1, 2),
    ];

    for v in vec.iter() {
        match v {
            Value::Bool(b) => println!("Bool: {}", b),
            Value::Int(i) => println!("Int: {}", i),
            Value::Tuple(a, b) => println!("Tuple: ({}, {})", a, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer1() {
        answer1();
    }
    
    #[test]
    fn test_question2() {
        question2();
    }
    
    #[test]
    fn test_answer3() {
        answer3();
    }

    #[test]
    fn test_answer4() {
        answer4();
    }

    #[test]
    fn test_answer5() {
        answer5();
        matches();
    }
}