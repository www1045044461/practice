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
}