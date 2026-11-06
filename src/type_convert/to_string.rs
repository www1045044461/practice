///
/// 1.只要为一个类型实现了 ToString，就可以将任何类型转换成 String;但是更好的使用Display特征来实现格式化输出。
/// 2.使用 parse 方法可以将一个 String 转换成 i32 数字，这是因为在标准库中为 i32 类型实现了 FromStr: : impl FromStr for i32
/// 3.还可以为自定义类型实现 FromStr 特征;
/// 4.std::mem::transmute 可以在类型之间进行位级别的转换，但需要谨慎使用，因为它可能导致未定义行为。
///     a. transmute 可以将一个指针转换成一个函数指针，该转换并不具备可移植性，原因是在不同机器上，函数指针和数据指针可能有不同的位数( size )。
///     b. transmute 还可以扩展或缩短一个不变量的生命周期，将 Unsafe Rust 的不安全性体现的淋漓尽致
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

// impl fmt::Display for Point {
//     // 实现 fmt 方法
// }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f,"({},{})",self.x,self.y)
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn answer1() {
    let origin = Point { x: 0, y: 0 };
    // 填空
    assert_eq!(&origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!")
}

// 为了使用 `from_str` 方法, 你需要引入该特征到当前作用域中
use std::str::FromStr;
fn question2() {
    // let parsed: i32 = "5".__.unwrap();
    // let turbo_parsed = "10".__.unwrap();
    // let from_str = __.unwrap();
    // let sum = parsed + turbo_parsed + from_str;
    // assert_eq!(sum, 35);
    // println!("Success!")
}

/// 使用 parse 方法将字符串转换为 i32 类型，
/// 并使用 FromStr 特征的 from_str 方法将字符串转换为 i32 类型。
fn answer2() {
    let parsed = "5".parse::<i32>().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);
    println!("Success!")
}

mod modname {
    use std::num::ParseIntError;
    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl FromStr for Point {
        type Err = ParseIntError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let coords: Vec<&str> = s
                .trim_matches(|p| p == '(' || p == ')')
                .split(',')
                .map(|x| x.trim())
                .collect();

            let x_fromstr = coords[0].parse::<i32>()?;
            let y_fromstr = coords[1].parse::<i32>()?;

            Ok(Point {
                x: x_fromstr,
                y: y_fromstr,
            })
        }
    }

    pub fn question3() {
        // 使用两种方式填空
        // 不要修改其它地方的代码
        let p = "(3,4)".parse::<Point>();
        let p1 = Point::from_str("(3,4)");
        assert_eq!(p.unwrap(), Point { x: 3, y: 4 });
        assert_eq!(p1.unwrap(), Point { x: 3, y: 4 });

        println!("Success!")
    }
}

fn foo() -> i32 {
    0
}
fn example4() {
    let pointer = foo as *const ();
    let function = unsafe { std::mem::transmute::<*const (), fn() -> i32>(pointer) };

    assert_eq!(function(), 0);
}

struct MyU16 {
    a: u8,
    b: u8,
}
fn example5() {
    let my_u16 = MyU16 { a: 1, b: 2 };
    let pointer = &my_u16 as *const MyU16;
    let u16_value = unsafe {
        let _p = std::mem::transmute::<*const MyU16, *const u16>(pointer);
        *_p
    };
    println!("u16_value: {}", u16_value);
    assert_eq!(u16_value, 0x0201);
}

struct R<'a>(&'a i32);
/// 扩展引用的生命周期到 'static
unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    std::mem::transmute::<R<'b>, R<'static>>(r)
}
unsafe fn shorten_lifetime<'a, 'c>(r: &'a mut R<'static>) -> &'a mut R<'c> {
    std::mem::transmute::<&'a mut R<'static>, &'a mut R<'c>>(r)
}

/// 其他代替std::mem::transmute的安全的方式
fn example6() {
    /*Turning raw bytes(&[u8]) to u32, f64, etc.: */
    let raw_bytes = [0x78, 0x56, 0x34, 0x12];

    let num = unsafe { std::mem::transmute::<[u8; 4], u32>(raw_bytes) };

    // use `u32::from_ne_bytes` instead
    let num = u32::from_ne_bytes(raw_bytes);
    // or use `u32::from_le_bytes` or `u32::from_be_bytes` to specify the endianness
    let num = u32::from_le_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);
    let num = u32::from_be_bytes(raw_bytes);
    assert_eq!(num, 0x78563412);

    /*Turning a pointer into a usize: */
    let ptr = &13;
    let ptr_num_transmute = unsafe { std::mem::transmute::<&i32, usize>(ptr) };

    // Use an `as` cast instead
    let ptr_num_cast = ptr as *const i32 as usize;

    let _number = unsafe {
        let _i32 = ptr as *const i32;
        *_i32
    };
    println!("转换后的值: {}", _number);

    /*Turning an &mut T into an &mut U: */
    let mut i32_v = i32::MAX;
    let ptr = &mut i32_v;
    
    // Now, put together `as` and reborrowing - note the chaining of `as`
    // `as` is not transitive
    let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };
    let val_transmuted = unsafe { std::mem::transmute::<&mut i32, &mut u32>(ptr) };
    println!("val_transmuted: {}->{}", i32_v, val_transmuted);

    /*Turning an &str into a &[u8]: */
    // this is not a good way to do this.
    let slice = unsafe { std::mem::transmute::<&str, &[u8]>("Rust") };
    assert_eq!(slice, &[82, 117, 115, 116]);

    // You could use `str::as_bytes`
    let slice = "Rust".as_bytes();
    assert_eq!(slice, &[82, 117, 115, 116]);

    // Or, just use a byte string, if you have control over the string
    // literal
    assert_eq!(b"Rust", &[82, 117, 115, 116]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer1() {
        answer1();
    }

    #[test]
    fn test_answer2() {
        answer2();
    }

    #[test]
    fn test_question3() {
        modname::question3();
    }

    #[test]
    fn test_example4() {
        example4();
        example5();
    }

    #[test]
    fn test_example5() {
        // example5();
        static VALUE: i32 = 42;

        let mut _r_global = R(&VALUE);
        let _global = R(&VALUE);

        //销毁时，局部引用的生命周期结束的错误演示
        //1.对象持有大的生命周期引用;
        //2.将该对象传进小scope;
        //3. 在小scope中将局部引用的生命周期扩展到 'static
        //4. 在小scope结束后，局部引用被销毁，导致外部的 'static 槽位持有悬垂引用
        {
            let _local = 14;
            let _r_local = R(&_local);
            // 将局部引用的生命周期扩展到 'static
            _r_global = unsafe { extend_lifetime(_r_local) };
        }
        println!("_r_global: {}", _r_global.0);


        //短期借用错误示例
        //1.对象持有大的生命周期引用;
        //2.将该对象传进小scope;
        //3. 在小scope中convert成局部短期借用并传递给外部的 'static 槽位
        //4. 在小scope结束后，局部短期借用被销毁，导致外部的 'static 槽位持有悬垂引用
        let mut slot:R<'static> = R(&VALUE);
        {
            let local = 7;
            let short = R(&local);
            let view = unsafe {shorten_lifetime(&mut slot)};
            *view = short; //将局部引用传递给短期借用的槽位
        }//local在这里销毁
        println!("slot: {}", slot.0);
        //输出乱码
    }

    #[test]
    fn test_example6() {
        example6();
    }
}
