/// 引用和借用部分

pub fn question1() {
    let x = 5;
    // 填写空白处
    //let p = __;
    let p = &x;

    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84

    let s1 = String::from("hello");
    let s2_ref = &s1;
    let p1 = &s1;
    let p2 = s2_ref;
    let p3 = &s1;
    let p4 = &s2_ref;
    let mut p5 = &s1;
    println!(
        "s1 的内存地址是 {:p} s2_ref地址是 {:p} 
    p3地址是 {:p} p4地址是 {:p} p5的地址是 {:p}",
        p1, p2, p3, p4, p5
    ); // output:
    println!("s1 的内存地址是 {} s2_ref地址是 {}", p1, p2); // output:

    //    println!("x内容:{:p}",x);
    //    println!("s1内容:{:p}",s1);
}

fn question2() {
    let x = 5;
    let y = &x;
    // 只能修改以下行
    // assert_eq!(5, y);
    // can't compare `{integer}` with `&{integer}`
    // the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
    assert_eq!(5, *y);
}

// 修复错误
fn question3() {
    let mut s = String::from("hello, ");
    // borrow_object(s); // 原题:不能使用,类型不匹配
    borrow_object(&s);
    move_object(s);
}
fn borrow_object(s: &String) {}
fn move_object(s: String) {}

// 修复错误
fn question4() {
    let mut s = String::from("hello, ");
    // push_str(s); // 原题:不能使用,类型不匹配
    push_str(&mut s);
}

fn question5() {
    let mut s = String::from("hello, ");
    //填写空白处,让代码工作
    // let p = __;
    let p = &mut s;
    p.push_str("world");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

fn self_method_deref() {
    let mut s = String::from("hi");
    s.push_str(" rust");
    // 这里写的是 s.push_str(...)
    // 实际会自动借用成 (&mut s).push_str(...)

    // 正确语义
    let mut s1 = &mut s;
    s1.push_str("dwdw");

    let b = Box::new(String::from("hello"));
    let n = b.len();
    // Box<String> 会自动解引用到 String，再调用 len
    println!("{} {}", s, n);
}

fn takes_str(x: &str) {
    println!("{}", x);
}

fn takes_slice(x: &[i32]) {
    println!("{:?}", x);
}

fn slices_deref() {
    let s = String::from("world");
    takes_str(&s);
    // &String 自动强转为 &str

    let v = vec![1, 2, 3];
    takes_slice(&v);
    // &Vec<i32> 自动强转为 &[i32]
}

fn need_ref(x: &String) {
    println!("{}", x);
}

fn need_ref_test() {
    let s = String::from("abc");
    // need_ref(s); // 这一行会报错：期望 &String，实际是 String
    need_ref(&s); // 需要显式写借用
}

/// ref 与 &类似,可以用来获取一个值的引用但是他们的用法各有不同
fn question6() {
    let c = '中';

    let r1 = &c;
    // 填写空白处，但是不要修改其它行的代码
    // let __ r2 = c;
    let ref r2 = c;

    println!("r1: {}, r2: {}", *r1, *r2);
    assert_eq!(*r1, *r2);

    println!("r1 的内存地址是 {:p}", r1);
    println!("r2 的内存地址是 {:p}", r2);
    // 判断两个内存地址的字符串是否相等
    assert_eq!(get_addr(r1), get_addr(r2));
}

// 获取传入引用的内存地址的字符串形式
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

// 移除代码某个部分，让它工作
// 你不能移除整行的代码！
fn question7() {
    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s; //原题
    let r1 = &s;
    let r2 = &s; //生命周期内只允许一个可变引用或者多个不可变引用，但不能同时存在可变和不可变引用

    println!("{}, {}", r1, r2);
}

/// Arc 和 Mutex 的使用示例-->从引用规则延申出来的
use std::sync::{Arc, Mutex};
fn arc_mutex_example() {
    let data = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter_clone = Arc::clone(&data);
        let handle = std::thread::spawn(move || {
            for _ in 0..1000 {
                let mut guard = counter_clone.lock().unwrap();
                *guard += 1;
                // guard 在这里被自动释放，因为它超出了作用域
            }
        });

        handles.push(handle);
    }

    for join_handle in handles {
        join_handle.join().unwrap();
    }

    let final_count = *data.lock().unwrap();
    println!("Final count: {}", final_count);
}

//8.可变性
fn borrow_object_as_mut(s: &mut String) {
    s.push_str("world");
}

fn question8() {
    // 通过修改下面一行代码来修复错误
    // let  s = String::from("hello, ");
    let mut s = String::from("hello");
    borrow_object_as_mut(&mut s)
}

/// 9.可以从可变对象钟借用不可变引用
fn question9() {
    let mut s = String::from("hello, ");
    borrow_object(&s);
    s.push_str("world");
}

//10.NLL
// 注释掉一行代码让它工作
fn question10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
    // println!("{}",r1); //原题
    // 注释掉这里不让r1的生命周期继续到r2的生命周期内，这样就不会有两个可变引用同时存在了
}

//11.

fn question11() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
    // 你不能同时使用 r1 和 r2
    // println!("{}",r1);
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
        // question6();
        // question7();
        // arc_mutex_example();
        // question8();
        // question9();
        // question10();
        question11();
        // self_method_deref();
        // slices_deref();
    }
}
