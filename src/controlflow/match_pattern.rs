use crate::controlflow::match_pattern::Message::Quit;

/// 匹配模式

// 填空
enum Direction {
    East,
    West,
    North,
    South,
}

/// 1. 🌟🌟 match 可以匹配一个值的多个模式
fn question1() {
    let dire = Direction::South;
    // match dire {
    //     Direction::East => println!("East"),
    //     __  => { // 在这里匹配 South 或 North
    //         println!("South or North");
    //     },
    //     _ => println!(__),
    // };
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => { 
            // 在这里匹配 South 或 North
            println!("South or North");
        }
        Direction::West => println!("West"),
    }
}


/// 🌟🌟 match 是一个表达式，因此可以用在赋值语句中
fn question2() {
    let boolean = true;

    // 使用 match 表达式填空，并满足以下条件
    //
    // boolean = true => binary = 1
    // boolean = false => binary = 0
    // let binary = __; // 原题
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);
}


// 填空
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn question3() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    match msg {
        // __ => { // 这里匹配 Message::Move //原题
        Message::Move { x:a, y:b }=>{
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        // Message::ChangeColor(_, g, b) => {
        //     assert_eq!(g, __);
        //     assert_eq!(b, __);
        // } 
        //原题
        Message::ChangeColor(r,g ,b )=>{
            assert_eq!(g,255);
            assert_eq!(b,0);
        },
        Message::Write(s) => println!("Message::Write with {}", s),
        Message::Quit => println!("no data in these variants")
    }
}


/// matches! 看起来像 match, 但是它可以做一些特别的事情
fn question4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // 使用 `matches!` 填空
    for ab in alphabets {
        // assert!(__)// 原题
        let t1 = matches!(ab, 'a'..='z' | 'A'..='Z'| '0'..='9');
        println!("{} is alphabet or number? {}", ab, t1);
        // assert!(matches!(ab, 'a'..='z' | 'A'..='Z'| '0'..='9')); // 这里匹配字母
    }
} 


enum MyEnum {
    Foo,
    Bar
}

fn question5() {
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        // if e == MyEnum::Foo { // 修复错误，只能修改本行代码
        if let MyEnum::Foo = e  //方法2
        // if matches!(e, MyEnum::Foo) //方法1
        {
            count += 1;
        }
    }

    println!("count: {}", count);
    assert_eq!(count, 2);
}


/// 在有些时候, 使用 match 匹配枚举有些太重了，此时 if let 就非常适合.
fn question6() {
    let o = Some(7);

    // 移除整个 `match` 语句块，使用 `if let` 替代
    // match o {
    //     Some(i) => {
    //         println!("This is a really long string and `{:?}`", i);
    //     }
    //     _ => {}
    // };

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
    }
}

// 填空
enum Foo {
    Bar(u8)
}

fn question7() {
    let a = Foo::Bar(1);

    // __ //原题
    if let Foo::Bar(i) = a {
        println!("foobar 持有的值是: {}", i);
    }
}


enum Foo2 {
    Bar,
    Baz,
    Qux(u32)
}

/// 8. 🌟🌟🌟 使用 match 匹配枚举的不同变体
fn question8() {
    let a = Foo2::Qux(10);

    // 移除以下代码，使用 `match` 代替
    // if let Foo::Bar = a {
    //     println!("match foo::bar")
    // } else if let Foo::Baz = a {
    //     println!("match foo::baz")
    // } else {
    //     println!("match others")
    // }
    match a {
        Foo2::Bar => println!("match foo::bar"),
        Foo2::Baz => println!("match foo::baz"),
        Foo2::Qux(_) => println!("match others")
    }
}


/// 变量遮蔽( Shadowing )
fn question9() {
    let age = Some(30);
    if let Some(age) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
    //    assert_eq!(age, Some(30)); //原题
        assert_eq!(age,30);
    } // 新的 `age` 变量在这里超出作用域
    
    match age {
        // `match` 也能实现变量遮蔽
        Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
        _ => ()
    }
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
        // question8();
        question9();
    }
}
