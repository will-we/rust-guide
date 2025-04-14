extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::DeriveInput;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 注意这里使用打印语法树的结构，需要在重新编译生效
    println!("{:#?}", input);
    // 基于 input 构建 AST 语法树
    let ast:DeriveInput = syn::parse(input).unwrap();

    // 构建特征实现代码
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen1 = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen1.into()
}