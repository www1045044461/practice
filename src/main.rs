#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// 以上特性仅适用于 nightly Rust，并且用于启用 const 泛型表达式的支持

use core::num;
mod function_name_macro;
mod variable;
mod numbers_practice;
mod char_bool_unit;
mod statements_expressions;
mod function;
mod ownership;
mod complex_type;
mod controlflow;
mod patterns;
mod method_associated_methods;
mod generics_traits;

fn main()
{
    // variable::practice9();
    // numbers_practice::number_practice8();
    numbers_practice::number_practice9_solution();
}

