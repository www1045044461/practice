/// 关于使用vec存储多种类型的示例的讨论
/// 方法1:使用枚举存储多种类型的值
/// 优点是可以在一个向量中存储多种类型的值，缺点是每个枚举实例的大小取决于最大变体的大小，可能会浪费内存
/// 方法2:使用Box来约束存储多种类型的值，减少枚举的大小，节省内存
/// 方法3:使用结构体来存储多种类型的值，每种类型使用单独的向量，内存布局更加紧凑
/// 方法4:使用dyn Any进行运行时类型清除，能够在类型不确定的情况下存储多种类型的值，但会带来运行时开销

fn real_length_of_vector<T>(vec: &Vec<T>) -> usize
where
    T: Sized,
{
    let metasize = std::mem::size_of::<T>();
    vec.len() * metasize
}

mod enum_impliment {
    use super::real_length_of_vector;
    /// 表示可以存储多种类型的值的枚举
    enum Value {
        Integer(i32),
        Float(f64),
        Text(String),
    }

    pub fn example(cost: &mut i32) {
        println!("Example of storing multiple types in a vector");
        println!("Single value length:{}", std::mem::size_of::<Value>());

        let mut vec: Vec<Value> = Vec::new();
        let mut vec2: Vec<Value> = Vec::new();
        let mut vec3: Vec<Value> = Vec::new();

        for i in 0..100 {
            vec.push(Value::Integer(i));
        }

        for i in 0..100 {
            vec2.push(Value::Float(i as f64));
        }

        for i in 0..100 {
            vec3.push(Value::Text(i.to_string()));
        }

        let i_len = real_length_of_vector(&vec);
        let f_len = real_length_of_vector(&vec2);
        let t_len = real_length_of_vector(&vec3);

        println!(
            "Integer vector real length: {} {}",
            i_len,
            std::mem::size_of::<i32>() * vec.len()
        );
        println!(
            "Float vector real length: {} {}",
            f_len,
            std::mem::size_of::<f64>() * vec2.len()
        );
        println!(
            "Text vector real length: {} {}",
            t_len,
            std::mem::size_of::<String>() * vec3.len()
        );
        println!("所以不同类型的值在向量中占用的实际内存长度可能不同");
        *cost = t_len as i32;
    }
}

/// 使用Box来约束存储多种类型的值
mod enum_impliment_c1 {
    use super::real_length_of_vector;
    enum Value {
        Integer(i32),
        Float(f64),
        Text(Box<String>), // 使用Box来存储String，减少枚举的大小
    }

    pub fn example(cost: &mut i32) {
        println!("Example of storing multiple types in a vector with Box");
        println!("Single value length:{}", std::mem::size_of::<Value>());
        let mut vec: Vec<Value> = Vec::new();
        let mut vec2: Vec<Value> = Vec::new();
        let mut vec3: Vec<Value> = Vec::new();

        for i in 0..100 {
            vec.push(Value::Integer(i));
        }

        for i in 0..100 {
            vec2.push(Value::Float(i as f64));
        }

        for i in 0..100 {
            vec3.push(Value::Text(Box::new(i.to_string())));
        }

        let i_len = real_length_of_vector(&vec);
        let f_len = real_length_of_vector(&vec2);
        let t_len = real_length_of_vector(&vec3);

        println!(
            "Integer vector real length: {} {}",
            i_len,
            std::mem::size_of::<i32>() * vec.len()
        );
        println!(
            "Float vector real length: {} {}",
            f_len,
            std::mem::size_of::<f64>() * vec2.len()
        );
        println!(
            "Text vector real length: {} {}",
            t_len,
            std::mem::size_of::<Box<String>>() * vec3.len()
        );

        println!("所以不同类型的值在向量中占用的实际内存长度可能不同");
        *cost = t_len as i32;
    }
}

/// 使用结构体来存储多种类型的值，每种类型使用单独的向量
/// 1.好处是每种类型的值都存储在单独的向量中，内存布局更加紧凑，访问效率更高
/// 2.缺点是需要为每种类型单独维护向量，增加了代码复杂度
/// 3.适合在类型已知且固定的情况下使用，能够充分利用Rust的类型系统和内存布局优势
/// 4.不适合在类型不确定或需要动态类型的情况下使用
/// 5.无法线性遍历所有类型的值，只能分别遍历每个向量
mod enum_impliment_c2 {
    /// 用来区分类型的枚举
    /// 并配合vec实现位置记录
    /// 使用i8作为底层表示，减少枚举的大小
    /// 这样每个ValueType只占用1个字节
    /// 适合在order向量中存储大量类型信息的场景
    /// 但是增加了对枚举值的限制，不能超过i8的范围
    #[repr(i8)]
    enum ValueType {
        Integer,
        Float,
        Text,
    }

