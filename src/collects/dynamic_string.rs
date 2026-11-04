/// 1. std::string::String 是 UTF-8 编码、可增长的动态字符串.
/// 它也是我们日常开发中最常用的字符串类型，同时对于它所拥有的内容拥有所有权。
/// 2. 虽然 String 的底层是 Vec<u8> 也就是字节数组的形式存储的，但是它是基于 UTF-8 编码的字符序列。
/// String 分配在堆上、可增长且不是以 null 结尾。
/// 3. 而 &str 是切片引用类型( &[u8] )，指向一个合法的 UTF-8 字符序列，总之，&str 和 String
/// 的关系类似于 &[T] 和 Vec<T>
/// 4. 由于 String 都是 UTF-8 编码的，这会带来几个影响:
///     如果你需要的是非 UTF-8 字符串，可以考虑 OsString
///     无法通过索引的方式访问一个 String
/// 具体请看字符串索引。
/// 5. 我们可以使用 utf8_slice 来按照字符的自然索引方式对 UTF-8 字符串进行切片访问，
///     与之前的切片方式相比，它索引的是字符，而之前的方式索引的是字节.
///
/// 6. String类型本身类似于cpp的vector类型, 栈上数据只有 [指向堆的指针, 长度, 容量], 堆上存储实际的字符串内容。
///     当容量不足以容纳新的内容时，会重新分配堆内存。

// 填空并修复错误
// 1. 不要使用 `to_string()`
// 2. 不要添加/删除任何代码行
fn example1() {
    //原题
    // let mut s:String = "hello, ";
    // s.push_str("world".to_string());
    // s.push(__);
    // move_ownership(s);
    // assert_eq!(s, "hello, world!");
    // println!("Success!")

    //方法1
    // 没有领悟函数意义
    // let mut s:String = "hello, ".into();
    // s.push_str("world");
    // s.push('!');
    // borrow_string(&s);
    // assert_eq!(s, "hello, world!");
    // println!("Success!");

    //方法2
    let mut s1: String = String::from("hello, ");
    s1.push_str("world");
    s1.push('!');
    let spxs = s1.split("o");
    for s in spxs {
        println!("split: {}", s);
    }
    move_ownership(s1.clone());
    assert_eq!(s1, "hello, world!");
    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}

fn borrow_string(s: &String) {
    println!("borrowed string: \"{}\"", s);
}

//example2

// 填空
fn example2() {
    //    let mut s = String::from("hello, world");
    //    let slice1: &str = __; // 使用两种方法
    //    assert_eq!(slice1, "hello, world");
    //    let slice2 = __;
    //    assert_eq!(slice2, "hello");
    //    let slice3: __ = __;
    //    slice3.push('!');
    //    assert_eq!(slice3, "hello, world!");
    //    println!("Success!")
    let mut s = String::from("hello, world");
    let slice1: &str = &s; //方法1
    let slice1_method2: &str = s.as_str(); //方法2
    assert_eq!(slice1, "hello, world");
    let _slice2 = s.get(0..5).unwrap();
    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");
    let slice3: &mut String = &mut s;
    slice3.push('!');
    let mut _s1 = Vec::new();

    _s1.push(String::from("达瓦达瓦"));
    _s1.push(String::from("11112312"));

    let mut i1 = 13;
    // let mut refi1 = &mut i1;

    i1 = i1 + 1;

    let __vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

    ///告诉我这个变量是有用的，不要优化掉
    std::hint::black_box(i1);

    assert_eq!(slice3, "hello, world!");
    println!("i1: {}", i1);
    println!("Success!");
}

/// 实现一个vec分割切片的函数
/// 请解释这个函数的作用和实现原理:
/// 为什么不需要unsafe代码就可以实现这个功能?
fn split_vec<T>(vec: &mut Vec<T>, index: usize) -> (&mut [T], &mut [T]) {
    let len = vec.len();
    assert!(index <= len, "Index out of bounds");
    let (left, right) = vec.split_at_mut(index);
    (left, right)
}

