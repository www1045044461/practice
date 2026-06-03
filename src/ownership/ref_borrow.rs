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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question1() {
        // question1();
        // question2();
        // question3();
        // question4();
        // self_method_deref();
        slices_deref();
    }
}