    struct Values {
        integers: Vec<i32>,
        floats: Vec<f64>,
        strings: Vec<String>,
        //第三步实现:增加order之后反而浪费了空间;
        order: Vec<ValueType>,
    }

    impl Values {
        pub fn new() -> Self {
            Self {
                integers: Vec::new(),
                floats: Vec::new(),
                strings: Vec::new(),
                order: Vec::new(),
            }
        }

        pub fn push_integer(&mut self, value: i32) {
            self.integers.push(value);
            self.order.push(ValueType::Integer);
        }

        pub fn push_float(&mut self, value: f64) {
            self.floats.push(value);
            self.order.push(ValueType::Float);
        }

        pub fn push_string(&mut self, value: String) {
            self.strings.push(value);
            self.order.push(ValueType::Text);
        }

        pub fn len(&self) -> usize {
            let meta_len = std::mem::size_of::<Vec<i32>>()
                + std::mem::size_of::<Vec<f64>>()
                + std::mem::size_of::<Vec<String>>();
            meta_len
                + self.integers.len() * std::mem::size_of::<i32>()
                + self.floats.len() * std::mem::size_of::<f64>()
                + self.strings.len() * std::mem::size_of::<String>()
                + self.order.len() * std::mem::size_of::<ValueType>()
        }

        pub fn len_no_order(&self) -> usize {
            let meta_len = std::mem::size_of::<Vec<i32>>()
                + std::mem::size_of::<Vec<f64>>()
                + std::mem::size_of::<Vec<String>>();
            meta_len
                + self.integers.len() * std::mem::size_of::<i32>()
                + self.floats.len() * std::mem::size_of::<f64>()
                + self.strings.len() * std::mem::size_of::<String>()
        }
    }

    pub fn example(cost: &mut i32) {
        let mut values = Values::new();

        values.push_integer(10);
        values.push_float(3.14);
        values.push_string("hello".to_string());
        values.push_integer(20);

        for i in 0..100 {
            if i % 3 == 0 {
                values.push_integer(i);
            } else if i % 3 == 1 {
                values.push_float(i as f64);
            } else {
                values.push_string(i.to_string());
            }
        }

        // println!("{:?}", values.integers); // [10, 20]
        // println!("{:?}", values.floats); // [3.14]
        // println!("{:?}", values.strings); // ["hello"]
        println!("Total memory length: {}", values.len());
        *cost = values.len() as i32;
    }

    /// 二阶:使用静态分发实现统一访问
    /// 静态分发策略，通过实现InsertValue trait来实现不同类型值的插入
    trait InsertValue {
        fn insert(self, value: &mut Values);
    }

    impl InsertValue for i32 {
        fn insert(self, value: &mut Values) {
            value.integers.push(self);
            value.order.push(ValueType::Integer);
        }
    }

    impl InsertValue for f64 {
        fn insert(self, value: &mut Values) {
            value.floats.push(self);
            value.order.push(ValueType::Float);
        }
    }

    impl InsertValue for String {
        fn insert(self, value: &mut Values) {
            value.strings.push(self);
            value.order.push(ValueType::Text);
        }
    }

    impl Values {
        /// 静态分发策略实现函数，用于将不同类型的值插入到Values结构体中
        fn push<T: InsertValue>(&mut self, value: T) {
            value.insert(self);
        }
    }

    pub fn example_c2(cost: &mut i32) {
        let mut values = Values::new();

        for i in 0..100 {
            if i % 3 == 0 {
                values.push(i);
            } else if i % 3 == 1 {
                values.push(i as f64);
            } else {
                values.push(i.to_string());
            }
        }

        println!("Total memory length: {}", values.len());
        println!(
            "Total memory length without order: {}",
            values.len_no_order()
        );
        *cost = values.len() as i32;
    }
}

mod enum_impliment_c21 {
    use std::vec;