// 问题:  我们的代码中发生了多少次堆内存分配？
// 你的回答: 2次 String::from 和 slice.to_string()
fn example3() {
    // 基于 `&str` 类型创建一个 String,
    // 字符串字面量的类型是 `&str`
    let s: String = String::from("hello, world!");

    // 创建一个切片引用指向 String `s`
    let slice: &str = &s;

    // 基于刚创建的切片来创建一个 String
    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn example4() {
    let s = String::from("我是中国人");
    // let c3 = s[3]; //不允许使用字符串索引
}

/// 我们无法通过索引的方式访问字符串中的某个字符，
/// 但是可以通过切片的方式来获取字符串的某一部分 &s1[start..end]
fn example5() {
    let s = String::from("hello, 世界");
    // let slice1 = s[0]; //提示: `h` 在 UTF-8 编码中只占用 1 个字节
    let slice1 = &s[0..1];
    assert_eq!(slice1, "h");

    let slice2 = &s[3..5]; // 提示: `世` 在 UTF-8 编码中占用 3 个字节
    assert_eq!(slice2, "世");

    // 迭代 s 中的所有字符
    // for (i, c) in s.__ {
    //     if i == 7 {
    //         assert_eq!(c, '世')
    //     }
    // }
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世');
        }
    }

    println!("Success!")
}


/// 从字节数组创建一个字符串，并与另一个字符串进行比较
// fn example7() {
//     let mut s = String::new();
//     __;

//     let v = vec![104, 101, 108, 108, 111];

//     // 将字节数组转换成 String
//     let s1 = __;
    
    
//     assert_eq!(s, s1);

//     println!("Success!")
// }

fn example7_sln()
{
    let mut s = String::new();
    s.push_str("hello");
    
    let v = vec![104, 101, 108, 108, 111];
    let s1 = String::from_utf8(v).expect("failed convert");

    println!("s1: {}", s1);
    assert_eq!(s, s1);
    println!("Success!");
}


// 修改下面的代码以打印如下内容: 
// 25
// 25
// 25
// 循环中不会发生任何内存分配
fn example8() {
    // let mut s = String::new(); //原题
    let mut s = String::with_capacity(25);
    //直接以指定容量创建字符串，避免循环中发生内存分配

    println!("{}", s.capacity());

    for _ in 0..2 {  //忽略迭代变量
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}

// 填空
use std::mem;

/// 使用 `ManuallyDrop` 来手动管理字符串的内存，防止自动释放
/// 并用来重建原始的 String
fn example9() {
    let story = String::from("Rust By Practice");

    // 阻止 String 的数据被自动 drop
    let mut story = mem::ManuallyDrop::new(story);

    //原题
    // let ptr = story.__();
    // let len = story.__();
    // let capacity = story.__();
    let ptr = story.as_ptr() as *mut u8;
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    // 我们可以基于 ptr 指针、长度和容量来重新构建 String. 
    // 这种操作必须标记为 unsafe，因为我们需要自己来确保这里的操作是安全的
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    println!("新值{},原始值{}", s, *story);
    assert_eq!(*story, s);

    println!("Success!")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        example1();
    }

    #[test]
    fn test_example2() {
        example2();
    }

    #[test]
    fn test_example5() {
        example5();
    }

    ///为vec分割切片函数实现一个    
    #[test]
    fn test_split_vec() {
        let mut vec = vec![1, 2, 3, 4, 5];
        let (left, right) = split_vec(&mut vec, 3);
        assert_eq!(left, &[1, 2, 3]);
        assert_eq!(right, &[4, 5]);
    }

    /// 使用 utf8_slice 对 UTF-8 字符串进行字符索引切片
    #[test]
    fn example6() {
        let s = "The 🚀 goes to the 🌑!";
        let rocket = utf8_slice::slice(s, 4, 5);
        println!("rocket: {}", rocket);
        // Will equal "🚀"
    }

    #[test]
    fn test_example7_sln() {
        example7_sln();
    }

    #[test]
    fn test_example8() {
        example8();
    }

    #[test]
    fn test_example9() {
        example9();
    }
}
