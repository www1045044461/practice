/// 模式的进阶实现

/// 🌟🌟 使用 | 可以匹配多个值, 而使用 ..= 可以匹配一个闭区间的数值序列
fn question1()
{
    for i in 0..10{
        match_number(i);
    }
}

fn match_number(n: i32) {
    match n {
        // 匹配一个单独的值
        1 => println!("One!"),
        // 使用 `|` 填空，不要使用 `..` 或 `..=`
        // __ => println!("match 2 -> 5"), // 原题
        2..=5 => println!("match 2 -> 5"), // 匹配一个闭区间的数值序列
        // 匹配一个闭区间的数值序列
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match 11 -> +infinite")
        }
    }
}


struct Point {
    x: i32,
    y: i32,
}

/// 🌟🌟🌟 @ 操作符可以让我们将一个与模式相匹配的值绑定到新的变量上
fn question2() {
    // 填空，让 p 匹配第二个分支
    // let p = Point { x: __, y: __ }; // 原题
    let p = Point { x: 3, y: 20 };

    // 如果y的值是10、20或30，并且x的值在0到5之间，则匹配第二个分支
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // 第二个分支
        // Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x:0..=5, y:c@(10|20|30) } => println!("On the y axis at {}", c),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}


// 修复错误
enum Message {
    Hello { id: i32 },
}

fn question3() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id:id@( 3..=7),
            // id:3..=7, //原题   
        } => println!("id 值的范围在 [3, 7] 之间: {}", id),
        // Message::Hello { id: newid@10 | 11 | 12 } => {
        Message::Hello { id: newid@(10 | 11 | 12)} => {
            println!("id 值的范围在 [10, 12] 之间: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}


/// 🌟🌟 匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件，它能为分支模式提供更进一步的匹配条件
// 填空让代码工作，必须使用 `split`
fn question4() {
    let num = Some(4);
    let split = 5;
    match num {
        // Some(x) __ => assert!(x < split), // 原题
        Some(x) if x<split => {assert!(x < split); println!("x < split, x: {}, split: {}", x, split);},
        Some(x) => {assert!(x >= split); println!("x >= split, x: {}, split: {}", x, split);},
        None => {println!("None");},
    }
}


// 填空，让代码工作
/// 🌟🌟🌟 使用 .. 忽略一部分值匹配值
fn question5() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        // __ => { //原题
           (first, .., last) => 
           {
                assert_eq!(first, 2);
                assert_eq!(last, 2048);
           }
    }
}


// 修复错误，尽量少地修改代码
// 不要移除任何代码行
/// 🌟🌟 使用模式 &mut V 去匹配一个可变引用时，你需要格外小心，因为匹配出来的 V 是一个值，而不是可变引用
fn question6() {
    let mut v = String::from("hello,");
    let r = &mut v;
    match r {
    //    &mut value => value.push_str(" world!")  //原题
        value => value.push_str(" world!"),  
    }

    println!("结束时的值是:{}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_number() {
        // question1();
        // question2();
        // question3();
        // question4();
        // question5();
        question6();
    }
}
