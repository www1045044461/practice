/// 特征 Trait 可以告诉编译器一个特定的类型所具有的、且能跟其它类型共享的特性。
/// 我们可以使用特征通过抽象的方式来定义这种共享行为，还可以使用特征约束来限定一个泛型类型必须要具有某个特定的行为。

struct Sheep {
    naked: bool,
    name: String,
}

trait Animal {
    /// 关联函数签名；`Self` 指代实现者的类型
    /// 例如我们在为 Pig 类型实现特征时，那 `new` 函数就会返回一个 `Pig` 类型的实例，
    /// 这里的 `Self` 指代的就是 `Pig` 类型
    /// 返回dyn时，`Self: Sized` 约束是必须的，因为编译器需要知道具体的类型大小。
    /// 否则:
    /// the trait `traits_impl::Animal` is not dyn compatible
    /// the following types implement `traits_impl::Animal`:
    ///   generics_traits::traits_impl::Sheep
    ///   generics_traits::traits_impl::Cow
    /// consider defining an enum where each variant holds one of these types,
    /// implementing `traits_impl::Animal` for this new enum and using it instead
    fn new(name: String) -> Self
        where Self: Sized;

    fn name(&self) -> String;

    fn noise(&self) -> String;

    /// traits方法中还能提供默认的定义实现
    fn talk(&self) -> String {
        format!("{} makes a noise: {}", self.name(), self.noise())
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // `Sheep` 结构体上定义的方法可以调用 `Sheep` 所实现的特征的方法
            println!("{} is already naked...", self.name());
        }
    }
}

impl Animal for Sheep {
    ///`Self` 被替换成具体的实现者类型： `Sheep`
    fn new(name: String) -> Self {
        println!("Create Type:{:?}", std::any::type_name::<Self>());
        Sheep { naked: false, name }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn noise(&self) -> String {
        String::from("baa")
    }

    /// 默认的特征方法可以被重写
    fn talk(&self) -> String {
        format!("{} says: {}", self.name(), self.noise())
    }
}

///

// 完成两个 `impl` 语句块
// 不要修改 `main` 中的代码
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}
impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
    fn say_hi(&self) -> String {
        String::from("hi")
    }
}
struct Teacher {}
impl Hello for Teacher {
    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher")
    }

    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }
}

/// 3. 我们可以使用 #[derive] 属性来派生一些特征，对于这些特征编译器会自动进行默认实现，
/// 对于日常代码开发而言，这是非常方便的，例如大家经常用到的 Debug 特征，就是直接通过派生来获取默认实现，而无需我们手动去完成这个工作。
/// 想要查看更多信息，可以访问

// `Centimeters`, 一个元组结构体，可以被比较大小
#[derive(PartialEq, PartialOrd, Debug)]
struct Centimeters(f64);

// `Inches`, 一个元组结构体可以被打印
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// 添加一些属性让代码工作
// 不要修改其它代码！
#[derive(Debug, PartialEq, PartialOrd)]
struct Seconds(i32);

/// 3. 在 Rust 中，许多运算符都可以被重载，事实上，运算符仅仅是特征方法调用的语法糖。
/// 例如 a + b 中的 + 是 std::ops::Add 特征的 add 方法调用，因此我们可以为自定义类型实现该特征来支持该类型的加法运算。
use std::{ops};
// 实现 fn multiply 方法
// 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
// 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/
fn multiply<T>(a: T, b: T) -> T
where
    T: std::ops::Mul<Output = T>,
{
    a * b
}

// 修复错误，不要修改 `main` 中的代码!
struct Foo;
struct Bar;

struct FooBar;

struct BarFoo;

// 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl ops::Sub<Bar> for Foo {
    type Output = BarFoo;

    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}

/*****************************答案****************************************/

// 为FooBar添加==重载
impl std::cmp::PartialEq for FooBar {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl std::fmt::Debug for FooBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FooBar")
    }
}

impl std::fmt::Debug for BarFoo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BarFoo")
    }
}
/*********************************************************************/
/*************************答案2****************************************/
/// 注意减法运算符的实现需要同时关注self类型和rhs类型
impl ops::Sub<Foo> for Bar {
    type Output = BarFoo;

    fn sub(self, _rhs: Foo) -> BarFoo {
        BarFoo
    }
}
impl std::cmp::PartialEq for BarFoo {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}
/*********************************************************************/

///除了使用具体类型来作为函数参数，我们还能通过 impl Trait 的方式来指定实现了该特征的参数：
/// 该参数能接受的类型必须要实现指定的特征。

// 实现 `fn summary`
// 修复错误且不要移除任何代码行
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

fn summary(item: &impl Summary) {
    println!("{}", item.summarize());
}

/************************Example6**************************************/
struct Cow{
    name: String,
    naked: bool,
}
impl Animal for Cow {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn new(name: String) -> Self {
        Cow { name, naked: true }
    }

