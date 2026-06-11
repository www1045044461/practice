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
fn number_dispatch(value:u8)-> NumberType
{
    if value % 2 == 0 {
        NumberType::Even
    } else {
        NumberType::Odd
    }
}

fn question2(){
    // for i in 0..=10 { //含10迭代
    for i in 0..10 { //不含10迭代
        println!("{} is {:?}", i, number_dispatch(i));
    }
}


// 修复错误，不要新增或删除代码行
fn question3() {
    let names = [String::from("liming"),String::from("hanmeimei")];
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
    let a = [4,3,2,1];

    for item in a.iter() {
        println!("元素是: {}", item);
    }

    // 通过索引和值的方式迭代数组 `a` 
    // for (i,v) in a.__ { //原题
    for (index,value) in a.iter().enumerate() { //方法1
        println!("第{}个元素是{}",index+1,value);
    }
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
    println!("caller_contravariance method ====>{:p} sound:{}",u,u.sound());
}


fn low_level_contravariance<'a>(msg: &'a str) {
    println!("low_level_contravariance method ====>{:p}",msg);
}

/// 逆变:能接受任意生命周期&str的函数可以放到需要特定生命周期&'static str的函数参数位置
/// 因为&'static str是所有生命周期&str的子类型，所以传入&'static str的函数参数位置也是合法的。
fn high_level_contravariance_caller(f: impl Fn(&'static str)) {
    let msg: &'static str = "Hello, world!";
    println!("high_level_contravariance_caller method ====>{:p}",msg);
    f(msg);
    println!("high_level_contravariance_caller method <====");
}

struct Cat;
impl Animal for Cat {
    fn sound(&self) -> String {
        "Meow!".to_string()
    }
}

/// 容器类型的逆变:当一个容器类型（如Vec<T>）的元素类型T是某个trait的引用时，
/// 传入该trait的任何实现类型的引用都是合法的。
fn contravariance_example3() {
    let mut vec:Vec<&dyn Animal> = Vec::new();
    vec.push(&Cat{});
    vec.push(&Dog{});
    // 这里我们创建了一个Vec<&dyn Animal>，
    //它可以存储任何实现了Animal trait的类型的引用。
    //我们向其中添加了Cat和Dog的引用，这都是合法的，因为它们都实现了Animal trait。

    for item in vec {
        println!("Animal sound: {}", item.sound());
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
        high_level_contravariance_caller(low_level_contravariance);

        println!("====================");

        let dog = Dog;
        caller_contravariance(&dog);

        contravariance_example3();
    }
}