    #[repr(i8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ValueType {
        Integer,
        Float,
        Text,
    }

    /// 将ValueType转换为i8
    impl ValueType {
        fn to_i8(&self) -> i8 {
            match self {
                ValueType::Integer => 0,
                ValueType::Float => 1,
                ValueType::Text => 2,
            }
        }
    }

    /// 将i8转换为ValueType
    impl From<i8> for ValueType {
        fn from(value: i8) -> Self {
            match value {
                0 => ValueType::Integer,
                1 => ValueType::Float,
                2 => ValueType::Text,
                _ => panic!("Invalid value for ValueType"),
            }
        }
    }

    struct Values {
        integers: Vec<i32>,
        floats: Vec<f64>,
        strings: Vec<String>,
        //第三步实现:增加order之后反而浪费了空间;
        order: Vec<i8>,
        /// 用来标记压缩位
        /// 0 ->未压缩
        /// 1 ->占用[b0b1]
        /// 2-> 占用[b2b3]
        /// 3-> 占用[b4b5]
        /// 4-> 占用[b6b7]
        compressed_bit: i8,
    }

    /// 迭代Values结构体中的值类型，按照压缩存储的顺序返回ValueType
    struct ValuesIterator<'a> {
        values: &'a Values,
        stage: usize,
        gindex: usize,
        internal_index: usize,
        float_index: usize,
        text_index: usize,
    }

    impl ValuesIterator<'_> {
        fn new(values: &Values) -> ValuesIterator {
            ValuesIterator {
                values,
                stage: 0,
                gindex: 0,
                internal_index: 0,
                float_index: 0,
                text_index: 0,
            }
        }
    }

    /// 创建一个ValuesIterator实例，用于迭代Values中的值类型
    impl Iterator for ValuesIterator<'_> {
        type Item = (ValueType, usize);

        fn next(&mut self) -> Option<Self::Item> {
            if self.gindex >= self.values.order.len() {
                return None;
            }
            let byte = self.values.order[self.gindex];
            let code = (byte >> (self.stage * 2)) & 0b11;
            self.stage += 1;
            if self.stage == 4 {
                self.stage = 0;
                self.gindex += 1;
            }
            let _t = ValueType::from(code as i8);

            println!(
                "GIndex:{}, Stage:{}, Code:{},InternalIndex:{}, FloatIndex:{}, TextIndex:{}",
                self.gindex,
                self.stage,
                code,
                self.internal_index,
                self.float_index,
                self.text_index
            );

            let mut result = None;
            if ValueType::Integer == _t {
                let index = self.internal_index;
                self.internal_index += 1;
                result = Some((_t, index));
            } else if ValueType::Float == _t {
                let index = self.float_index;
                self.float_index += 1;
                result = Some((_t, index));
            } else if ValueType::Text == _t {
                let index = self.text_index;
                self.text_index += 1;
                result = Some((_t, index));
            }
            // self.global_index += 1;
            result
        }
    }

    impl Values {
        fn new() -> Self {
            Values {
                integers: Vec::new(),
                floats: Vec::new(),
                strings: Vec::new(),
                order: Vec::new(),
                compressed_bit: 0,
            }
        }

        fn len(&self) -> usize {
            let meta_len = std::mem::size_of::<Vec<i32>>()
                +std::mem::size_of::<Vec<f64>>()
                +std::mem::size_of::<Vec<String>>()
                +std::mem::size_of::<Vec<i8>>() // order vector
                +std::mem::size_of::<i8>();

            meta_len
                + self.integers.len() * std::mem::size_of::<i32>()
                + self.floats.len() * std::mem::size_of::<f64>()
                + self.strings.len() * std::mem::size_of::<String>()
                + self.order.len() * std::mem::size_of::<i8>()
        }

        /// 将ValueType压缩存储到order中
        /// 每个i8可以存储4个ValueType，每个ValueType占用2位
        /// 自己的实现不太好
        // fn order_push(&mut self, value_type: ValueType) {
        //     let _cbit = value_type.to_i8();
        //     let _bit = self.order.last();
        //     let mut value = match _bit {
        //         Some(&last) => last,
        //         None => 0,
        //     };

        //     if self.compressed_bit != 4 {
        //         let old_value = self.order.last_mut().unwrap();
        //         if self.compressed_bit == 0 {
        //             value = _cbit & value;
        //         } else if self.compressed_bit == 1 {
        //             value = (_cbit << 2) & value;
        //         } else if self.compressed_bit == 2 {
        //             value = (_cbit << 4) & value;
        //         } else if self.compressed_bit == 3 {
        //             value = (_cbit << 6) & value;
        //         }
        //         *old_value = value;
        //     } else {
        //         self.order.push(_cbit);
        //     }
        //     self.compressed_bit = (self.compressed_bit + 1) % 4;
        // }

        /// 将ValueType压缩存储到order中
        /// 比之前的实现更高效，避免了频繁的位运算和条件判断
        fn order_push(&mut self, value_type: ValueType) {
            let code = value_type.to_i8();
            if self.compressed_bit == 0 {
                self.order.push(code);
            } else {
                let last_index = self.order.len() - 1;
                self.order[last_index] |= code << (self.compressed_bit * 2);
            }
            self.compressed_bit = (self.compressed_bit + 1) % 4;
        }
    }

    trait InsertValue {
        fn insert(self, value: &mut Values);
    }

    impl InsertValue for i32 {
        fn insert(self, value: &mut Values) {
            value.integers.push(self);
            value.order_push(ValueType::Integer);
        }
    }

    impl InsertValue for f64 {
        fn insert(self, value: &mut Values) {
            value.floats.push(self);
            value.order_push(ValueType::Float);
        }
    }

    impl InsertValue for String {
        fn insert(self, value: &mut Values) {
            value.strings.push(self);
            value.order_push(ValueType::Text);
        }
    }

    pub fn example(cost: &mut usize) {
        let mut values = Values::new();

        for i in 0..100 {
            if i % 3 == 0 {
                i.insert(&mut values);
            } else if i % 3 == 1 {
                (i as f64).insert(&mut values);
            } else {
                format!("String {}", i).insert(&mut values);
            }
        }

        for value_type in ValuesIterator::new(&values) {
            // println!("{:?}", value_type);
            if let ValueType::Integer = value_type.0 {
                if let Some(&last) = values.integers.get(value_type.1) {
                    println!("Integer value: {}", last);
                }
            } else if let ValueType::Float = value_type.0 {
                if let Some(&last) = values.floats.get(value_type.1) {
                    println!("Float value: {}", last);
                }
            } else if let ValueType::Text = value_type.0 {
                if let Some(last) = values.strings.get(value_type.1) {
                    println!("Text value: {}", last);
                }
            }
        }
        *cost = values.len();
    }
}