    fn noise(&self) -> String {
        if self.naked {
            "Moo!".to_string()
        } else {
            "Baa!".to_string()
        }
    }

    fn talk(&self) -> String {
        format!("{} says {}", self.name, self.noise())
    }

}

// 错误:编译期并不知道返回什么类型
// fn random_animal1(random_number: f64) -> impl Animal {
//     if random_number < 0.5 {
//         Sheep { name: "Dolly".to_string() , naked:false}
//     } else {
//         Cow { name: "Bessie".to_string(),naked:true }
//     }
// }

// 错误同上
// fn random_animal1(random_number: f64) -> Box<impl Animal> {
//     if random_number < 0.5 {
//         Box::new(Sheep { name: "Dolly".to_string() , naked:false})
//     } else {
//         Box::new(Cow { name: "Bessie".to_string(),naked:true })
//     }
// }

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep { name: "Dolly".to_string() , naked:false})
    } else {
        Box::new(Cow { name: "Bessie".to_string(),naked:true })
    }
}

/********************Example7************************************* */
// 通过两种方法使用特征约束来实现 `fn sum`
fn sum<T>(x: T, y: T) -> T 
where T:std::ops::Add<Output = T>,
{
    x + y
}

fn sum1<T:std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

/*********************Example8***********************************/
// 修复代码中的错误
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}

//错误内容
// struct Unit(i32);

#[derive(Debug, PartialOrd, PartialEq)]
struct Unit(i32);

/*********************Example9***********************************/
// 填空
fn example1() {
    // `T: Trait` 是最常使用的方式
    // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    // assert_eq!(cacher.value(10), __);
    // assert_eq!(cacher.value(15), __);
    assert_eq!(cacher.value(10), 11);
    assert_eq!(cacher.value(15), 11); 
    //错误原因: Cacher 结构体只缓存了第一次计算的结果，因此无论传入什么参数，都会返回第一次计算的值。
}


fn example2() {
    // 还可以使用 `where` 来约束 T
    struct Cacher<T>
        where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    // assert_eq!(cacher.value(20), __);
    // assert_eq!(cacher.value(25), __);
    assert_eq!(cacher.value(20), 21);
    assert_eq!(cacher.value(25), 21);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 这里的类型注释时必须的
        let mut dolly: Sheep = Animal::new("Dolly".to_string());
        // TODO ^ 尝试去除类型注释，看看会发生什么
        dolly.talk();
        dolly.shear();
        dolly.talk();
    }

    #[test]
    fn test2() {
        let s = Student {};
        assert_eq!(s.say_hi(), "hi");
        assert_eq!(s.say_something(), "I'm a good student");

        let t = Teacher {};
        assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
        assert_eq!(t.say_something(), "I'm not a bad teacher");

        println!("Success!")
    }

    #[test]
    fn test3() {
        let _one_second = Seconds(1);

        println!("One second looks like: {:?}", _one_second);
        let _this_is_true = _one_second == _one_second;
        let _this_is_false = _one_second > _one_second;

        let foot = Inches(12);

        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100.0);

        let cmp = if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

        println!("One foot is {} than one meter.", cmp);
    }

    #[test]
    fn test4() {
        assert_eq!(multiply(2u8, 3u8), 6);
        assert_eq!(multiply(1.0, 5.0), 5.0);
    }

    #[test]
    fn test5() {
        // 不要修改下面代码
        // 你需要为 FooBar 派生一些特征来让代码工作
        assert_eq!(Foo + Bar, FooBar);
        assert_eq!(Foo - Bar, BarFoo);

        println!("Success!")
    }

    #[test]
    fn test6() {
        let post = Post {
            title: "Popular Rust".to_string(),
            author: "Sunface".to_string(),
            content: "Rust is awesome!".to_string(),
        };
        let weibo = Weibo {
            username: "sunface".to_string(),
            content: "Weibo seems to be worse than Tweet".to_string(),
        };

        summary(&post);
        summary(&weibo);
        // 原题

        println!("{:?}", post);
        println!("{:?}", weibo);
    }

    #[test]
    fn test7() {
        // 通过impl `std::ops::Add` 特征约束实现 `sum`
        assert_eq!(sum(2, 3), 5);
        assert_eq!(sum(1.5, 2.5), 4.0);
    }

    #[test]
    fn test8() {
        let pair = Pair{
            x: Unit(1),
            y: Unit(3)
        };

        pair.cmp_display(); 
        //无法编译，因为 Unit 没有实现 Debug 和 PartialOrd 特征
        //需要为Unit实现Debug和PartialOrd特征

    }

    #[test]
    fn test9() {
        // 这里可以添加针对 example1 和 example2 的测试
        example1();
        example2();
    }
}
