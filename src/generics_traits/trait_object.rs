///
/// 在特征练习中 我们已经知道当函数返回多个类型时，impl Trait 是无法使用的。
/// 对于数组而言，其中一个限制就是无法存储不同类型的元素，但是通过之前的学习，
/// 大家应该知道枚举可以在部分场景解决这种问题，但是这种方法局限性较大。此时就需要我们的主角登场了。

/**********************************************************/
/// Example1:
/// Rust 编译器需要知道一个函数的返回类型占用多少内存空间。
/// 由于特征的不同实现类型可能会占用不同的内存，因此通过 impl Trait 返回多个类型是不被允许的，
/// 但是我们可以返回一个 dyn 特征对象来解决问题。

trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

// 实现以下函数
fn hatch_a_bird(value: u8) -> Box<dyn Bird> {
    if value == 1 {
        Box::new(Swan)
    } else {
        Box::new(Duck)
    }
}
/*********************************************************************/
/// Example3:
/// &dyn and Box<dyn>

// 填空
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

/// Runtime polymorphism and ownership changed!
fn draw_with_box(x: Box<dyn Draw>) {
    let _str = x.draw();
    // print function signature and result
    println!("__FUNCTION__: draw_with_box");
    println!("__FUNCTION__: {}", _str);
}

/// Static polymorphism with references
fn draw_with_ref(x: &impl Draw) {
    println!("{}", std::any::type_name_of_val(x));
    println!("__FUNCTION__: {}", x.draw());
}

/// Takes ownership of a value that implements the Draw trait and draws it.
fn draw_with_moved_impl(x: impl Draw) {
    println!("__FUNCTION__: draw_with_moved_impl");
    println!("__FUNCTION__: {}", x.draw());
}

/// Takes a reference to a value that implements the Draw trait and draws it.
fn draw_with_dyn(x: &dyn Draw) {
    println!("__FUNCTION__: draw_with_dyn");
    println!("__FUNCTION__: {}", x.draw());
}

/***************************** Example4:*******************************8*/
trait Foo {
    fn method(&self) -> String;
}
impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("String: {}", *self)
    }
}

// fn static_dispatch<T:Foo>(x: T) {
fn static_dispatch<T>(x: T)
where
    T: Foo,
{
    println!("__FUNCTION__: static_dispatch");
    println!("__FUNCTION__: {}", x.method());
}

fn dynamic_dispatch(x: &dyn Foo) {
    println!("__FUNCTION__: dynamic_dispatch");
    println!("__FUNCTION__: {}", x.method());
}

/*******************Example5***************************/
/// 一个特征能变成特征对象 == 该特征必须是对象安全的 (object safe)
/// 1. 特征的方法不能返回 Self 类型，除非它是通过引用或 Box 包装的。
/// 2. 特征的方法不能是泛型的。
/// 勘误:
/// 1. 直接返回 Self 类型的方法无法作为特征对象使用，必须改为返回 Box<dyn MyTrait>。
/// 2. 不变;
///
/// 原理分析:
/// dyn对象安全定义:
///  对于dyn Trait这样一个特征对象，存在一个实现类型X,在X的对象转换为dyn Trait并调用Trait的方法时,所有方法的调用过程都必须明确:
///  1.
///  2.

// 使用至少两种方法让一下代码工作,不要添加/删除任何代码行

// 方法1: 使用 Box<Self> 代替 Self
mod example5 {
    // trait MyTrait {
    //     fn f(&self) -> Self;
    // }

    // impl MyTrait for u32 {
    //     fn f(&self) -> Self {
    //         42
    //     }
    // }

    // impl MyTrait for String {
    //     fn f(&self)->Self {
    //         self.clone()
    //     }
    // }

    // fn my_function(x: Box<dyn MyTrait>)
    // {
    //     x.f();
    // }

    // fn example5_question() {
    //     my_function(Box::new(13_u32));
    //     my_function(Box::new(String::from("hello")));
    // }
} // end of mod example5

//思路1:由于特征的方法返回 Self 类型，因此它不是对象安全的，不能直接作为特征对象使用。
// 使用静态分发来解决对象安全问题
mod example5_impl1 {
    trait MyTrait {
        fn f(&self) -> Self;
    }

    impl MyTrait for u32 {
        fn f(&self) -> Self {
            42
        }
    }

    impl MyTrait for String {
        fn f(&self) -> Self {
            self.clone()
        }
    }

    fn example5_question<T: MyTrait>(x: T) {
        x.f();
    }

