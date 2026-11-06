use core::panic::PanicInfo;
use std::fmt::Error;
use std::panic;
use std::sync::{Arc, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::collections::HashMap;
use std::panic::PanicHookInfo;
//
// panic!(): 它会打印出一条错误信息并打印出栈调用情况，最终结束当前线程:
//  1.主线程: 会导致整个程序终止。
//  2.子线程: 只会终止当前线程，主线程继续执行。
//
// 使用panic处理错误:
//     1. 使用 panic::set_hook() 自定义 panic 处理逻辑。
//     2. 使用 panic::catch_unwind() 捕获 panic 并进行处理。
//     3. 高级设计,子线程出现 panic 不直接打印,主线程通过 join() 捕获,其他线程发生 panic 时仍然正常打印
//     4. 通过 thread::current().id() 判断当前发生 panic 的线程
//
// 默认情况下:
//     默认情况下，栈调用只会展示最基本的信息:
//     例如:
//         thread 'main' panicked at 'I don't like lemonade!', src/panic_result/panics.rs:28:9
// 望获取更详细的信息:(栈展开unwinding or 停止abort)
//     可以通过自定义 panic hook 或者使用 RUST_BACKTRACE 环境变量来获取完整的栈调用信息。
//
// 在Cargo.toml种设置
// ```toml
// [profile.dev]
// panic = "unwind" # 开发版本使用 unwind 以便在 panic 时可以回溯栈信息
//
// [profile.release]
// panic = "abort" #release版本直接停止
// ```
use std::thread;
use std::time;

// 填空
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // 实现下面的代码
        panic!("I don't like lemonade!");
    }
    println!("Exercise Failed if printing out this line!");
}

fn example1() {
    drink("lemonade");
    println!("Exercise Failed if printing out this line!");
}

fn subprocoess(route: i32, delay: u64) -> Result<String, Error> {
    thread::sleep(std::time::Duration::from_secs(delay));
    if route == 0 {
        panic!("Subprocess failed after {} seconds", delay)
    } else {
        Ok("Subprocess succeeded".to_string())
    }
}
/// example2(): 它会创建一个子线程执行 subprocoess 函数，如果子线程 panic，主线程会捕获并打印 "Thread panicked"。
fn example2() {
    let handle = thread::spawn(|| subprocoess(0, 3));
    let result = handle.join();
    if let Ok(result) = result{
        println!("Thread succeeded: {:?}", result);
    } else if let Err(a) = result {
        println!("Thread panicked:{:?}", a);
    }
}


/// implace panic handling in the main thread using std::panic::set_hook and std::panic::catch_unwind.
/// 改进方案1:可以通过 std::panic::set_hook() 覆盖默认的 Panic Hook。
fn example3() {
    // 清空默认的panic hook
    panic::set_hook(Box::new(|_| {
        // println!("Panic occurred: {:?}", handler);
    }));
}

// 修复所有的 panic，让代码工作
fn question2() {
    // assert_eq!("abc".as_bytes(), [96, 97, 98]);
    // let v = vec![1, 2, 3];
    // let ele = v[3];
    // let ele = v.get(3).unwrap();
    // // 大部分时候编译器是可以帮我们提前发现溢出错误，并阻止编译通过。但是也有一些时候，这种溢出问题直到运行期才会出现
    // let v = production_rate_per_hour(2);
    // divide(15, 0);
    // println!("Success!")
}

// 修复所有的 panic，让代码工作
fn answer2() {
    assert_eq!("abc".as_bytes(), [97, 98, 99]);
    let mut v = vec![1, 2, 3];
    v.push(4);
    let ele = v[3];
    let ele = v.get(3).unwrap();
    // 大部分时候编译器是可以帮我们提前发现溢出错误，并阻止编译通过。但是也有一些时候，这种溢出问题直到运行期才会出现
    let v = production_rate_per_hour(2);
    divide(15, 1);
    println!("Success!")
}

