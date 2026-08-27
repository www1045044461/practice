///方法和关联函数的实例
/// 单个实例直接在里面实现
use std::ops::{Add, Sub};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    fn new(x: f64, y: f64) -> Self {
        Vector { x, y }
    }

    /// 计算向量的模
    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// 缩放向量
    fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }

    /// 内积
    fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// 外积
    fn cross(&self, other: &Vector) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// 归一化
    fn normalize(&mut self) -> &mut Self {
        let mag = self.magnitude();
        if mag != 0.0 {
            self.x /= mag;
            self.y /= mag;
        }
        self
    }

    /// 旋转向量
    fn rotate(&self, angle: f64) -> Vector {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        let new_x = self.x * cos_angle - self.y * sin_angle;
        let new_y = self.x * sin_angle + self.y * cos_angle;
        Vector::new(new_x, new_y)
    }

    /// 绕指定轴旋转向量
    /// # 参数
    /// - `axis`: 旋转轴向量
    /// - `angle`: 旋转角度（弧度）
    ///
    /// # 返回值
    /// 返回旋转后的向量
    fn axis_rotate(&self, axis: &Point, angle: f64) -> Vector {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let cross_product = self.cross(&Vector::new(axis.x, axis.y));
        let new_x = self.x * cos_angle - self.y * sin_angle
            + axis.x * (1.0 - cos_angle)
            + axis.y * sin_angle;
        let new_y = self.x * sin_angle + self.y * cos_angle - axis.x * sin_angle
            + axis.y * (1.0 - cos_angle);

        Vector::new(new_x, new_y)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// 关联函数的实现
impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    fn origin() -> Self {
        Point { x: 0f64, y: 0f64 }
    }
}

/// 5. 🌟 每一个结构体允许拥有多个 impl 语句块I
impl Point {
    /// 移动方法
    fn moving(&mut self, x: f64, y: f64) -> Self {
        self.x += x;
        self.y += y;
        Point {
            x: self.x,
            y: self.y,
        }
    }

    /// 距离方法
    fn distance(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    /// 平移方法
    /// # 参数
    /// - `x`: x方向的平移量
    /// - `y`: y方向的平移量
    fn translate(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    /// 旋转方法
    /// # 参数
    /// - `angle`: 旋转角度（弧度）
    fn rotate(&mut self, angle: f64) {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let new_x = self.x * cos_angle - self.y * sin_angle;
        let new_y = self.x * sin_angle + self.y * cos_angle;

        self.x = new_x;
        self.y = new_y;
    }

    /// 返回旋转后的新点，不修改原点
    /// # 参数
    /// - `angle`: 旋转角度（弧度）
    fn rotate_new(&self, angle: f64) -> Point {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let new_x = self.x * cos_angle - self.y * sin_angle;
        let new_y = self.x * sin_angle + self.y * cos_angle;
        Point { x: new_x, y: new_y }
    }

    /// 绕指定点旋转方法
    /// # 参数
    /// - `axis`: 旋转中心点
    /// - `angle`: 旋转角度（弧度）
    fn rotate_around(&mut self, axis: &Point, angle: f64) {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let translated_x = self.x - axis.x;
        let translated_y = self.y - axis.y;

        let new_x = translated_x * cos_angle - translated_y * sin_angle + axis.x;
        let new_y = translated_x * sin_angle + translated_y * cos_angle + axis.y;

        self.x = new_x;
        self.y = new_y;
    }

    fn rotate_around_new(&self, axis: &Point, angle: f64) -> Point {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let translated_x = self.x - axis.x;
        let translated_y = self.y - axis.y;

        let new_x = translated_x * cos_angle - translated_y * sin_angle + axis.x;
        let new_y = translated_x * sin_angle + translated_y * cos_angle + axis.y;

        Point { x: new_x, y: new_y }
    }
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new((self.x - rhs.x) as f64, (self.y - rhs.y) as f64)
    }
}

struct Rectangle {
    lt: Point,
    rb: Point,
}

impl Rectangle {
    /// 面积
    fn area(&self) -> u32 {
        let _w = self.rb.x - self.lt.x;
        let _h = self.lt.y - self.rb.y;
        (_w * _h) as u32
    }

    /// 周长
    fn perimeter(&self) -> u32 {
        let _w = self.rb.x - self.lt.x;
        let _h = self.lt.y - self.rb.y;
        2 * (_w + _h) as u32
    }

    /// 移动
    fn translate(&mut self, x: f64, y: f64) {
        self.lt.moving(x, y);
        self.rb.moving(x, y);
    }

    /// 判断点是否在矩形内
    fn contains(&self, p: &Point) -> bool {
        p.x >= self.lt.x && p.x <= self.rb.x && p.y <= self.lt.y && p.y >= self.rb.y
    }

