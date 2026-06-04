/// 字符串和&str

// 修复错误，不要新增代码行
// 🌟 正常情况下我们无法使用 str 类型，但是可以使用 &str 来替代
fn question1() {
    // let s: str = "hello, world"; // 原题
    let s = "hello, world"; 
    println!("The string is: {}", s);
}

// 2.要持有str类型只有使用Box<str>，但是我们更常用String来持有字符串

// 使用至少两种方法来修复错误
fn question2() {

    //相当于1层指针,对实参解引用无效
    let s: Box<str> = "hello, world".into();
    println!("实参地址:{:p}",s); //Box实现了std::fmt::Pointer
    // println!("实参地址:{:p}",*s); //str没有实现Pointer所以需要解引用
    // greetings(s); //原题
    greetings(s.as_ref()); //方法1
    greetings(&s); //方法1

    println!("----------------------------");

    let s:Box<&str> = Box::new("hello, world");
    println!("实参地址:{:p}", s);
    println!("实参地址:{:p}", *s);
    greetings(*s); //相当于2层指针;
}

fn borrow_method1(mut value: Box<i32>)
{
    let reference: &mut i32 = value.as_mut();
    *reference = *reference + 1;
    println!("The value is: {} address:{:p}", reference,reference);
}

fn borrow_test()
{
    let value = Box::new(42);
    println!("Original value: {} address:{:p}", value, value);
    borrow_method1(value);
}

fn greetings(s: &str) {
    println!("{} 地址:{:p}",s, s)
}

fn param_method1() {
    let mut i = 13;
    let mut y = 14;
    let mut r1 = &i; //绑定r1可变但是指向的是不可变引用
    
    println!("r1: {}, address: {:p}", r1, r1);
    r1 = &y; //换指向
    println!("r1: {}, address: {:p}", r1, r1);

    // r1 = 13;
}

fn param_method2() {
    let mut i = 13;
    let mut y = 14;
    let r1 = &mut i; //绑定r1不可变但是指向的是可变引用
    
    println!("r1: {}, address: {:p}", r1, r1);
    // r1 = &mut y; //错误:不能对不可变绑定赋值两次
    *r1 = 25;
    println!("r1: {}, address: {:p}", r1, r1);
}

fn param_method3()
{
    let mut i = 13;
    let mut y = 14;
    let mut r1 = &mut i; //绑定r1不可变但是指向的是可变引用
    
    println!("初始值 r1: {}, address: {:p}", r1, r1);
    r1 = &mut y; //错误:不能对不可变绑定赋值两次
    println!("换指向 r1: {}, address: {:p}", r1, r1);
    *r1 =  25;
    println!("修改内容 r1: {}, address: {:p}", r1, r1);
}

//所有需要mut的型参例子
fn string_mut_method(mut s:String)
{
    s.push('|'); // push 需要 &mut self
}

fn vec_mut_method(mut v:Vec<i32>)
{
    v.push(42);
}

fn box_mut_method(mut b:Box<i32>)
{
    *b += 1; // 需要可变访问内部值
}

fn hashmap_mut_method(mut m:std::collections::HashMap<String, i32>)
{
    m.insert("key".to_string(), 42);
}

//错误的写法:expected type, found keyword `mut` expected type
// fn string_mut_method2(s:mut String)
// {
//     s.push('|');
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question1() {
        // question1();
        // question2();
        // borrow_test();
        // param_method1();
        // param_method2();
        param_method3();
    }
}
