//! 1. HashMap 默认使用 SipHash 1-3 哈希算法，该算法对于抵抗 HashDos 攻击非常有效。
//! 在性能方面:
//!     1. 如果你的 key 是中型大小的，那该算法非常不错;
//!     2. 但是如果是小型的 key( 例如整数 )亦或是大型的 key ( 例如字符串 )，那你需要采用社区提供的其它算法来提高性能。
//! 哈希表的算法是基于 Google 的 SwissTable
//!
//! Key 类型选择规则:任何实现了 Eq 和 Hash 特征的类型都可以用于 HashMap 的 key，包括:
//!     1. bool (虽然很少用到，因为它只能表达两种 key)
//!     2. int, uint 以及它们的变体，例如 u8、i32 等
//!     3. String 和 &str (提示: HashMap 的 key 是 String 类型时，你其实可以使用 &str 配合 get 方法进行查询
//!     4. 需要注意的是，f32 和 f64 并没有实现 Hash，原因是 浮点数精度 的问题会导致它们无法进行相等比较。
//!     5. 任何自定义的集合类型，只要其所有字段都实现了 Eq 和 Hash，该集合类型也会自动实现 Eq 和 Hash。
//!         例如 Vect<T> 要实现 Hash，那么首先需要 T 实现 Hash。
//!
//! 容量:
//!     1. HashMap 的容量是指它内部桶的数量，而不是实际存储的键值对数量。
//!     2. 当实际存储的键值对数量接近容量时，HashMap 会自动扩容以保持性能。
//!     3. 可以使用 with_capacity 方法来创建具有指定初始容量的 HashMap，以减少扩容次数。
//!
//! 所有权:
//!     1.对于实现了 Copy 特征的类型，例如 i32，那类型的值会被拷贝到 HashMap 中。
//!     2.而对于有所有权的类型，例如 String，它们的值的所有权将被转移到 HashMap 中。
//!
//! 更换哈希算法:
//!     1. 默认情况下，HashMap 使用 SipHash 1-3 算法。
//!     2. 如果需要更高性能，可以使用社区提供的其它哈希算法，例如 ahash。
// 填空并修复错误
use std::{backtrace::BacktraceStatus::Captured, collections::HashMap};
fn example1() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    // scores.insert("Ashley", 69.0); // 原题错误，类型不匹配
    scores.insert("Ashley", 69);
    // scores.insert("Katie", "58"); // 原题错误，类型不匹配
    scores.insert("Katie", 58);

    // get 返回一个 Option<&V> 枚举值
    let score = scores.get("Sunface");
    // assert_eq!(score, Some(98)); // 原题
    assert_eq!(score, Some(&98)); //使用引用进行比较

    if scores.contains_key("Daniel") {
        // 索引返回一个值 V
        let score = scores["Daniel"];
        // assert_eq!(score, __); //原题
        assert_eq!(score, 95); // Daniel 的分数
        scores.remove("Daniel");
    }

    // assert_eq!(scores.len(),__); // 原题
    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score)
    }
}

fn example2() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // 使用两种方法实现 team_map2
    // 提示:其中一种方法是使用 `collect` 方法
    // let teams_map2...
    let teams_map2 = teams.iter().cloned().collect::<HashMap<&str, i32>>();

    assert_eq!(teams_map1, teams_map2);

    println!("Success!")
}

// 填空
fn example3() {
    // 编译器可以根据后续的使用情况帮我自动推断出 HashMap 的类型，当然你也可以显式地标注类型：HashMap<&str, u8>
    // let mut player_stats = HashMap::new();

    // // 查询指定的 key, 若不存在时，则插入新的 kv 值
    // player_stats.entry("health").or_insert(100);

    // assert_eq!(player_stats["health"], __);

    // // 通过函数来返回新的值
    // player_stats.entry("health").or_insert_with(random_stat_buff);
    // assert_eq!(player_stats["health"], __);

    // let health = player_stats.entry("health").or_insert(50);
    // assert_eq!(health, __);
    // *health -= 50;
    // assert_eq!(*health, __);

    // println!("Success!")
}

fn example3_sln() {
    // 编译器可以根据后续的使用情况帮我自动推断出 HashMap 的类型，当然你也可以显式地标注类型：HashMap<&str, u8>
    let mut player_stats = HashMap::new();

    // 查询指定的 key, 若不存在时，则插入新的 kv 值
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);

    // 通过函数来返回新的值
    player_stats
        .entry("health")
        .or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!")
}

fn random_stat_buff() -> u8 {
    // 为了简单，我们没有使用随机，而是返回一个固定的值
    42
}

// 修复错误
// 提示: `derive` 是实现一些常用特征的好办法
#[derive(PartialEq, Eq, Hash, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn example4() {
    // 使用 HashMap 来存储 viking 的生命值
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // 使用 derive 的方式来打印 viking 的当前状态
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}

/// 容量讨论
/// 1. 使用 with_capacity 初始化 HashMap 可以减少扩容次数，提高性能。
/// 2. 类似于cpp的stl 使用shrink_to 和 shrink_to_fit 方法来控制容量。
fn example5() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    // 事实上，虽然我们使用了 100 容量来初始化，但是 map 的容量很可能会比 100 更多
    assert!(map.capacity() >= 100);

    // 对容量进行收缩，你提供的值仅仅是一个允许的最小值，实际上，Rust 会根据当前存储的数据量进行自动设置，当然，这个值会尽量靠近你提供的值，同时还可能会预留一些调整空间

    let capacity1 = map.capacity();
    map.shrink_to(50);
    let capacity2 = map.capacity();
    assert!(map.capacity() >= 50);

    // 让 Rust  自行调整到一个合适的值，剩余策略同上
    map.shrink_to_fit();
    let capacity3 = map.capacity();
    assert!(map.capacity() >= 2);
    println!("Success!");
    println!("Capacities: {}, {}, {}", capacity1, capacity2, capacity3);
}

// 修复错误，尽可能少的去修改代码
// 不要移除任何代码行！
fn example6() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    // 所有权在这里发生了转移
    m2.insert(v2.clone(), v1);

    assert_eq!(v2, "hello");

    println!("Success!")
}

use std::hash::BuildHasherDefault;
// 引入第三方的哈希函数
use twox_hash::XxHash64;

fn example7() {
    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        example1();
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
    fn test_example5() {
        example5();
    }
    #[test]
    fn test_example6() {
        example6();
    }
    #[test]
    fn test_example7() {
        example7();
    }
}
