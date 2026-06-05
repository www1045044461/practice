/// 切片跟数组相似，但是切片的长度无法在编译期得知，因此你无法直接使用切片类型。
/// 一个切片引用占用了2个字大小的内存空间
/// ( 从现在开始，为了简洁性考虑，如无特殊原因，我们统一使用切片来特指切片引用 )。
///  该切片的第一个字是指向数据的指针，第二个字是切片的长度。
/// 字的大小取决于处理器架构，例如在 x86-64 上，字的大小是 64 位也就是 8 个字节，
/// 那么一个切片引用就是 16 个字节大小。
/// 
///切片( 引用 )可以用来借用数组的某个连续的部分，对应的签名是 &[T]，
/// 大家可以与数组的签名对比下 [T; Length]。


// 修复代码中的错误，不要新增代码行!
fn question1() {
    let arr = [1, 2, 3];
    // let s1: [i32] = arr[0..2]; // 原题-错误:编译器无法推导出切片的长度
    let s1 = &arr[0..2]; //方法1

    // let s2: str = "hello, world" as str; //原题
    //错误原因: `str` 是一个动态大小类型，不能直接使用它来声明变量。
    //我们需要使用 `&str` 来表示字符串切片。
    let s2: &str = "hello, world"; //方法2
}


fn question2() {
    let arr: [char; 4] = ['中', '国', '人','好'];

    let slice = &arr[..2];
    let slice2 = &arr[0..3];
    let slice3 = &arr[0..4];
    
    let ss1 = &arr[2];
    
    // 修改数字 `8` 让代码工作
    // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
    // assert!(std::mem::size_of_val(&slice) == 8); //原题
    assert!(std::mem::size_of_val(&slice) == 16); 
    println!("2长度slice:{}  
        3长度slice:{}
        4长度slice:{}",
        std::mem::size_of_val(&slice),
        std::mem::size_of_val(&slice2),
        std::mem::size_of_val(&slice3));

    println!("单个指针长度:{}", std::mem::size_of_val(&ss1));
}


fn question3() {
   let arr: [i32; 5] = [1, 2, 3, 4, 5];
  // 填空让代码工作起来
  // let slice: __ = __; //原题
  let slice = &arr[1..4]; //方法1
  assert_eq!(slice, &[2, 3, 4]);
}


/// 字符串切片
fn question4() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // 填空，不要再使用 0..2
    // let slice2 = &s[__];
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);
}


fn question5() {
    let s = "你好，世界";
    // 修改以下代码行，让代码工作起来
    // let slice = &s[0..2];
    let slice = &s[0..3];
    //UTF8编码中，中文字符占用3个字节，
    //因此需要使用0..3来获取第一个中文字符的切片。

    println!("slice: {}", &slice);
    assert!(slice == "你");
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
        question5();
    }
}