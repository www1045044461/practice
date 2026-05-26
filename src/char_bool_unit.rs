/// 字符、bool和元组类型
/// 1. rust的char类型是4字节的Unicode标量值，而不是单字节的ASCII字符 
/// 
use crate::function_name_macro::function_name;
use std::mem::size_of_val; 
fn char_pracitice()
{
    let c1 = 'a';
    let c2 = '中';

    assert_eq!(size_of_val(&c1), 1);
    assert_eq!(size_of_val(&c2), 3);
    println!("Success!");
}

fn solution_char_practice()
{
    let c1 = 'a';
    let c2 = '中';

    assert_eq!(size_of_val(&c1), 4);
    assert_eq!(size_of_val(&c2), 4);
    println!("Success!");
}

fn print_char(c:char)
{
    println!("The char is: {}", c);
}

fn question2()
{
    // let c = "天"; //原题
    let c = '天'; //原题
    print_char(c);
}

fn bool_question()
{
    let _f = false;
    let t = true;

    //原题
    // if !t {
    //     println!("Success!");
    // }
    if !_f{
        println!("Success!");
    }
}

fn bool_question2()
{
    let f = true;
    // let t = true && false; //原题
    let t = true || false; 
    dbg!(t);
    dbg!(f);
    assert_eq!(t,f);
    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}

fn unit_question() {
    let _v: () = ();
    let v = (2, 3);
    // assert_eq!(_v, implicitly_ret_unit()); //原题
    assert_eq!(_v, implicitly_ret_unit());
    println!("unit Success!");
}

/// 6.元组类型占用内存的大小
fn questtion6()
{
    // let unit = (); //原题
    let unit = (2i16,3i16);
    assert!(size_of_val(&unit) == 4);
    println!("Success6!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_practice() {
        // char_pracitice();
        solution_char_practice();
        question2();
        bool_question();
        bool_question2();
        unit_question();
        questtion6();
    }
}
