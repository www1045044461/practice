/// enums的测试过程

// 修复错误
/// 🌟🌟 在创建枚举时，你可以使用显式的整数设定枚举成员的值。
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C语言风格的枚举定义
// 问题:enum非primitive类型
enum Number2 {
    // Zero = 0.0,
    // One = 1.0,
    // Two = 2.0,
    Zero = 0,
    One = 1,
    Two = 2,
}


fn question1() {
    // 通过 `as` 可以将枚举值强转为整数类型
    // assert_eq!(Number::One, Number1::One);
    // assert_eq!(Number1::One, Number2::One); // 原题
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
} 

// 填空
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Quit => write!(f, "Quit"),
            Message::Move { x, y } => write!(f, "Move to ({}, {})", x, y),
            Message::Write(text) => write!(f, "Write: {}", text),
            Message::ChangeColor(r, g, b) => write!(f, "Change color to ({}, {}, {})", r, g, b),
        }
    }
}

/// 🌟 枚举成员可以持有各种类型的值
fn question2() {
    // 原题
    // let msg1 = Message::Move{__}; // 使用x = 1, y = 2 来初始化
    // let msg2 = Message::Write(__); // 使用 "hello, world!" 来初始化
    let msg1 = Message::Move { x: 1, y: 2 }; // 使用x = 1, y = 2 来初始化
    let msg2 = Message::Write(String::from("hello, world!")); // 使用 "hello, world!" 来初始化

    dbg!(msg1);
    dbg!(msg2);
    //总结:enum初始化类似于struct初始化，区别在于enum的每个变体都可以有不同的字段和类型。
    //同样需要实现debug trait才能使用dbg!宏打印枚举实例。
} 


// 仅填空并修复错误
fn question3() {
    let msg = Message::Move{x: 1, y: 2};
    // if let Message::Move{__} = msg { //原题
    // if let Message::Move { x:a, y:b } = msg { //方法1
    if let Message::Move { x, y } = msg { //方法1
        // println!("x: {}, y: {}", x, y); //cannot find value `x` in this scope
        // assert_eq!(a, b); //使用a和b来比较x和y的值

        println!("x: {}, y: {}", x, y); //使用x和y来比较x和y的值
    } else {
        panic!("不要让这行代码运行！");
    }

    let msg1= Message::Move { x: 13, y: 13 };
    if let Message::Move { x:_, y } = msg1 {
        // assert_eq!(x, y); //cannot find value `x` in this scope
        assert_eq!(y, 13); //使用y来比较y的值
    } else {
        panic!("不要让这行代码运行！");
    }
} 


// 填空，并修复错误
/// 🌟🌟 使用枚举对类型进行接口同一化(只要实现一套Display接口就能传入多种枚举)
fn question4() {
    // let msgs: __ = [
    //     Message::Quit,
    //     Message::Move{x:1, y:3},
    //     Message::ChangeColor(255,255,0)
    // ];
    // 原题
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    println!("{}", msg);
}


// 填空让 `println` 输出，同时添加一些代码不要让最后一行的 `panic` 执行到
/// 🌟🌟 Rust 中没有 null，我们通过 Option<T> 枚举来处理值为空的情况
fn question5() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // 原题
    // if let __ = six {
    //     println!("{}", n)
    // } 
    if let Some(n) = six {
        println!("{}", n);
        return;
    }
        
    panic!("不要让这行代码运行！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    // 原题
    // match x {
    //     __ => None,
    //     __ => Some(i + 1),
    // }
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

//自实现列表
enum List<T> {
    Cons(T, Box<List<T>>), //包含一个元素和一个指向下一个节点的指针
    Nil, //尾节点
}

impl<T> List<T> {
    fn new() -> Self {
        List::Nil
    }

    fn prepend(self,elem:T)-> Self {
        List::Cons(elem, Box::new(self))
        //完成prepend方法，使得它能够在列表前面添加一个元素并返回新的列表
    }

    fn len(&self)->usize
    {
        match self {
            List::Cons(_, tail) => 1 + tail.len(), //递归调用了
            List::Nil => 0,
        }
    }
}

/// 专门为了实现Display trait而写的代码
impl <T:std::fmt::Display> List<T> {
    fn stringify(&self) -> String {
        match self {
            List::Cons(head, tail) => format!("{}, {}", head, tail.stringify()),
            List::Nil => String::from("Nil"),
        }
    }
}

fn question6() {
    // 创建一个新的链表(也是空的)
    let mut list = List::new();

    // 添加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 打印列表的当前状态
    println!("链表的长度是: {}", list.len());
    println!("{}", list.stringify());

    let mut list1 = List::new();
    list1 = list1.prepend(FileNode1{name:String::from("file1.txt"), size:100});
    list1 = list1.prepend(FileNode1{name:String::from("file2.txt"), size:200});
    println!("链表的长度是: {}", list1.len());
    //println!("{}", list1.stringify());
    //Error:the method `stringify` exists for enum `List<FileNode1>`, but its trait bounds were not satisfied
    //method cannot be called on `List<FileNode1>` due to unsatisfied trait bounds

    let mut list2 = List::new();
    list2 = list2.prepend(FileNode2{
        name:String::from("file1.txt"),
        size:300,
    });
    list2 = list2.prepend(FileNode2 { name: String::from("file2.txt"), size: 400 });
    list2 = list2.prepend(FileNode2 { name: String::from("file3.txt"), size: 410 });
    println!("链表的长度是: {}", &list2.len());
    println!("{}", list2.stringify());
}

struct FileNode1 {
    name:String,
    size:u32,
}

struct FileNode2 {
    name:String,
    size:u32,
}

impl std::fmt::Display for FileNode2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileNode2 {{ name: {}, size: {} }}", self.name, self.size)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // question1();
        // question2();
        // question3();
        // question4();
        // question5();
        question6();
    }
}

