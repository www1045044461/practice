//! 高级特征示例
//! 1.关联类型主要用于提升代码的可读性:极大的减少其它类型在实现该特征时所需的模版代码
//!     思维方法:对于带有泛型参数的特征,可以考虑使用关联类型来简化实现,尤其对于泛型函数中涉及该特征的情况
//! 2.当我们使用泛型类型参数时，可以为该泛型参数指定一个具体的默认类型，这样当实现该特征时，如果该默认类型可以使用，那用户再无需手动指定具体的类型。
//!     思维方法:impl 和 impl for 语句中Self类型可以作为关联类型的具体类型使用,直接引用Self
//!     当Rhs为单类型参数时可以忽略Self类型的显式指定
//! 3. 全限定引用多trait的同名方法;
//!     思维方法:当一个类型实现了多个特征且这些特征中有同名方法时，可以使用全限定语法来明确调用哪一个特征的方法
//!

/************************Example1*************************************/
mod example1 {
    struct Container(i32, i32);

    //使用关联类型实现重新实现以下特征
    //trait Contains{
    // type A;
    // type B;
    //};

    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains<i32, i32> for Container {
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (self.0 == *number_1) && (self.1 == *number_2)
        }
        fn first(&self) -> i32 {
            self.0
        }
        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
        container.last() - container.first()
    }

    pub fn example() {
        let number_1 = 3;
        let number_2 = 10;
        let container = Container(number_1, number_2);
        println!(
            "Does container contain {} and {}? {}",
            number_1,
            number_2,
            container.contains(&number_1, &number_2)
        );

        println!("First number:{}", container.first());
        println!("Last number:{}", container.last());

        println!("The difference is :{}", difference(&container));
    }
}

mod example1_slu1 {
    struct Container(i32, i32);
    trait Contains {
        type A;
        type B;
        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        type A = i32;
        type B = i32;

        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (self.0 == *number_1) && (self.1 == *number_2)
        }
        fn first(&self) -> i32 {
            self.0
        }
        fn last(&self) -> i32 {
            self.1
        }
    }

    ///使用关联类型实现的 Contains 特征计算差值
    fn difference(container: &Container) -> i32 {
        container.last() - container.first()
    }

    pub fn example() {
        let number_1 = 3;
        let number_2 = 10;
        let container = Container(number_1, number_2);
        println!(
            "Does container contain {} and {}? {}",
            number_1,
            number_2,
            container.contains(&number_1, &number_2)
        );

        println!("First number:{}", container.first());
        println!("Last number:{}", container.last());

        println!("The difference is :{}", difference(&container));
    }
}

mod exmple2 {
    use std::ops::Add;
    use std::ops::Mul;
    use std::ops::Sub;
    #[derive(Debug, PartialEq, Clone)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // 用三种方法填空: 其中两种使用默认的泛型参数，另外一种不使用
    // impl __ {
    //     type Output = Self;

    //     fn sub(self, other: Self) -> Self::Output {
    //         Point {
    //             x: self.x - other.x,
    //             y: self.y - other.y,
    //         }
    //     }
    // }

    ///默认解法1:外加where约束
    // impl<T> Sub for Point<T>
    // where
    //     T: Sub<Output = T>,
    // {
    //     type Output = Self;

    //     fn sub(self, other: Self) -> Self::Output {
    //         Point {
    //             x: self.x - other.x,
    //             y: self.y - other.y,
    //         }
    //     }
    // }

    /// 方法2:感觉更加复杂了,
    impl<T: Sub<Output = T>> Sub<Point<T>> for Point<T> {
        type Output = Self;
        fn sub(self, rhs: Point<T>) -> Self::Output {
            Point {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    /// 方法3: 忽略Trait处的泛型
    /// 可以为该泛型参数指定一个具体的默认类型，这样当实现该特征时，如果该默认类型可以使用，那用户再无需手动指定具体的类型。
    impl<T: Add<Output = T>> Add for Point<T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    ///方法4: Trait处使用内部关联类型
    /// Self可以作为Mul的右操作数类型，这样在实现Mul特征时就不需要再指定具体的类型。
    /// 这个类型在 (impl for 类型) 和(impl 类型),Self==这个类型;
    impl<T: Mul<Output = T>> Mul<Self> for Point<T> {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            }
        }
    }

    pub fn example() {
        assert_eq!(
            Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
            Point { x: 1, y: 3 }
        );

        let p1 = Point { x: 2, y: 3 };
        let p2 = Point { x: 4, y: 5 };
        let _p1 = p1.clone();
        let _p2 = p2.clone();
        let p3 = p1 * p2;

        let p4 = _p1 + _p2;

        assert_eq!(p3, Point { x: 8, y: 15 });
        assert_eq!(p4, Point { x: 6, y: 8 });

        println!("Success!")
    }
}

mod example3 {
    trait UsernameWidget {
        fn get(&self) -> String;
    }
    trait AgeWidget {
        fn get(&self) -> u32;
    }

    struct Form {
        username: String,
        age: u32,
    }

    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }
    impl AgeWidget for Form {
        fn get(&self) -> u32 {
            self.age
        }
    }

    pub fn example() {
        let form = Form {
            username: "Alice".to_string(),
            age: 30,
        };
        // println!("Username: {}", form.get()); //多个get方法被找到;
        // println!("Age: {}", form.get()); //同上

        println!("Use Full Qualified Syntax");
        println!("User name:{}", UsernameWidget::get(&form));
        println!("Age :{}", AgeWidget::get(&form));
    }
}

