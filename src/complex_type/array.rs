/// 数组部分
/// 数组类型[T;N] 类型和长度必须编译期间已知


fn question1() {
    // 使用合适的类型填空
    // let arr: __ = [1, 2, 3, 4, 5];
    let arr:[i32;5] = [1, 2, 3, 4, 5];

    // 修改以下代码，让它顺利运行
    // assert!(arr.len() == 4);
    assert!(arr.len() == 5);
}


fn question2() {
    // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    
    // 填空
    // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
    // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
    // assert!(std::mem::size_of_val(&arr) == __);
    assert!(std::mem::size_of_val(&arr) == 12);
}

///🌟 数组中的所有元素可以一起初始化为同一个值
fn question3() {
    // 填空
    // let list: [i32; 100] = __ ; // 原题
    let list:[i32;100] = [1;100];
    assert!(list[0] == 1);
    assert!(list.len() == 100);
}

/// 数组中的元素必须是同一类型的 
fn question4 () {
    // 修复错误
    // let _arr = [1, 2, '3']; // 原题
    let _arr = [1, 2, 3];
}


/// 🌟 数组的下标索引从 0 开始.
fn question5() {
    let arr = ['a', 'b', 'c'];
    // let ele = arr[1]; // 只修改此行来让代码工作;原题
    let ele = arr[0];
    assert!(ele == 'a');
}

/// 
/// 6.越界代码会导致panic 
fn question6() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
    let name0 = names.get(0).unwrap();

    // 但是下标索引就存在越界的风险了
    // let _name1 = &names[2]; //原题
    let _name1 = &names[1];
    println!("Success! {}", name0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question1() {
        // question1();
        // question2();
        // question3();
        // question3();
        // question5();
        question6();
    }
}