fn divide(x:u8, y:u8) {
    println!("{}", x / y)
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u8 = 21; //原值 221
    #[allow(arithmetic_overflow)]
    match speed {
        1..=4 => (speed * cph) as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}


/// 屏蔽指定线程的 panic 信息显示
/// 但上面的实现存在竞态：
///     1. 子线程可能在安装 Hook 之前就发生 panic。而且 Hook 仍然是全局的，不能把它当成真正的线程局部设置。
///     2. 如果采用这种方式，必须在子线程启动执行任务之前完成 Hook 配置，并考虑其他并发任务的影响。
/// 
fn example4()
{
    let _old_hook = panic::take_hook();
    let handle = thread::Builder::new()
        .name("worker".into())
        .spawn(||{
            subprocoess(0, 3)
        }).expect("创建线程失败");
    
    // 注意：此处仅用于解释思路，不可保证避免竞态
    let worker_id = handle.thread().id();
    panic::set_hook(Box::new(move|info|{
        if thread::current().id() == worker_id {
            return;
        }else {
            _old_hook(info); //只处理非 worker 线程的 panic
        }
    }));

    let result = handle.join();
    if let Ok(result) = result {
        println!("Thread succeeded: {:?}", result);
    }else if let Err(a) = result {
        println!("Thread panicked: {:?}", a);
    }
}


/// 实现反思:
///  1. 静态引用: 使用OnceLock或者LazyLock 来初始化静态的 HandlerMap，避免了直接使用静态可变引用带来的安全问题。
///  2. 通过 RwLock 来保护 HandlerMap 的并发访问，保证在多线程环境下的安全性。
///  3. 使用 Arc 包装旧的 panic hook，确保在闭包中可以安全地调用它，避免生命周期问题。
///  4. Rust的 RwLockReadGuard和 RwLockWriteGuard 会在作用域结束时自动释放锁，避免了手动解锁的复杂性,不需要手动 drop和new;
///  5. 通过将线程名称与处理函数映射，允许对不同线程设置不同的 panic 处理逻辑，实现更细粒度的控制。
/// 
/// PanicEventManager 用于管理不同线程的 panic 事件处理函数。
/// 它通过一个 HashMap 将线程名称映射到对应的 panic 处理函数。
struct PanicEventManager
{
}

type PanicHandleTrait = dyn Fn(&PanicHookInfo<'_>) -> () + Send + Sync + 'static;
type PanicHandler = Box<PanicHandleTrait>;
type HandlerMap = RwLock<HashMap<String, PanicHandler> >;

impl PanicEventManager 
{
    /// 误区:这里不需要返回 '&static mut
    fn handlers()->&'static HandlerMap
    {
        static HANDLERS:OnceLock<HandlerMap> = OnceLock::new();
        HANDLERS.get_or_init(||RwLock::new(HashMap::new()))
    }

    /// 注册线程名称对应的 panic 处理函数
    fn register(thread_name: String, handler: PanicHandler) {
        // self.map.insert(thread_name, handler);
        let mut guard = Self::handlers().write().unwrap();
        guard.insert(thread_name, handler);
    }

    fn unregister(thread_name: &str) {
        let mut guard = Self::handlers().write().unwrap();
        guard.remove(thread_name);
    }

    fn Listening()
    {
        let _old = panic::take_hook();
        let _old_ref = Arc::new(_old);
        //如果引用 self 则生命周期在这里会扩张，可能导致 self 无法被释放
        panic::set_hook(
            Box::new(move |info| 
            {
                let thread_name = thread::current().name().unwrap_or("").to_string();
                if let Some(handler) 
                    = Self::handlers().read().unwrap().get(&thread_name) 
                {
                    handler(info); // 处理当前线程的 panic
                }else{
                    _old_ref.clone()(info); // 处理非注册线程的 panic
                }
            })
        );
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        // example1();
        example2();
    }

    #[test]
    fn test_answer2() {
        answer2();
    }

    #[test]
    fn test_replace_hook() 
    {
        example3();
    }

    #[test]
    fn target_thread_spwan_masked() {
        example4();
    }

    #[test]
    fn target_multiple_threads() {
        PanicEventManager::Listening();
        //创建10个线程
        let mut handles = Vec::new();
        for i in 0..10 {
            let thread_name = format!("thread-{}", i);
            PanicEventManager::register(thread_name.clone(), Box::new(|info| {
                let _local_thread = thread::current();
                println!("Thread {:?} panicked: {:?}", _local_thread.name(), info);
            }));

            let handle = std::thread::Builder::new()
                .name(thread_name)
                .spawn(move || {
                    subprocoess(i%2, i as u64)
                })
                .unwrap();
            handles.push(handle);
        }

        for handle in handles {
            let name = &handle.thread().name().unwrap_or("").to_string();
            let result = handle.join();
            if let Err(err) = result {
                println!("Thread {} panicked with error: {:?}", name, err);
            }else if let Ok(ret) = result {
                println!("Thread {} finished successfully with return value: {:?}", name, ret);
            }
        }
        println!("All threads have finished.");
    }
}
