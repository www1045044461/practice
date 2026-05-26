/// 语句和表达式的
fn question1() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 下面表达式的值将被赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

/// 使用两种方式让代码工作
fn question2()
{
    // let v = {
    //     let mut x = 1;
    //     x += 2;
    // };
    // assert!(v == 3);
    //原题

    ///方法1
    let v = {
        let mut x = 1;
        x += 2;
        x
    };
    assert!(v == 3);

    //方法2
    let v1 = {
    
        let mut x = 1;
        x += 2;
    };
    assert_eq!(v1,());
}

/// 问题3: 修改代码让它工作
fn question3() {
//    let v = (let x = 3); // 原题
   let v = {
        let x = 3;
        x
    };
   assert!(v == 3);
}


fn question4() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    // x + y; // 原题
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question1() {
        question1();
        question2();
        question3();
        question4();
    }
}

