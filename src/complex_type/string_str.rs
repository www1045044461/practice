/// 字符串和&str

// 修复错误，不要新增代码行
// 🌟 正常情况下我们无法使用 str 类型，但是可以使用 &str 来替代
fn question1() {
    // let s: str = "hello, world"; // 原题
    let s = "hello, world"; 
    println!("The string is: {}", s);
}

// 2.要持有str类型只有使用Box<str>，但是我们更常用String来持有字符串
// 使用至少两种方法来修复错误
fn question2() {

    //相当于1层指针,对实参解引用无效
    let s: Box<str> = "hello, world".into();
    println!("实参地址:{:p}",s); //Box实现了std::fmt::Pointer
    // println!("实参地址:{:p}",*s); //str没有实现Pointer所以需要解引用
    // greetings(s); //原题
    greetings(s.as_ref()); //方法1
    greetings(&s); //方法1

    println!("----------------------------");

    let s:Box<&str> = Box::new("hello, world");
    println!("实参地址:{:p}", s);
    println!("实参地址:{:p}", *s);
    greetings(*s); //相当于2层指针;
}

fn borrow_method1(mut value: Box<i32>)
{
    let reference: &mut i32 = value.as_mut();
    *reference = *reference + 1;
    println!("The value is: {} address:{:p}", reference,reference);
}

fn borrow_test()
{
    let value = Box::new(42);
    println!("Original value: {} address:{:p}", value, value);
    borrow_method1(value);
}

fn greetings(s: &str) {
    println!("{} 地址:{:p}",s, s)
}


fn param_method1() {
    let mut i = 13;
    let mut y = 14;
    let mut r1 = &i; //绑定r1可变但是指向的是不可变引用
    
    println!("r1: {}, address: {:p}", r1, r1);
    r1 = &y; //换指向
    println!("r1: {}, address: {:p}", r1, r1);

    // r1 = 13;
}

fn param_method2() {
    let mut i = 13;
    let mut y = 14;
    let r1 = &mut i; //绑定r1不可变但是指向的是可变引用
    
    println!("r1: {}, address: {:p}", r1, r1);
    // r1 = &mut y; //错误:不能对不可变绑定赋值两次
    *r1 = 25;
    println!("r1: {}, address: {:p}", r1, r1);
}

fn param_method3()
{
    let mut i = 13;
    let mut y = 14;
    let mut r1 = &mut i; //绑定r1不可变但是指向的是可变引用
    
    println!("初始值 r1: {}, address: {:p}", r1, r1);
    r1 = &mut y; //错误:不能对不可变绑定赋值两次
    println!("换指向 r1: {}, address: {:p}", r1, r1);
    *r1 =  25;
    println!("修改内容 r1: {}, address: {:p}", r1, r1);
}

//所有需要mut的型参例子
fn string_mut_method(mut s:String)
{
    s.push('|'); // push 需要 &mut self
}

fn vec_mut_method(mut v:Vec<i32>)
{
    v.push(42);
}

fn box_mut_method(mut b:Box<i32>)
{
    *b += 1; // 需要可变访问内部值
}

fn hashmap_mut_method(mut m:std::collections::HashMap<String, i32>)
{
    m.insert("key".to_string(), 42);
}

//错误的写法:expected type, found keyword `mut` expected type
// fn string_mut_method2(s:mut String)
// {
//     s.push('|');
// }


/// String本体分配在heap上,底层时Vec<u8>,UTF8编码 
fn question3() {
    // let mut s = __; // 原题
    let mut s = String::new(); //方法1
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
}


// 修复所有错误，并且不要新增代码行
fn  question4() {
    // let  s = String::from("hello");
    let mut s = String::from("hello");
    s.push(',');
    // s.push(" world");
    s.push_str(" world");
    // s += "!".to_string();
    s += "!";
    s.push_str("!");
    println!("{}", s)
}


// 使用replace方法替换字符串中的子串
fn question5() {
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    // let s1 = s.__("dogs", "cats");
    let s1 = s.replace("dogs", "cats");
    assert_eq!(s1, "I like cats")
}