/// 方法3:使用dyn Any进行运行时类型清除
mod enum_impliment_c3 {
    use std::any::Any;

    fn len(values: &Vec<Box<dyn Any>>) -> usize {
        values.len() * std::mem::size_of::<Box<dyn Any>>()
    }

    pub fn example(cost: &mut usize) {
        let mut values: Vec<Box<dyn Any>> = Vec::new();

        for i in 0..100 {
            if i % 3 == 0 {
                values.push(Box::new(i));
            } else if i % 3 == 1 {
                values.push(Box::new(i as f64));
            } else {
                values.push(Box::new(format!("String {}", i)));
            }
        }

        *cost = len(&values);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example() {
        let mut pre_len = 0;
        let mut afr_len = 0;
        super::enum_impliment::example(&mut pre_len);
        println!("使用Box存储String的向量示例");
        super::enum_impliment_c1::example(&mut afr_len);
        println!("方法占比{}-->{}", pre_len, afr_len);
    }

    #[test]
    fn test_example_c2() {
        let mut pre = 0;
        super::enum_impliment::example(&mut pre);
        println!("使用结构体存储多种类型的值示例");
        let mut afr = 0;
        super::enum_impliment_c2::example(&mut afr);
        println!("方法占比{}-->{}", pre, afr);
        //比起原始方法节省了近一半空间;

        let mut _pre1 = 0;
        let mut _afr1 = 0;
        println!("使用静态分发策略的示例完成");
        super::enum_impliment_c2::example_c2(&mut _pre1);
        println!("静态分发策略示例完成-->附加order信息的逆向遍历");
        super::enum_impliment_c21::example(&mut _afr1);
        println!(
            "静态分发策略示例完成-->附加order信息的逆向遍历结果: {}-->{}",
            _pre1, _afr1
        );
    }

    #[test]
    fn test_example_c3() {
        let mut pre_len = 0;
        let mut afr_len = 0;
        super::enum_impliment::example(&mut pre_len);
        super::enum_impliment_c3::example(&mut afr_len);
        println!("方法占比{}-->{}", pre_len, afr_len);
    }
}
