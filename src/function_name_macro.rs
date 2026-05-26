macro_rules! function_name {
    () => {{
        fn f() {}
        const NAME: &str = stringify!(f);
        &NAME[..NAME.len() - 3]
    }}
}

/// 这个宏永远只会分析f所以他会下标访问无效

pub(crate) use function_name;
