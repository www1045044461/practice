/// 所有权实验

/// 1. 使用尽可能多的方式通过编译
pub fn question1() {
    // 原题
    // let s = String::from("hello, ownership");
    // let x = s;
    // println!("{},{}", s,x);

    //1. 直接clone - 两处内存实体
    let s = String::from("hello, ownership");
    let x = s.clone();
    println!("直接clone: {},{}", s,x);

    //2. 使用引用 - 一处内存实体
    let s = String::from("hello, ownership");
    let x = &s;
    println!("使用引用: {},{}", s,x);

    //3. 使用切片 - 一处内存实体
    let s = String::from("hello, ownership");
    let x = &s[..];
    println!("使用切片: {},{}", s,x);

    //4.只读引用复制 - 一处内存实体
    let s = String::from("hello, ownership");
    let x = s.as_str();
    println!("只读引用复制: {},{}", s,x);

    //5.字面值引用
    let s = "hello, ownership";
    let x = s;
    println!("字面值引用: {},{}", s,x);

}

/*--------------------------------------------------------------------*/

/// 2.
// 不要修改 main 中的代码
fn question2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);
    println!("{}", s2);
}

// // 只能修改下面的代码!
// 原题
// fn take_ownership(s: String) {
//     println!("{}", s);
// }
/// 该函数获取了一个string的所有权
fn take_ownership(s:String)->String
{
    println!("{}", s);
    s
}
/*--------------------------------------------------------------------*/

/// 3. 所有权3

fn question3() {
    let s = give_ownership();
    let s2 = give_ownership2();
    println!("{}", s);
    println!("{}", s2);
}

// 只能修改下面的代码! 原题
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     // 将 String 转换成 Vec 类型
//     let _s = s.into_bytes();
//     s
// }

fn give_ownership() -> String {
    let s = String::from("hello, world");
    s //方法1: 直接返回s，Rust会自动将s的所有权转移给调用者
}

fn give_ownership2() -> String {
    let s = String::from("hello, world");
    let _s = s.as_bytes();
    s 
    //方法2: 使用as_bytes方法获取s的字节表示，但不改变s的所有权，最后返回s
    //return之后_s的作用域结束，s的所有权仍然在函数内，返回s时会将所有权转移给调用者
}

/*--------------------------------------------------------------------*/
/// 4.
// 修复错误，不要删除任何代码行
fn question4() {
    let s = String::from("hello, world");
    // print_str(s);
    print_str(&s); //针对引用语义的修改
    // print_str(s.clone()); //针对move语义的修改
    println!("{}", s);
}

/// 原题
// fn print_str(s: String)  {
//     println!("{}",s)
// }

fn print_str(s: &String)  {
    println!("{}",s)
}

/*--------------------------------------------------------------------*/
/// 5.不要使用 clone，使用 copy 的方式替代
fn question5() {
    // let x = (1, 2, (), "hello".to_string());
    let x = (1, 2, (), "hello"); 
    //解法:使用字面值类型替代String类型，元组中的元素都是Copy类型，因此整个元组也是Copy类型，可以直接复制而不需要克隆。
    let y = x; // 原题
    println!("{:?}, {:?}", x, y);
}

/*--------------------------------------------------------------------*/

/// 6. 当所有权转移时，可变性也可以随之改变 1
fn question6() {
    let s = String::from("hello, ");
    
    // 只修改下面这行代码 !
    // let s1 = s;
    s1.push_str("world")
}

/// 5.不要使用 clone，使用 copy 的方式替代
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question1() {
        // question1();
        // question2();
        // question3();
        // question4();
        question5();
    }
}