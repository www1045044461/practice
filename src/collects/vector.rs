use core::slice;

/// 1.相比 [T; N] 形式的数组， Vector 最大的特点就是可以动态调整长度。
/// 2. Vec 可以使用 extend 方法进行扩展
/// 3. 只要为 Vec 实现了 From<T> 特征，那么 T 就可以被转换成 Vec。
/// 4. 与 String 的切片类似， Vec 也可以使用切片。如果说 Vec 是可变的，那它的切片就是不可变或者说只读的，我们可以通过 & 来获取切片。
///    在 Rust 中，将切片作为参数进行传递是更常见的使用方式，例如当一个函数只需要可读性时，
///    那传递 Vec 或 String 的切片 &[T] / &str 会更加适合。
/// 5.

fn example1() {
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
    let v = vec![1, 2, 3];
    is_vec(&v);

    // ...在下面的代码中, v 是 Vec<[u8; 3]> , 而不是 Vec<u8>
    // 使用 Vec::new 和 `for` 来重写下面这段代码
    // let v1 = vec!(arr); // 这样会得到 Vec<[u8; 3]>
    let mut v1 = Vec::new();
    for item in arr {
        v1.push(item);
    }

    is_vec(&v1);

    assert_eq!(v, v1);

    println!("Success!")
}

fn is_vec(v: &Vec<u8>) {}

fn example2() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2 = Vec::new();
    // v2.__; //原题
    v2.extend([1, 2, 3]);

    assert_eq!(v1, v2);
    println!("Success!")
}

// 填空
// 原题
fn example3() {
    // // array -> Vec
    // // impl From<[T; N]> for Vec
    // let arr = [1, 2, 3];
    // let v1 = __(arr);
    // let v2: Vec<i32> = arr.__();

    // assert_eq!(v1, v2);

    // // String -> Vec
    // // impl From<String> for Vec
    // let s = "hello".to_string();
    // let v1: Vec<u8> = s.__();

    // let s = "hello".to_string();
    // let v2 = s.into_bytes();
    // assert_eq!(v1, v2);

    // // impl<'_> From<&'_ str> for Vec
    // let s = "hello";
    // let v3 = Vec::__(s);
    // assert_eq!(v2, v3);

    // // 迭代器 Iterators 可以通过 collect 变成 Vec
    // let v4: Vec<i32> = [0; 10].into_iter().collect();
    // assert_eq!(v4, vec![0; 10]);

    // println!("Success!")
}

fn example3_sln() {
    // array -> Vec
    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.into();

    assert_eq!(v1, v2);

    // String -> Vec
    // impl From<String> for Vec
    let s = "hello".to_string();
    // let v1: Vec<u8> = s.__();
    let v1 = s.into_bytes();

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    // let v3 = Vec::__(s);
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    // 迭代器 Iterators 可以通过 collect 变成 Vec
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("Success!")
}

// 修复错误并实现缺失的代码
fn example4() {
    let mut v = Vec::from([1, 2, 3]);
    // for i in 0..5 {
    for i in 0..v.len() {
        println!("{:?}", v[i])
    }

    //自己的答案
    // for i in 0..5 {
    //     // 实现这里的代码...
    //     if i < v.len() {
    //         v[i] += 1;
    //     } else {
    //         v.push(i + 2);
    //     }
    // }
    for i in 0..5 {
        ///更加具有安全性和可读性的方法来处理可变索引的情况
        if let Some(_v) = v.get_mut(i) {
            *_v += 1;
        } else {
            v.push(i + 2);
        }
    }
    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}

/// 修复错误
fn example5() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // 越界访问将导致 panic.
    // 修改时必须使用 `v.len`
    // let slice2 = &v[0..4]; //原题:错误1
    let slice2 = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    // 切片是只读的
    // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..3];
    // slice3.push(4); // 错误:切片类型不是 Vec，不能直接 push 元素
    // 可变切片无法直接增加元素，只能修改已有元素的值

    let slice3 = &mut v[0..4]; // 重新获取可变切片，包含新添加的元素

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!")
}

fn slice_vec_example1() {
    let mut v = vec![1, 2, 3];
    let slice = &v[0..2];
    // v.push(13);
    //cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("{:?}", slice);
}

fn slice_vec_example2() {
    let mut v = vec![1, 2, 3];
    let slice = &mut v[0..2];
    // v.push(14); //cannot borrow `v` as mutable more than once at a time
    println!("{:?}", slice);
}

fn slice_vec_example3() {
    let mut v = vec![1, 2, 3];
    let slice = &v[0..2];
    v.push(15); //cannot borrow `v` as mutable because it is also borrowed as immutable
    // println!("{:?}", slice);
}

fn slice_vec_example4() {
    let mut v = vec![1, 2, 3];
    let slice = &v[0..2];
    std::mem::forget(slice);
    v.push(15); //cannot borrow `v` as mutable because it is also borrowed as immutable
}

// 修复错误
// 容量问题
fn example6() {
    // let mut vec = Vec::with_capacity(10);

    // assert_eq!(vec.len(), __);
    // assert_eq!(vec.capacity(), 10);

    // // 由于提前设置了足够的容量，这里的循环不会造成任何内存分配...
    // for i in 0..10 {
    //     vec.push(i);
    // }
    // assert_eq!(vec.len(), __);
    // assert_eq!(vec.capacity(), __);

    // // ...但是下面的代码会造成新的内存分配
    // vec.push(11);
    // assert_eq!(vec.len(), 11);
    // assert!(vec.capacity() >= 11);

    // // 填写一个合适的值，在 `for` 循环运行的过程中，不会造成任何内存分配
    // let mut vec = Vec::with_capacity(__);
    // for i in 0..100 {
    //     vec.push(i);
    // }

    // assert_eq!(vec.len(), __);
    // assert_eq!(vec.capacity(), __);

    // println!("Success!")
}

fn example6_sln() {
    let mut vec = Vec::with_capacity(10);

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    // 由于提前设置了足够的容量，这里的循环不会造成任何内存分配...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(),10); // capacity should remain 10 after pushing 10 elements

    // ...但是下面的代码会造成新的内存分配
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    // 填写一个合适的值，在 `for` 循环运行的过程中，不会造成任何内存分配
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("Success!")
}

// #[derive(Debug)] //原题
#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn example7() {
    // 填空
    // let v : Vec<IpAddr>= __; //原题
    let v:Vec<IpAddr> = vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];
    
    // 枚举的比较需要派生 PartialEq 特征
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!")
}


trait IpAddrTrait {
    fn display(&self);
}

struct V4(String);
impl IpAddrTrait for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddrTrait for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

fn example8() {
    // 填空
    // let v: __= vec![
    //     Box::new(V4("127.0.0.1".to_string())),
    //     Box::new(V6("::1".to_string())),
    // ];

    let v:Vec<Box<dyn IpAddrTrait>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        example1();
    }

    #[test]
    fn test_example2() {
        example2();
    }

    #[test]
    fn test_example3() {
        example3();
    }

    #[test]
    fn test_example3_sln() {
        example3_sln();
    }

    #[test]
    fn test_example4() {
        example4();
    }

    #[test]
    fn test_example5() {
        example5();
        slice_vec_example1();
        slice_vec_example2();
        slice_vec_example3();
        slice_vec_example4();
    }

    #[test]
    fn test_example6_sln() {
        example6_sln();
    }

    #[test]
    fn test_example7() {
        example7();
    }

    #[test]
    fn test_example8() {
        example8();
    }
}
