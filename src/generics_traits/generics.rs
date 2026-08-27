use std::hint::select_unpredictable;

// 填空
struct A; // 具体的类型 `A`.
struct S(A); // 具体的类型 `S`.
struct SGen<T>(T); // 泛型 `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {
    //获取T的类型名并打印
    println!("Type of T is: {}", std::any::type_name::<T>());
}

// 实现下面的泛型函数 sum
/*fn sum
fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
}
*/
/// 2. 支持以上三种类型的加法操作
fn sum<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    println!("调用方法参数类型；{:?}", std::any::type_name::<T>());
    a + b
}

/*
// 3.实现一个结构体 Point 让代码工作
fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
*/
#[derive(Debug)]
struct Point<T>
where
    T: std::fmt::Debug,
{
    x: T,
    y: T,
}

/*
// 4.修改以下结构体让代码工作
struct Point2<T> {
    x: T,
    y: T,
}
fn main() {
    // 不要修改这行代码！
    let p = Point2{x: 5, y : "hello".to_string()};
}*/
#[derive(Debug)]
struct Point2<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    x: T,
    y: U,
}

/*
// 5.为 Val 增加泛型参数，不要修改 `main` 中的代码
struct Val {
    val: f64,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
} */
struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

/*
5.实现 mixup，不要修改其它代码
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // 实现 mixup，不要修改其它代码！
    fn mixup
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
} */
impl<T, U> Point2<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    /// 使用Move语义实现组合
    /// 关键点:两个Point对象的两个值类型不见得一致,需要引入V，W两个类型
    pub fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W>
    where
        V: std::fmt::Debug,
        W: std::fmt::Debug,
    {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

/*
7.修复错误，让代码工作
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point{x: 5, y: 10};
    println!("{}",p.distance_from_origin())
} */
impl Point<f32>  
{
    fn distance_from_origin(&self) -> f32
    {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generics() {
        /* 原题
        // 使用非泛型函数
        reg_fn(__); // 具体的类型
        gen_spec_t(__); // 隐式地指定类型参数  `A`.
        gen_spec_i32(__); // 隐式地指定类型参数`i32`.

        // 显式地指定类型参数 `char`
        generic::<char>(__);

        // 隐式地指定类型参数 `char`.
        generic(__);
        */

        //这种类型没有字段，所以“构造它”时不需要写花括号或参数，直接写类型名本身就已经是一个值了，也就是：
        let s_obj = S(A);
        reg_fn(s_obj);

        /// 方法参数为固定类型只不过参数类型是一个固定类型的泛型
        let s_gen_obj = SGen(A {});
        gen_spec_t(s_gen_obj);

        let s_gen_obj2 = SGen(42);
        gen_spec_i32(s_gen_obj2);

        let s_gen_obj3 = SGen('c');
        let s_gen_obj4 = SGen("dwdw");
        generic(s_gen_obj4);
        generic(s_gen_obj3);
    }

    #[test]
    pub fn test_sum() {
        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));
    }

    #[test]
    pub fn test_point() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        dbg!(integer);
        dbg!(float);
    }

    #[test]
    pub fn test_point2() {
        let p = Point2 {
            x: 5,
            y: "hello".to_string(),
        };
        dbg!(p);
    }

    #[test]
    pub fn test_val() {
        let x = Val { val: 3.0 };
        let y = Val {
            val: "hello".to_string(),
        };
        dbg!(x.value());
        dbg!(y.value());
    }

    #[test]
    pub fn test_mixup() {
        let p1 = Point2 { x: 13, y: "123" };
        let p2 = Point2 { x: 13.4, y: 11 };
        let p3 = p1.mixup(p2);

        dbg!(p3);
    }

    #[test]
    pub fn test_distance() {
        let p = Point { x: 5.0, y: 10.0 };
        println!("{}", p.distance_from_origin())
    }
}
