use core::hash;

/// 条件控制流示例

// if-else 不返回表达式不可赋值
fn question1() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

#[derive(Debug)]
enum NumberType {
    Odd,
    Even,
}

/// 返回值为表达式类型 {} 中最后一个表达式的值和返回值一样
fn number_dispatch(value: u8) -> NumberType {
    if value % 2 == 0 {
        NumberType::Even
    } else {
        NumberType::Odd
    }
}

fn question2() {
    // for i in 0..=10 { //含10迭代
    for i in 0..10 {
        //不含10迭代
        println!("{} is {:?}", i, number_dispatch(i));
    }
}

// 修复错误，不要新增或删除代码行
fn question3() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names { //这里添加&
        // do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // numbers中的元素实现了 Copy，因此无需转移所有权
    for n in numbers {
        // do something with name...
    }

    println!("{:?}", numbers);
}

/// 同时访问索引和数值
fn question4() {
    let a = [4, 3, 2, 1];

    for item in a.iter() {
        println!("元素是: {}", item);
    }

    // 通过索引和值的方式迭代数组 `a`
    // for (i,v) in a.__ { //原题
    for (index, value) in a.iter().enumerate() {
        //方法1
        println!("第{}个元素是{}", index + 1, value);
    }
}

// 填空，让最后一行的  println! 工作 !
/// 🌟🌟 当条件为 true 时，while 将一直循环
fn question5() {
    // 一个计数值
    let mut n = 1;

    // 当条件为真时，不停的循环
    // while n __ 10 {
    while n <= 10 {
        //原题
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        // __; //原题
        n += 1;
    }

    println!("n 的值是 {}, 循环结束", n);
}

// 填空，不要修改其它代码
/// 🌟 使用 break 可以跳出循环
fn question6() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            //    __ //原题
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);
}

/// 🌟🌟 continue 会结束当次循环并立即开始下一次循环
fn question7() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            //    __;
            continue;
        }
        //    __
        break;
    }
    assert_eq!(n, 66);
}


/// 🌟🌟 loop 一般都需要配合 break 或 continue 一起使用
fn question8() {
    let mut count = 0u32;
    println!("Let's count until infinity!");
    // 无限循环
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // 跳过当前循环的剩余代码
            // __; //原题
            continue;
        }

        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            // __; //原题
            break;
        }
    }

    assert_eq!(count, 5);
}


/// 🌟🌟 loop 是一个表达式，因此我们可以配合 break 来返回一个值
fn question9() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // __; //原题
            break counter * 2;
        }
    };

    println!("The result is {}", &result);
    assert_eq!(result, 20);
}

/// while不能返回值，所以下面的代码是错误的
// fn while_example(){
//     let mut n = 1;
//     let result = while n <= 10 {
//         println!("n is {}", n);
//         n += 1;
//         if n == 5
//         {
//             break 10;
//         }
//     }
// }

/// 🌟🌟🌟 当有多层循环时，你可以使用 continue 或 break 来控制外层的循环。
/// 要实现这一点，外部的循环必须拥有一个标签 'label, 然后在 break 或 continue 时指定该标签
fn question10() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // 这只会跳出 inner1 循环
                break 'inner1; // 这里使用 `break` 也是一样的
            }
            count += 2;
        }
        count += 5;
        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }
            continue 'outer;
        }
    }
    // assert!(count == __)
    assert!(count == 30)
}

pub trait Animal {
    fn sound(&self) -> String;
}

pub struct Dog;

impl Animal for Dog {
    fn sound(&self) -> String {
        "Woof!".to_string()
    }
}

/// 🌟 逆变：当一个函数参数类型是某个trait的引用时，传入该trait的任何实现类型的引用都是合法的。
pub fn caller_contravariance(u: &impl Animal) {
    println!(
        "caller_contravariance method ====>{:p} sound:{}",
        u,
        u.sound()
    );
}

fn low_level_contravariance<'a>(msg: &'a str) {
    println!("low_level_contravariance method ====>{:p}", msg);
}

