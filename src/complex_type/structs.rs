use std::fmt::Pointer;

/// 结构体相关信息


// fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}

/// 🌟 对于结构体，我们必须为其中的每一个字段都指定具体的值
fn question1() {
    let age = 30;
    //原题
    // let p = Person {
    //     name: String::from("sunface"),
    //     age,
    // };
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };

    println!("person: {:?}", p);
} 

impl std::fmt::Debug for Person  {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        // 方法1
        // f.debug_struct("Person")
        // .field("name",&self.name)
        // .field("age",&self.age)
        // .field("hobby",&self.hobby)
        // .finish()

        //方法2
        f.write_str(&format!("Person {{ name:{}, age:{}, hobby:{} }}", self.name, self.age, self.hobby))
    }
}


/// 单元结构体没有任何字段
struct Unit;
struct Unit2;

trait SomeTrait {
    // ...定义一些行为
    fn say_hello(&self);
}

// 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
impl SomeTrait for Unit { 
    fn say_hello(&self) {
        println!("Hello from Unit!");
    }
}

impl SomeTrait for Unit2  {
    fn say_hello(&self) {
        println!("Hello from Unit2!");
    }
}

fn question2() {
    let u = Unit;
    let u2 = Unit2;
    
    do_something_with_unit(&u);
    do_something_with_unit(&u2);
    //&dyn Traits用法

    let u1 :Box<dyn SomeTrait>= Box::new(Unit);
    let u3 :Box<dyn SomeTrait>= Box::new(Unit2);
    do_someting_with_unit_box(u1);
    println!();
    do_someting_with_unit_box(u3);
    println!();
    //Box<dyn Traits>用法

    do_something_with_unit_generic(&u);
    println!();
    do_something_with_unit_generic(&u2);
    println!();
    //泛型用法

    do_something_with_unit_t1(u);
} 

// 填空，让代码工作
// fn do_something_with_unit(u: __) {   }
fn do_something_with_unit_t1(u: Unit) {   }
fn do_something_with_unit(u: &dyn SomeTrait) {  
    println!("dyn ref method ====>{:p}",&u);
    u.say_hello();
    println!("dyn ref method <====");
}
fn do_someting_with_unit_box(u: Box<dyn SomeTrait>) {
    println!("ref box method ====>{:p}",u);
    u.say_hello();
    println!("ref box method <====");
}
fn do_something_with_unit_generic<T: SomeTrait>(u: &T) {
    println!("generic method ====>{:p}",&u);
    u.say_hello();
    println!("generic method <====");
}


// 填空并修复错误
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/// 复习:使用From trait 来实现类型转换
impl From<Point> for Color {
    fn from(p: Point) -> Self {
        Color(p.0, p.1, p.2)
    }
}

fn question3() {
    // let v = Point(__, __, __); //原题
    let v = Point(0,127,255); //原题
    // check_color(v);  //原题
    check_color(v.into()); //方法1
    //error:expected struct `Color`, found struct `Point`
}   

fn check_color(p: Color) {
    // let (x, _, _) = p; //原题
    let Color(x, _, _) = p; //方法1
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    // assert_eq!(__, 255); //原题
    assert_eq!(p.2, 255);
 }


 #[derive(Debug)]
struct Person2 {
    name: String,
    age: u8,
}

/// 🌟 你可以在实例化一个结构体时将它整体标记为可变的，
/// 但是 Rust 不允许我们将结构体的某个字段专门指定为可变的.
fn question4() {
    let age = 18;
    //原题
    // let p = Person2 {
    //     name: String::from("sunface"),
    //     age,
    // };
    let mut p = Person2 {
        name: String::from("sunface"),
        age,
    };

    // how can you believe sunface is only 18? 
    p.age = 30;

    // 填空
    // __ = String::from("sunfei"); //原题
    p.name = String::from("sunfei"); //不可修改

    dbg!(p);
}


// 填空
struct Person3 {
    name: String,
    age: u8,
}
fn question5() {} 

/// 通过name和age和Person3的成员重名来初始化结构体
fn build_person(name: String, age: u8) -> Person3 {
    Person3 {
        age,
        // __ //原题
        name
    }
}


// 填空，让代码工作
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn question6() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    dbg!(u2);
} 

fn set_email(u: User) -> User {
//     User {
//         email: String::from("contact@im.dev"),
//         __
//     }
// 原题
    User{
        email: String::from("contact@im.dev"),
        ..u //使用结构体更新语法来创建一个新的User实例，除了email字段被更新了以外，其他字段的值都和u一样。
    };

    // u //错误:u部分成员已经move掉了
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
        question6();
    }
}
