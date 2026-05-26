/// 问题1: 修改代码让它工作->函数形参未声明
// fn sum(x,y:i32)
//{
//    x+y;
//}
fn sum(x:i32,y:i32)->i32
{
    x+y
}

fn question1()
{
    //不要修改下面的2行代码
    let (x,y) = (1,2);
    let s = sum(x,y);
    assert_eq!(s,3);
}


fn question2() {
   print();
}

// 原题:使用另一个类型来替代 i32
// fn print() -> i32 {
//    println!("hello,world");
// }
fn print()->() //注意不是void
{
    println!("hello,world");
}

// 用两种方法求解
fn question3() {
    never_return();
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    panic!("This function never returns!");
}

/// 4.发散函数使用
fn question4() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // 这里与其返回一个 None，不如使用发散函数替代
    never_return_fn()
}

// 使用三种方法实现以下发散函数
fn never_return_fn() -> ! {
    // 1. 死循环返回结果为发散函数
    // loop {
    // }

    // 2. 使用 panic! 宏来触发一个错误，导致程序崩溃，从而实现发散函数的效果
    // panic!("This function never returns!");

    // 3. 使用 todo! 宏来标记未实现的功能，同时表明该函数不会返回
    // todo!("This function is not implemented yet, but it will never return!");

    //4. 使用 unimplemented! 宏来标记未实现的功能，同时表明该函数不会返回
    unimplemented!("This function is not implemented yet, but it will never return!");
}

// 5. 发散函数同时能取代match表达式的任何
fn question5() {
    // 填空
    // let b = __; //原题
    let b = false;

    let _v = match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
            //注意:这里不能直接使用 never_return_fn()，因为它会导致整个函数发散，而我们只想在 `false` 分支中发散。
        }
    };

    println!("Exercise Failed if printing out this line!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question1() {
        question1();
        question2();
        // question3();
        question4();
        // get_option(13);
        question5();
    }
}