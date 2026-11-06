use std::num::ParseIntError;
/// 方式1: 使用 Result<T, E> 来处理可能的错误，避免直接使用 unwrap 导致 panic。
///     Result<T> 是一个枚举类型用于描述返回的结果或错误，它包含两个成员(变体 variants) :
///     Ok(T): 返回一个结果值 T
///     Err(e): 返回一个错误，e 是具体的错误值
///
/// 方式2: ? 跟 unwrap 非常像，但是 ? 会返回一个错误，而不是直接 panic.
///
/// 方式3:
///     map and and_then 是两个常用的组合器( combinator )，可以用于 Result<T, E> (也可用于 Option<T>).
///
/// 事实上 main 函数还可以返回一个 Result 类型：如果 main 函数内部发生了错误，那该错误会被返回并且打印出一条错误的 debug 信息。
/// 
/// 如果我们要在代码中到处使用 std::result::Result<T, ParseIntError> ，
///     那毫无疑问，代码将变得特别冗长和啰嗦，对于这种情况，可以使用类型别名来解决
///     例如:
///     type ParseResult<T> = Result<T, ParseIntError>;
// 填空并修复错误

// fn multiply(n1_str: &str, n2_str: &str) -> __ { //原题
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn question1() {
    let result = multiply("10", "2");
    // assert_eq!(result, __);
    assert_eq!(result, Ok(20));

    let result = multiply("4", "2");
    // assert_eq!(result.__, 8);
    assert_eq!(result.unwrap(), 8);

    println!("Success!")
}

// 使用 `?` 来实现 multiply
// 不要使用 unwrap !
fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

fn example2() {
    assert_eq!(multiply1("3", "4").unwrap(), 12);
    println!("Success!")
}

use std::fs::File;
use std::io::{self, Read};

fn read_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 填空
// 不要修改其它代码
fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    // __; //原题
    File::open("hello.txt")?.read_to_string(&mut s)?;
    //分级使用?
    Ok(s)
}

fn example3() {
    assert_eq!(
        read_file1().unwrap_err().to_string(),
        read_file2().unwrap_err().to_string()
    );
    println!("Success!")
}

// 使用两种方式填空: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    //    n_str.parse::<i32>().__ //原题
    n_str.parse::<i32>().map(|n| n + 2)
}

fn example4() {
    assert_eq!(add_two("4").unwrap(), 6);
    println!("Success!")
}

mod question5 {
    use std::num::ParseIntError;

    // 使用 Result 重写后，我们使用模式匹配的方式来处理，而无需使用 `unwrap`
    // 但是这种写法实在过于啰嗦..
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        match n1_str.parse::<i32>() {
            Ok(n1) => match n2_str.parse::<i32>() {
                Ok(n2) => Ok(n1 * n2),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    // 重写上面的 `multiply` ，让它尽量简洁
    // 提示：使用 `and_then` 和 `map`
    fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;
        Ok(n1 * n2)
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn example() {
        let twenty = multiply1("10", "2");
        print(twenty);

        // 下面的调用会提供更有帮助的错误信息
        let tt = multiply("t", "2");
        print(tt);

        println!("Success!")
    }
}

mod question6 {
    // 填空
    // type __;
    type Res<T> = Result<T, super::ParseIntError>;

    // 使用上面的别名来引用原来的 `Result` 类型
    fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
    }

    // 同样, 这里也使用了类型别名来简化代码
    fn print(result: Res<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn example() {
        print(multiply("10", "2"));
        print(multiply("t", "2"));
        println!("Success!")
    }
}


fn example7() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_question1() {
        question1();
    }
    #[test]
    fn test_example2() {
        example2();
    }
    #[test]
    fn test_example3() {
        example3();
    }
    #[test]
    fn test_example4() {
        example4();
    }
    #[test]
    fn test_question5() {
        question5::example();
    }
    #[test]
    fn test_question6(){
        question6::example();
    }
    #[test]
    fn test_example7() {
        example7().unwrap();
    }
}
