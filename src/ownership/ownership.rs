use std::string;

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
    let mut s1 = s; // 将s的所有权转移给s1，并且s1是可变的
    s1.push_str("world")
}

fn question7() {
    let x = Box::new(5);
    
    // let ...      // 完成该行代码，不要修改其它行！
    let mut y = Box::new(10);
    
    *y = 4;
    
    assert_eq!(*x, 5);
}

/**********************************************************************/
fn example_test() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age 
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    // println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}

pub fn question8() {
    let t = (String::from("hello"), String::from("world"));
    // let _s = t.0; //原题
    let _s = &t.0;
    // 仅修改下面这行代码，且不要使用 `_s`
    println!("{:?}", t);

    //解答思路: 原题中，t.0 的所有权被转移给了 _s，因此 t 这个元组就无法再使用了。
    //通过将 _s 定义为对 t.0 的引用，我们避免了所有权的转移，这样 t 仍然可以被使用，从而成功编译并输出整个元组。
}

pub fn question9(){
   let t = (String::from("hello"), String::from("world"));

   // 填空，不要修改其它代码
   //let (__, __) = __;
   let (s1, s2) = (&t.0, &t.1);
   
   //思路: 通过将 s1 和 s2 定义为对 t.0 和 t.1 的引用，我们避免了所有权的转移，这样 t 仍然可以被使用，从而成功编译并输出整个元组。
   println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
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
        // question5();
        // question6();
        // question7();
        // example_test();
        // question8();
        question9();
    }
}