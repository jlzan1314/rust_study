//! 过程宏必须定义在独立的 proc-macro crate 中。

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, Lit, Meta};

/// 读取字段名和文档注释，为结构体生成 describe() 方法。
#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    // 编译期：把 token 序列解析成语法树。
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(name, "Describe 只支持具名字段结构体")
                    .to_compile_error()
                    .into()
            }
        },
        _ => {
            return syn::Error::new_spanned(name, "Describe 只支持结构体")
                .to_compile_error()
                .into()
        }
    };

    let descriptions = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap().to_string();
        // /// 注释在这里已经是 #[doc = "..."] 属性。
        let docs: Vec<_> = field
            .attrs
            .iter()
            .filter_map(|attr| {
                if !attr.path().is_ident("doc") {
                    return None;
                }
                if let Meta::NameValue(meta) = &attr.meta {
                    if let Expr::Lit(expr) = &meta.value {
                        if let Lit::Str(value) = &expr.lit {
                            return Some(value.value().trim().to_string());
                        }
                    }
                }
                None
            })
            .collect();
        let description = format!("  {field_name}：{}", docs.join(" "));
        quote! { println!(#description); }
    });
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    // 返回新增的 impl；derive 宏不需要返回原结构体。
    quote! {
        impl #impl_generics #name #type_generics #where_clause {
            pub fn describe() {
                println!("{} 的字段说明（编译期生成）：", stringify!(#name));
                #(#descriptions)*
            }
        }
    }
    .into()
}
