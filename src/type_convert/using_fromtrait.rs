///
/// 1. From 特征允许让一个类型定义如何基于另一个类型来创建自己，因此它提供了一个很方便的类型转换的方式。
/// 2. 通过实现 From 特征，可以使用 `from` 方法进行类型转换。
/// 3. 通常情况下，如果实现了 From 特征，Rust 会自动为你实现相应的 Into 特征。
/// 4. TryFrom 特征允许进行可能失败的类型转换，它返回一个 Result 类型，表示转换成功或失败。
/// 
/// 实例:
///     当执行错误处理时，为我们自定义的错误类型实现 From 特征是非常有用。这样就可以通过 ? 自动将某个错误类型转换成我们自定义的错误类型
/// 
/// 

struct NewI32(i32);

impl From<char> for NewI32 {
    fn from(c: char) -> Self {
        NewI32(c as i32)
    }
}

// From 被包含在 `std::prelude` 中，因此我们没必要手动将其引入到当前作用域来
// use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// impl From<i32> for Number {
//     // 实现 `from` 方法
// }
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

// example3: 使用 From 特征实现自定义错误类型的转换
use std::fs;
use std::io;
use std::num;

#[derive(Debug)]
enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    // 实现 from 方法
    fn from(value: io::Error) -> Self {
        CliError::IoError(value)
    }
}

impl From<num::ParseIntError> for CliError {
    // 实现 from 方法
    fn from(value: num::ParseIntError) -> Self {
        CliError::ParseError(value)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? 自动将 io::Error 转换成 CliError
    let contents = fs::read_to_string(&file_name)?;
    let _contents = fs::read_to_string(file_name);
    //? 操作符:通过返回值Result自动将错误类型转换成CliError
    // let contents2 = match _contents {
    //     Ok(c) => c,
    //     Err(e) => return Err(CliError::IoError(e)),
    // };
    // 真实实现功能
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn example4() {
    // println!("Success!")
    let result = open_and_parse_file("./1.txt");
    match result {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Error: {:?}", e),
    }
}

// 填空
fn example3() {
    // let num = __(30);
    // assert_eq!(num.value, 30);
    // let num: Number = __;
    // assert_eq!(num.value, 30);
    // println!("Success!");
    let num = Number::from(30);
    assert_eq!(num.value, 30);
    let num: Number = 30.into();
    assert_eq!(num.value, 30);
    println!("Success!")
}


// TryFrom 和 TryInto 也被包含在 `std::prelude` 中, 因此以下引入是没必要的
// use std::convert::TryInto;
fn question4() {
    // let n: i16 = 256;

    // // Into 特征拥有一个方法`into`,
    // // 因此 TryInto 有一个方法是 ?
    // let n: u8 = match n.__() {
    //     Ok(n) => n,
    //     Err(e) => {
    //         println!("there is an error when converting: {:?}, but we catch it", e.to_string());
    //         0
    //     }
    // };
    // assert_eq!(n, __);
    // println!("Success!");
}

// TryFrom 和 TryInto 也被包含在 `std::prelude` 中, 因此以下引入是没必要的
// use std::convert::TryInto;

fn question4_sln() {
    let n: i16 = 256;

    // Into 特征拥有一个方法`into`,
    // 因此 TryInto 有一个方法是 ?
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    // assert_eq!(n,255);
    assert_eq!(n, 0); //错误
    println!("Success!")
}

#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // 实现 `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}

fn question5() {
    // assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    // assert_eq!(EvenNum::try_from(5), Err(()));
    // // 填空
    // let result: Result<EvenNum, ()> = 8i32.try_into();
    // assert_eq!(result, __);
    // let result: Result<EvenNum, ()> = 5i32.try_into();
    // assert_eq!(result, __);
    // println!("Success!");
}

fn question5_sln() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // 填空
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("Success!")
}

#[cfg(test)]
mod tests {
    use core::alloc;

    #[test]
    fn test_example1() {
        let native_str: &str = "hello";
        let s = String::from(native_str);
        let _str = native_str.to_string();
        // alloc::string::&str::to_string(&self)//函数定义位置
        let s2: String = _str.into();
        //into和from是相互关联的，From实现了Into也会自动实现
        //同时into需要显式指定目标类型，否则编译器无法推断
    }

    #[test]
    fn test_example2() {
        use super::NewI32;
        // impl From<bool> for i32
        let i1: i32 = false.into();
        let i2: i32 = i32::from(false);
        assert_eq!(i1, i2);
        assert_eq!(i1, 0);

        // 使用两种方式修复错误
        // 1. 哪个类型实现 From 特征 : impl From<char> for ? , 你可以查看一下之前提到的文档，来找到合适的类型
        // 2. 上一章节中介绍过的某个关键字
        // let i3: i32 = 'a'.into();
        let i3: i32 = 'a' as i32; //方法1:使用as
        let i4: u32 = 'a'.into(); //方法2:使用into,但是i32需要改成u32

        // 使用两种方法来解决错误
        // let s: String = 'a' as String;
        let s1 = 'a'.to_string(); //方法1:使用to_string(),使用SpecToString实现静态分发
        let s2: String = 'a'.into(); //方法2:使用into,需要显式指定目标类型
        let s3 = String::from('a'); //方法3:使用String::from

        println!("Success!")
    }
    #[test]
    fn test_example3() {
        super::example3();
    }

    #[test]
    fn test_example4() {
        super::example4();
    }
    #[test]
    fn test_example5() {
        super::question4_sln();
    }
    #[test]
    fn test_example6() {
        super::question5_sln();
    }
}
