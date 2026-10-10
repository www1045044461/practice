#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// 以上特性仅适用于 nightly Rust，并且用于启用 const 泛型表达式的支持

use std::ptr;

/// 用于演示 Rust 中的常量泛型 (const generics) 的使用。
/// 在之前的泛型中，可以抽象为一句话：针对类型实现的泛型，所有的泛型都是为了抽象不同的类型，那有没有针对值的泛型？
/// 答案就是 Const 泛型。
/// Cpp中泛型使用int/long/char/short值进行实例化。比如std::array<int, 5>

//1. 定义一个固定长度的数组结构体，使用常量泛型
//下面的例子同时使用泛型和 const 泛型来实现一个结构体，该结构体的字段中的数组长度是可变的
struct ArrayPaire<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T, const N: usize> ArrayPaire<T, N> {
    fn new(left: [T; N], right: [T; N]) -> Self {
        Self { left, right }
    }
}

/// @markdown
/// 目前，const 泛型参数只能使用以下形式的实参:
/// 1. 一个单独的 const 泛型参数
/// 2. 一个字面量 (i.e. 整数, 布尔值或字符).
/// 3. 一个具体的 const 表达式( 表达式中不能包含任何 泛型参数)
fn foo<const N: usize>() {}
fn bar<T, const M: usize>() {
    foo::<M>(); // ok: 符合第一种
    foo::<2021>(); // ok: 符合第二种
    foo::<{ 20 * 100 + 20 * 10 + 1 }>(); // ok: 符合第三种

    // foo::<{ M + 1 }>(); // error: 违背第三种，const 表达式中不能有泛型参数 M //原题
    // foo::<{ std::mem::size_of::<T>() }>(); // error: 泛型表达式包含了泛型参数 T //原题

    let _: [u8; M]; // ok: 符合第一种
    // let _: [u8; std::mem::size_of::<T>()]; // error: 泛型表达式包含了泛型参数 T //原题
    // 总结: const 泛型参数的实参必须是单独的 const 泛型参数、字面量或具体的 const 表达式，不能包含泛型参数。
}

/// 3.const 泛型还能帮我们避免一些运行时检查，提升性能
pub struct MinSlice<T, const N: usize> {
    pub head: [T; N],
    pub tail: [T],
}

impl<T, const N: usize> MinSlice<T, N> {
    pub fn from_slice(slice: &[T]) -> Option<&Self> {
        if slice.len() >= N {
            let tail_len = slice.len() - N;
            let head_len = N;
            let raw_slice:*const [T] = ptr::slice_from_raw_parts(slice.as_ptr(),tail_len);
            let ptr = raw_slice as *const MinSlice<T, N>;
            Some(unsafe {
                &*ptr
            })
        } else {
            None
        }
    }
}

fn use_const_improve_performance() {
    let slice: &[u8] = b"Hello, world!!!!!!";
    let reference: Option<&u8> = slice.get(6);
    // 我们知道 `.get` 返回的是 `Some(b' ')`
    // 但编译器不知道
    assert!(reference.is_some());
    if let Some(reference) = reference {
        println!("{}", *reference);
    } else {
        panic!("Expected Some(reference), but got None");
    }

    let slice: &[u8] = b"Hello, world!!!!!!!!";

    // 当编译构建 MinSlice 时会进行长度检查，也就是在编译期我们就知道它的长度是 12
    // 在运行期，一旦 `unwrap` 成功，在 `MinSlice` 的作用域内，就再无需任何检查
    let minslice = MinSlice::<u8, 13>::from_slice(slice).unwrap();
    let value: u8 = minslice.head[6];
    println!("metasize长度为:{}", std::mem::size_of_val(&minslice));
    // println!("metasize长度为:{}", std::mem::size_of::<MinSlice<u8, 13>>()); //DST无法获取长度
    println!("metasize长度为Head:{}", std::mem::size_of_val(&minslice.head));
    println!("metasize长度为Head:{}", std::mem::size_of::<[u8;13]>());
    println!("metasize长度为Tail:{}", std::mem::size_of_val(&minslice.tail));
    // println!("metasize长度为Tail:{}", std::mem::size_of::<[u8]>()); //同上
    println!("{}",String::from_utf8_lossy(&minslice.head[..]));
    assert_eq!(value, b' ');
}