/// 🌟🌟 你只能将 String 跟 &str 类型进行拼接，并且 String 的所有权在此过程中会被 move
// 修复所有错误，不要删除任何一行代码
fn question6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // let s3 = s1 + s2; 
    // let s3 = s1.clone() + s2.as_str();  //方法1
    let s3 = s1.clone() + &s2; 
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}


// 7.🌟🌟 我们可以使用两种方法将 &str 转换成 String 类型
fn question7() {
    let s = "hello, world";
    // greetings_string(s); //原题
    greetings_string(s.to_string());
    greetings_string(String::from(s));
}

fn greetings_string(s: String) {
    println!("{} ",s);
}


/// 🌟🌟 我们可以使用 String::from 或 to_string 将 &str 转换成 String 类型
// 使用两种方法来解决错误，不要新增代码行
fn question8() {
    let s = "hello, world".to_string();
    // let s1: &str = s; //原题
    let s1: &str = &s; //方法1
    let s2:&str = &s[0..5];//方法2
    let s3:&str = s.as_ref();//方法3
}

/// 字符串转义
fn question9() {
    // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
    // 填空以输出 "I'm writing Rust"
    // let byte_escape = "I'm writing Ru\x73__!"; //原题
    let byte_escape = "I'm writing Ru\x73\x74!"; //方法1
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // 也可以使用 Unicode 形式的转义字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );

    // 还能使用 \ 来连接多行字符串
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

/* 填空并修复所有错误 */
fn question10() {
    // let raw_str = r"Escapes don't work here: \x3F \u{211D}"; //原题
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    // 修改上面的行让代码工作
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // 如果你希望在字符串中使用双引号，可以使用以下形式
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果希望在字符串中使用 # 号，可以如下使用：
    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // 填空
    // let long_delimiter = __; //原题
    let long_delimiter = r###"Hello, "##""###; //方法1
    assert_eq!(long_delimiter, "Hello, \"##\"")
}

use std::str;

/// 想要一个非 UTF-8 形式的字符串吗(我们之前的 str, &str, String 都是 UTF-8 字符串) ? 可以试试字节字符串或者说字节数组:
fn question11() {
    // 注意，这并不是 `&str` 类型了！
    let bytestring: &[u8; 21] = b"this is a byte string";

    // 字节数组没有实现 `Display` 特征，因此只能使用 `Debug` 的方式去打印
    println!("A byte string: {:?}", bytestring);

    // 字节数组也可以使用转义
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但是不支持 unicode 转义
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);


    // raw string
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 将字节数组转成 `str` 类型可能会失败
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // 字节数组可以不是 UTF-8 格式
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

    // 但是它们未必能转换成 `str` 类型
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}


/// 字符串索引
/// 🌟🌟 你无法通过索引的方式去访问字符串中的某个字符，但是可以使用切片的方式 &s1[start..end] ，
/// 但是start 和 end 必须准确落在字符的边界处.
fn question12() {
    let s1 = String::from("hi,中国");
    // let h = s1[0]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
    let h = &s1[0..1];
    assert_eq!(h, "h");

    // let h1 = &s1[3..5];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
    let h1 = &s1[3..6];
    assert_eq!(h1, "中");
}

/// 操作UTF8字符串
fn question13() {
    // 填空，打印出 "你好，世界" 中的每一个字符
    // for c in "你好，世界".__ { //原题
    for c in "你好，世界".chars() { 
        println!("{}", c)
    }
}

/// 我们可以使用三方库 utf8_slice 来访问 UTF-8 字符串的某个子串
/// ，但是与之前不同的是，该库索引的是字符，而不是字节.
use utf8_slice;
fn utf8_slice_test() {
    let s = "The 🚀 goes to the 🌑!";
    let rocket = utf8_slice::slice(s, 4, 5);
    // 结果是 "🚀"
    println!("Rocket: {}", rocket);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question1() {
        // question1();
        // question2();
        // borrow_test();
        // param_method1();
        // param_method2();
        // param_method3();
        // question3();
        // question4();
        // question5();
        // question6();
        // question7();
        // question8();
        // question9();
        // question10();
        // question11();
        // question12();
        // question13();
        utf8_slice_test();
    }
}