/// 逆变:能接受任意生命周期&str的函数可以放到需要特定生命周期&'static str的函数参数位置
/// 因为&'static str是所有生命周期&str的子类型，所以传入&'static str的函数参数位置也是合法的。
fn high_level_contravariance_caller(f: impl Fn(&'static str)) {
    let msg: &'static str = "Hello, world!";
    println!("high_level_contravariance_caller method ====>{:p}", msg);
    f(msg);
    println!("high_level_contravariance_caller method <====");
}

struct Cat;
impl Animal for Cat {
    fn sound(&self) -> String {
        "Meow!".to_string()
    }
}

fn container_caller(vec: Vec<&dyn Animal>) {
    for item in vec {
        println!("Animal sound: {}", item.sound());
    }
}

/// 容器类型的逆变:当一个容器类型（如Vec<T>）的元素类型T是某个trait的引用时，
/// 传入该trait的任何实现类型的引用都是合法的。
fn contravariance_example3() {
    let mut vec: Vec<&dyn Animal> = Vec::new();
    vec.push(&Cat {});
    vec.push(&Dog {});
    // 这里我们创建了一个Vec<&dyn Animal>，
    //它可以存储任何实现了Animal trait的类型的引用。
    //我们向其中添加了Cat和Dog的引用，这都是合法的，因为它们都实现了Animal trait。

    container_caller(vec);
}

/// 逆变示例：函数参数类型的逆变
fn handler_any<'a>(msg: &'a str) {
    println!("handler_any method ====>{:p}", msg);
}
/// 逆变示例：函数参数类型的逆变
fn run_all(handlers: Vec<fn(&'static str)>) {
    let msg: &'static str = "Hello, world!";
    for h in handlers {
        h(msg);
    }
}

/// 逆变示例：函数参数类型的逆变
/// ```markdown
/// 1. 子类型关系：'static 更长，所以 &'static str 是 &'a str 的子类型（可缩短使用）。
/// 2. 函数参数位是逆变：如果 A <: B，那么 fn(B) <: fn(A)。
/// 3. 所以“能接更泛参数”的函数，可以放到“只需接更具体参数”的位置。
/// ```
fn contravariance_example3_real() {
    let mut handlers: Vec<fn(&'static str)> = Vec::new();

    //关键点: handler_any函数的参数类型是&'a str，它可以接受任何生命周期的字符串切片。
    handlers.push(handler_any);
    run_all(handlers);
}

/// 不变演示:这个函数的作用是强行拉平两个生命周期，使它们必须完全相同。
fn need_short<'a>(x: &mut &'a str, y: &'a str) {
    *x = y;
}

fn compare<'a>(x: &'a str, y: &'a str) -> bool {
    x == y
}

/// 1. &'static str 是 &'a str 的子类型关系来源;
/// 2. 但到了 &mut T 就不允许了，因为它是不可变的，所以 Rust 不允许 &mut T 是 T 的子类型关系。
/// 3. 这就是 Rust 中的逆变示例，&mut T 不允许是 T 的子类型关系，以防止悬垂引用的发生。
/// 4. Cell<T> 这类"可写容器"通常也是不变;
fn invariance_example1() {
    let mut s: &'static str = "hello";
    {
        let short = String::from("world");
        let short_ref: &str = &short;
        //`short` does not live long enough
        // borrowed value does not live long enough

        // 如果 &mut T 对 T 是协变，这里就会被允许
        // 但那样会把一个短生命周期引用写进 s
        // 离开作用域后 s 就悬垂了
        // 所以 Rust 这里不允许
        //
        // need_short(&mut s, short_ref);
        compare(s, short_ref);
    }
    println!("{}", s);
}

use std::cell::Cell;

/// cell 也是不变的，因为它是可写的，如果它是协变的，就会有同样的问题。
fn invariance_example2() {
    let cell: Cell<&'static str> = Cell::new("hello");

    // 假设 Cell 是协变，
    // 那么 Cell<&'static str> 就能当作 Cell<&short str> 用，
    // 然后可以把短生命周期引用 set 进去，造成问题。
    //
    // 所以这类可写容器通常是不变的。
    {
        let short = String::from("world");
        let short_ref: &str = &short;
        //error: `short` does not live long enough borrowed value does not live long enough
        // cell.set(short_ref);
    }
    println!("{}", cell.get());
}

use std::cell::RefCell;

fn invariance_example3() {
    let rc: RefCell<&'static str> = RefCell::new("hello");

    {
        let short = String::from("world");
        let short_ref: &str = &short;
        //`short` does not live long enough borrowed value does not live long enough

        // 如果 RefCell<T> 对 T 是协变，
        // 就可能把 short_ref 写进原本装 &'static str 的容器
        //
        // *rc.borrow_mut() = short_ref;
    }

    println!("{}", rc.borrow());
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
        // high_level_contravariance_caller(low_level_contravariance);
        // println!("====================");
        // let dog = Dog;
        // caller_contravariance(&dog);
        // contravariance_example3();
        // contravariance_example3_real();
        // invariance_example1();
        // invariance_example2();
        // question5();
        // question6();
        // question7();
        // question8();
        // question9();
        question10();
    }
}
