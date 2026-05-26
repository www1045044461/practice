use core::range;
use std::ops::{Range,RangeInclusive};

/// 数值类型测试

/// > Tips: 如果我们没有显式的给予变量一个类型，那编译器会自动帮我们推导一个类型
///
/// ```rust,editable
///
/// // 移除某个部分让代码工作
/// fn main() {
///     let x: i32 = 5;
///     let mut y: u32 = 5;
///
///     y = x;
///     
///     let z = 10; // 这里 z 的类型是?
/// }
/// ```
pub fn number_practice1() {
    let x: i32 = 5;
    let mut y: u32 = 5;
    y = x as u32;
    let z = 10; //这里z的类型是?
}

/// 2. 🌟
/// ```rust,editable
///
/// // 填空
/// fn main() {
///     let v: u16 = 38_u8 as __;
/// }
/// ```
#[allow(unused_variables)]
pub fn number_practice2() {
    let v: u16 = 38_u8 as u16;
}

// 修改 `assert_eq!` 让代码工作
pub fn number_practice3() {
    let x = 5;
    let _ty1 = type_of(&x);
    println!("The type of x is: {}", &_ty1);
    assert_eq!("u32".to_string(), _ty1);
}

// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// 填空，让代码工作
pub fn number_practice4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
}

pub fn number_practice5() {
    let v1 = 251_u16 + 8;
    let v2: u16 = u16::checked_add(251, 8).unwrap() as u16; //安全的加法--如果结果超出 i8 的范围，则返回 None
    println!("{},{}", v1, v2);
}

// 修改 `assert!` 让代码工作
pub fn number_practice6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597); //1579为原来的值
}

// 将 ? 替换成你的答案
pub fn number_practice7() {
    let x: f32 = 1_000.000_1; // f64 我手动设置f32也行
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    println!("x: {}, y: {}, z: {}", x, y, z);
}

/// 8. 🌟🌟 使用两种方法来让下面代码工作
///
///
///```rust,editable
///
/// fn main() {
///     assert!(0.1+0.2==0.3);
/// }
///```
pub fn number_practice8() {
    assert!(0.1_f32+0.2_f32==0.3_f32);
    assert!((0.1_f64+ 0.2 - 0.3).abs() < 0.001);
    //方法1: 使用 f32 类型来进行计算，因为 f32 的精度较低，可能会导致 0.1 + 0.2 的结果恰好等于 0.3。
    //方法2: 使用 f64 类型来进行计算，并且使用一个小的误差范围来比较结果，因为浮点数的精度问题可能会导致直接比较 0.1 + 0.2 和 0.3 不相等。
    //结论: 在处理浮点数时，直接比较可能会遇到精度问题，因此使用适当的类型和比较方法是很重要的。
}

/// 9.🌟🌟 两个目标: 1. 修改 assert! 让它工作 2. 让 println! 输出: 97 - 122
/// 
/// 
pub fn number_practice9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}",c);
    }
}

pub fn number_practice9_solution() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    } 
    //-3+1 = -2 
    //-2+0 = -2
    // -1 
    assert!(sum == -5);
    for c in 'a'..='z' {
        print!("{} ",c as u8);
    }
}

/// 10. 区间范围实例
pub fn number_practice10() {
    assert_eq!((1..5),Range{start: 1, end: 5});
    assert_eq!((1..=5),RangeInclusive::new(1, 5));
}

/// 接受一个实现了 IntoIterator 的类型，并打印其中的元素
pub fn range_print<T>(collect:T) where T: IntoIterator, T::Item: std::fmt::Display {
    for item in collect {
        println!("{}", item);
    }
    println!("------------------");
}

pub fn number_practice10_range() {
    let open_r1 = 1..5;
    let open_r2 = Range{start: 1, end: 5};
    let closed_r1 = 1..=5;
    let closed_r2 = RangeInclusive::new(1, 5);

    range_print(open_r1);
    range_print(open_r2);
    range_print(closed_r1);
    range_print(closed_r2);
}

// 填空，并解决错误
pub fn number_practice11() {
    // 整数加法
    // assert!(1u32 + 2 == __);
    assert!(1u32 + 2 == 3);

    // 整数减法
    // assert!(1i32 - 2 == __);
    assert!(1i32 - 2 == -1);
    
    // assert!(1u8 - 2 == -1);
    //  ssert!(1u8 - 2 == 255);
    
    // assert!(3 * 50 == __);
    assert!(3 * 50 == 150);

    // assert!(9.6 / 3.2 == 3.0); // error ! 修改它让代码工作
    assert!(9.6 / 3.2 == 3.0); // error ! 修改它让代码工作

    // assert!(24 % 5 == __);
    assert!(24 % 5 == 4);
    
    // 逻辑与或非操作
    // assert!(true && false == __);
    assert!(true && false == false);

    // assert!(true || false == __);
    assert!(true || false == true);

    // assert!(!true == __);
    assert!(!true == false);

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

/// try_into 和 from 的练习
#[derive(Debug)]
struct  Kilometers(f64); 

#[derive(Debug)]
struct Meters(f64);

impl Kilometers {
    fn clone(&self) -> Self {
        Kilometers(self.0)
    }
}

impl  From<Kilometers> for Meters {
    fn from(km: Kilometers) -> Self {
        Meters(km.0 * 1000.0)
    }
}

#[derive(Debug)]
struct Port(u16);
#[derive(Debug)]
enum PortError {
    InvalidPort,
    PortOutOfRange,
}

impl TryFrom<i32> for Port {
    type Error = PortError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(PortError::InvalidPort)
        } else if value > u16::MAX as i32 {
            Err(PortError::PortOutOfRange)
        } else {
            Ok(Port(value as u16))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kilometers_to_meters() {
        let km = Kilometers(1.0);
        let km2 = km.clone();
        let m: Meters = km.into();
        println!("Kilometers: {:?}, Meters: {:?}", &km2, &m);
        let m2:Meters = km2.into();
        assert_eq!(m.0, 1000.0);
    }

    #[test]
    fn test_port_try_from() {
        let p1 = Port::try_from(8080);
        let p2:Result<Port, _> = 70000.try_into();

        println!("Port 8080: {:?}, Port 70000: {:?}", &p1, &p2);
    }

    #[test]
    fn test_practice10_range() {
        number_practice10_range();
    }

    #[test]
    fn test_practice11_solution() {
        number_practice11();
    }
}