mod example4 {
    trait Pilot {
        fn fly(&self) -> String;
    }

    trait Wizard {
        fn fly(&self) -> String;
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) -> String {
            String::from("This is your captain speaking.")
        }
    }

    impl Wizard for Human {
        fn fly(&self) -> String {
            String::from("Up!")
        }
    }

    impl Human {
        fn fly(&self) -> String {
            String::from("*waving arms furiously*")
        }
    }

    pub fn example() {
        let person = Human;

        assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
        assert_eq!(Wizard::fly(&person), "Up!");

        // assert_eq!(__, "*waving arms furiously*"); //原题
        assert_eq!(person.fly(), "*waving arms furiously*");

        println!("Success!")
    }
}

mod Supertraits_Example {

    trait Person {
        fn name(&self) -> String;
    }

    // Person 是 Student 的 supertrait .
    // 实现 Student 需要同时实现 Person.
    trait Student: Person {
        fn university(&self) -> String;
    }

    trait Programmer {
        fn fav_language(&self) -> String;
    }

    // CompSciStudent (computer science student) 是 Programmer
    // 和 Student 的 subtrait. 实现 CompSciStudent 需要先实现这两个 supertraits.
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }

    struct CSStudent {
        name: String,
        university: String,
        fav_language: String,
        git_username: String,
    }

    // 为 CSStudent 实现所需的特征
    // impl ...

    impl Programmer for CSStudent  {
        fn fav_language(&self) -> String {
            self.fav_language.clone()
        }
    }

    impl Person for CSStudent {
        fn name(&self) -> String {
            self.name.clone()
        }
    }

    /// 依赖 Person 特征
    impl Student for CSStudent { 
        fn university(&self) -> String {
            self.university.clone()
        }
    }

    /// 依赖 Student 和 Programmer 特征
    impl CompSciStudent for CSStudent {
        fn git_username(&self) -> String {
            self.git_username.clone()
        }
    }

    pub fn example() {
        let student = CSStudent {
            name: "Sunfei".to_string(),
            university: "XXX".to_string(),
            fav_language: "Rust".to_string(),
            git_username: "sunface".to_string(),
        };

        // 填空
        // println!("{}", comp_sci_student_greeting(__)); //原题
        println!("{}", comp_sci_student_greeting(&student));
    }
}

// 孤儿原则
mod orphan_rule_example {

    trait Animal {
        fn name(&self) -> String;
    }

    struct Dog {
        name: String,
    }


    /// 类型和trait都在本地模块中，遵循孤儿原则，可以为 Dog 实现 Animal 特征
    // 为 Dog 实现 Animal 特征
    impl Animal for Dog {
        fn name(&self) -> String {
            self.name.clone()
        }
    }

    //trait为本地，遵循孤儿原则，可以为 u32 实现 Animal 特征
    impl Animal for u32{
        fn name(&self) -> String {
            self.to_string()
        }
    }

    // 为 Dog 实现 Display 特征，遵循孤儿原则，因为 Dog 类型在本地模块中
    impl std::fmt::Display for Dog {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Dog's name is {}", self.name)
        }
    }

    /// only traits defined in the current crate can be implemented for types defined outside of the crate
    /// impl doesn't have any local type before any uncovered type parameters
    /// for more information see https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
    /// define and implement a trait or new type instead
    // impl  std::fmt::Display for String {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "String is {}", self)
    //     }
    // }

    pub fn example() {
        let dog = Dog {
            name: "Buddy".to_string(),
        };
        println!("Dog's name is {}", dog.name());
    }
}

mod new_type_example {

    /// 孤儿规则违反
    /// 不能为外部类型实现外部trait
    /// 这里为 String 实现 Display 特征违反了孤儿规则
    // impl std::fmt::Display for String{
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "String is {}", self)
    //     }
    // }
    
    /// 本地newtype，遵循孤儿规则，可以为其实现外部trait
    struct MyString(String);

    /// 为 MyString 实现 Display 特征，遵循孤儿规则
    impl std::fmt::Display for MyString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyString is {}", self.0)
        }
    }

    trait SelfDisplay  
    {
        fn fmt(&self,f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }

    ///错误实现
    // impl SelfDisplay for String {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "String is {}", self)
    //     }
    // }

    //正确实现
    impl SelfDisplay for String {
        fn fmt(&self,f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "String is {}", self)
        }
    }

    pub fn example() {
        let my_string = MyString("Hello".to_string());
        println!("{}---->", my_string);

        let std_string = String::from("This is std String");
        let formatter_info = format!("{}", std_string);
        println!("{}--->", formatter_info);
    }
}

#[cfg(test)]
mod tests {
    use crate::generics_traits::advance_traits::{example1, example1_slu1, exmple2};

    use super::example1::*;

    #[test]
    fn test_example() {
        println!("using generic example:");
        example1::example();
        println!("using associated type example:");
        example1_slu1::example();
    }

    #[test]
    fn test_example2() {
        exmple2::example();
    }

    #[test]
    fn test_example3() {
        super::example3::example();
        super::example4::example();
    }

    #[test]
    fn test_example4()
    {
        super::Supertraits_Example::example();
    }

    #[test]
    fn test_new_type_example() {
        super::orphan_rule_example::example();
        super::new_type_example::example();
    }
}
