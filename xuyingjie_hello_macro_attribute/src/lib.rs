use proc_macro::TokenStream;  // Rust 编译器传入/传出的代码流
use syn::{parse_macro_input, ItemFn}; // rust 语言转换成语法树
use quote::quote;  // 语法树转换成rust语言


#[proc_macro_attribute]
pub fn timer_duration(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;

    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident; // 函数名称
    let fn_block =  &input.block; // 函数体
    let fn_sig = &input.sig; // 函数签名


    let expend = quote! {
        #fn_sig {
            let start = std::time::Instant::now();
             let __timer_result = { #fn_block };    
            let duration = start.elapsed();         // 计算耗时
            println!("函数 `{}` 执行耗时: {:?}", stringify!(#fn_name), duration);
            __timer_result
        }
    };
    TokenStream::from(expend)
    
}