    pub fn answer() {
        example5_question(13_u32);
        example5_question(String::from("hello"));
    }
}

//思路2:首先将Trait的方法返回类型改为 Box<dyn MyTrait>，使其对象安全
mod example5_impl2 {
    trait MyTrait {
        //误区: 直接返回 Box<Self> 仍然无法作为特征对象使用，必须改为 Box<dyn MyTrait>
        fn f(&self) -> Box<dyn MyTrait>;
    }

    impl MyTrait for u32 {
        fn f(&self) -> Box<dyn MyTrait> {
            Box::new(42)
        }
    }

    impl MyTrait for String {
        fn f(&self) -> Box<dyn MyTrait> {
            Box::new(self.clone())
        }
    }

    fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
        x.f()
    }

    pub fn example5_question() {
        my_function(Box::new(13_u32));
        my_function(Box::new(String::from("hello")));
    }
}

mod example5_extends {
    use crate::generics_traits::trait_object;

    trait MyTrait {
        fn describe(&self) -> String;

        fn duplicate(&self) -> Self
        where
            Self: Sized;

        fn inspect<T>(&self, _value: T)
        where
            Self: Sized;
    }

    impl MyTrait for u32 {
        fn describe(&self) -> String {
            println!("Describing u32: {}", self);
            format!("u32: {}", self)
        }

        fn duplicate(&self) -> Self
        where
            Self: Sized,
        {
            println!("Duplicating u32: {}", self);
            *self
        }

        fn inspect<T>(&self, _value: T)
        where
            Self: Sized,
        {
            // println!("Inspecting u32 with value: {:?}", _value);
            // do nothing
        }
    }

    impl MyTrait for String  {
        fn describe(&self) -> String {
            println!("Describing String: {}", self);
            format!("String: {}", self)
        }

        fn duplicate(&self) -> Self
        where
            Self: Sized,
        {
            println!("Duplicating String: {}", self);
            self.clone()
        }

        fn inspect<T>(&self, _value: T)
        where
            Self: Sized,
        {
            // println!("Inspecting String with value: {:?}", _value);
            // do nothing
        }
    }

    pub fn example5_extends() {
    {
        let text = String::from("hello");
        let trait_object:&dyn MyTrait = &text;

        assert_eq!(text.duplicate(), "hello"); //Self==String 可以調用
        text.inspect(42); //Self==String 可以調用

        assert_eq!(trait_object.describe(), "String: hello"); 
        //經過虛表調用 describe 方法

        // trait_object.duplicate(); // 错误：dyn MyTrait 不满足 Self: Sized
        // the `duplicate` method cannot be invoked on a trait object;
        // the method cannot be invoked on a trait object because its `Self: Sized` bound is not satisfied
        // trait_object.inspect(42);  
        // 同上

    }
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bird() {
        // 填空
        let duck = Box::new(Duck);
        duck.swim();
        let bird = hatch_a_bird(2);
        // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
        // bird.swim();
        // 但它依然可以叫唤
        assert_eq!(bird.quack(), "duck duck");

        let bird = hatch_a_bird(1);
        // 这只鸟儿忘了如何飞翔，因此以下代码会报错
        // bird.fly();
        // 但它也可以叫唤
        assert_eq!(bird.quack(), "swan swan");

        println!("Success!");
    }

    /// Example2: 使用 dyn 特征对象创建数组
    #[test]
    fn test_array_dyn() {
        // let _arrars = [Box::new(Duck) as Box<dyn Bird>, Box::new(Swan) as Box<dyn Bird>]; //using as
        let _arrarys: [Box<dyn Bird>; 2] = [Box::new(Duck), Box::new(Swan)]; //use left type bound!
        for bird in &_arrarys {
            println!("{}", bird.quack());
        }
    }

    #[test]
    fn test_draw() {
        let x = 1.1f64;
        let y = 8u8;

        // draw x
        draw_with_box(Box::new(x));
        // draw y
        draw_with_ref(&y);

        draw_with_dyn(&x);

        draw_with_moved_impl(y);
        draw_with_moved_impl(x);

        println!("Success!")
    }

    /// Example4: 静态分发与动态分发
    #[test]
    fn test_foo() {
        let x = 5u8;
        let y = String::from("hello");

        static_dispatch(x);
        static_dispatch(y.clone());

        dynamic_dispatch(&x);
        dynamic_dispatch(&y);
    }

    #[test]
    fn test_example5_question() {
        // example5_question();
        example5_impl1::answer();
    }

    
    #[test]
    fn test_example5_extends() 
    {
        example5_extends::example5_extends();
    }
}