    /// 缩放
    fn scale(&mut self, factor: f64) {
        let center_x = (self.lt.x + self.rb.x) as f64 / 2.0;
        let center_y = (self.lt.y + self.rb.y) as f64 / 2.0;

        let new_lt_x = ((self.lt.x as f64 - center_x) * factor + center_x).round() as f64;
        let new_lt_y = ((self.lt.y as f64 - center_y) * factor + center_y).round() as f64;
        let new_rb_x = ((self.rb.x as f64 - center_x) * factor + center_x).round() as f64;
        let new_rb_y = ((self.rb.y as f64 - center_y) * factor + center_y).round() as f64;

        self.lt.x = new_lt_x;
        self.lt.y = new_lt_y;
        self.rb.x = new_rb_x;
        self.rb.y = new_rb_y;
    }

    // 旋转
    // @param axis 旋转轴
    // @param angle 旋转角度，单位为弧度
    // fn rotate(&mut self,axis:Point,angle:f32) -> Rectangle
    // {
    //     let mut vec_lt = Vector::new((self.lt.x - axis.x) as f64, (self.lt.y - axis.y) as f64);
    //     let mut vec_rb = Vector::new((self.rb.x - axis.x) as f64, (self.rb.y - axis.y) as f64);

    //     let cos_angle = angle.cos() as f64;
    //     let sin_angle = angle.sin() as f64;
    // }
}

struct TrafficLight {
    color: String,
}

/// 3.🌟🌟 &self 实际上是 self: &Self 的缩写或者说语法糖
/// 2.🌟🌟 self 会拿走当前结构体实例(调用对象)的所有权，而 &self 却只会借用一个不可变引用，&mut self 会借用一个可变引用
impl TrafficLight {
    // 原题
    // 使用 `Self` 填空
    // pub fn show_state(__)  {
    //     println!("the current state is {}", self.color);
    // }
    pub fn show_state(&self) {
        println!("the current state is {}", self.color);
    }

    // 填空，不要使用 `Self` 或其变体
    // pub fn change_state(__) {
    //     self.color = "green".to_string()
    // }
    pub fn change_state(&mut self) {
        self.color = "green".to_string();
    }

    /// 🌟🌟 定义在 impl 语句块中的函数被称为关联函数，因为它们跟当前类型关联在一起。
    /// 关联函数与方法最大的区别就是它第一个参数不是 self ，
    /// 原因是它们不需要使用当前的实例，因此关联函数往往可以用于构造函数：初始化一个实例对象。
    pub fn new(color: &str) -> Self {
        Self {
            color: color.to_string(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// 为 TrafficLightColor 实现所需的方法
/// 6.🌟🌟🌟 我们还可以为枚举类型定义方法
impl TrafficLightColor {
    pub fn color(&self) -> &str {
        match self {
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
            TrafficLightColor::Green => "green",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    fn approx_eq(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() < tolerance,
            "left={left}, right={right}, tolerance={tolerance}"
        );
    }

    #[test]
    fn test_vector_rotate() {
        let v = Vector::new(1.0, 0.0);
        println!("Original vector: {:?}", v);
        let rotated_v = v.rotate(std::f64::consts::PI / 2.0);
        println!("Rotated vector: {:?}", rotated_v);

        approx_eq(rotated_v.x, 0.0, 1e-10);
        approx_eq(rotated_v.y, 1.0, 1e-10);
    }

    #[test]
    fn test_point_rotate_around() {
        let p = Point::new(1.2, 1.5);
        let axis = Point::new(0.5, 2.0);
        // let p2 = p.rotate_around_new(&axis,  (std::f64::consts::PI*0.5) as f64);
        let p2 = p.rotate_around_new(&axis, (std::f64::consts::PI * 0.5) as f64);
        println!("Rotated point: {:?}", p2);

        let axis2 = Point::new(axis.x, axis.y);
        let v1 = p - axis;
        let v2 = p2 - axis2;
        let v3 = v1.dot(&v2);

        dbg!(v3);
        dbg!(v1);
        dbg!(v2);
        assert!(v3.abs() < 1e-10, "v3 is not close to zero: {}", v3);
    }

    #[test]
    fn test_rect_area() {
        let rect1 = Rectangle {
            lt: Point { x: 0.0, y: 50.0 },
            rb: Point { x: 30.0, y: 0.0 },
        };

        /// 🌟🌟 方法跟函数类似：都是使用 fn 声明，有参数和返回值。
        /// 但是与函数不同的是，方法定义在结构体的上下文中(枚举、特征对象也可以定义方法)，
        /// 而且方法的第一个参数一定是 self 或其变体 &self 、&mut self，self 代表了当前调用的结构体实例。
        println!("矩形面积: {}", rect1.area());
        assert_eq!(rect1.area(), 1500);
    }
    #[test]
    fn test_enum_method() {
        let c = TrafficLightColor::Yellow;
        assert_eq!(c.color(), "yellow");
        println!("{:?}", c);
    }
}