/// 练习1:
/// 🌟🌟 <T, const N: usize> 是结构体类型的一部分，和数组类型一样，
/// 这意味着长度不同会导致类型不同： Array<i32, 3> 和 Array<i32, 4> 是不同的类型

// 修复错误
struct Array<T, const N: usize> {
    data: [T; N],
}

fn test_array1() {
    // 原题
    // let arrays = [
    //     Array{
    //         data: [1, 2, 3],
    //     },
    //     Array {
    //         data: [1.0, 2.0, 3.0],
    //     },
    //     Array {
    //         data: [1, 2]
    //     }
    // ];
    let arrays = [
        Array::<i32, 3> { data: [1, 2, 3] },
        Array::<i32, 3> { data: [1, 2, 3] },
        Array::<i32, 3> { data: [1, 2, 4] },
    ];

    for array in &arrays {
        for item in &array.data {
            print!("{} \t", item);
        }
        println!();
    }
}

// 练习2:
// 填空
// fn print_array<__>(__) {
//     println!("{:?}", arr);
// }
fn print_array<T, const N: usize>(arr: [T; N])
where
    T: std::fmt::Debug,
{
    println!("{:?}", arr);
}

/// 两种等效的写法,前者是使用 `where` 约束，后者是直接在泛型参数列表中约束。
fn print_array1<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn test_print_array() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array1(arr);
}

/// 3.练习3:🌟🌟🌟 有时我们希望能限制一个变量占用内存的大小，
/// 例如在嵌入式环境中，此时 const 泛型参数的第三种形式 const 表达式 就非常适合.
#[allow(incomplete_features)]
#[feature(generic_const_exprs)]
fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    println!("类型{:?}", std::any::type_name::<T>());
    println!("size of val: {}", core::mem::size_of_val(&val));
    println!("size of type: {}", core::mem::size_of::<T>());
    println!("");
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

/********************************stable版本实现*******************************************************/
use core::marker::PhantomData; 

struct StableSizeChecker<T>(PhantomData<T>); 

impl<T> StableSizeChecker<T> {
    const SIZE: usize = core::mem::size_of::<T>();
    const ASSERT:() = {
        assert!(Self::SIZE < 768);
    };
}

fn stable_check_size<T>(value:T)
{
    let _ = StableSizeChecker::<T>::ASSERT;
    println!("size of value: {}", core::mem::size_of_val(&value));
    println!("size of type: {}", core::mem::size_of::<T>());
}
/******************************************************************************************************/

#[cfg(test)]
mod tests {
    #[test]
    fn test5() {
        test_print_array();
    }

    #[test]
    fn test_4() {
        test_array1();
    }

    #[test]
    fn test3() {
        use_const_improve_performance();
    }

    use super::*;
    #[test]
    fn test2() {
        bar::<u8, 3>();
    }

    #[test]
    fn test_array_paire() {
        let left = [1, 2, 3];
        let right = [4, 5, 6];
        let array_paire = ArrayPaire::new(left, right);
        assert_eq!(array_paire.left, [1, 2, 3]);
        assert_eq!(array_paire.right, [4, 5, 6]);
    }

    // 修复 main 函数中的错误
    #[test]
    fn test6() {
        print!("--- Const Check ---\n");
        check_size([0u8; 767]);
        check_size([0i32; 191]);
        // check_size(["hello你好"; __]); // size of &str ?
        // check_size(["hello你好"; __]); // size of &str ?
        // check_size([(); __].map(|_| "hello你好".to_string()));  // size of String?
        // check_size(['中'; __]); // size of char ?
        check_size(["hello你好"; 14]); // size of &str ?
        check_size([(); 14].map(|_| "hello你好".to_string())); // size of String?
        check_size(['中'; 2]); // size of char ?


        println!("--- Stable Check ---");

        stable_check_size([0u8; 767]);
        stable_check_size([0i32; 191]);
        stable_check_size(["hello你好"; 14]); // size of &str ?
        stable_check_size([(); 14].map(|_| "hello你好".to_string())); // size of String?
        stable_check_size(['中'; 2]); // size of char ?
    }
}
