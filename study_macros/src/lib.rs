use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ItemFn, ItemStruct, LitStr, ReturnType, Type, parse::Nothing, parse_macro_input, parse_quote,
};

/// 在函数体开始执行时向标准输出打印一条日志。
///
/// `#[log_call]` 使用函数名，`#[log_call("自定义标签")]` 使用给定的字符串。
/// 适用于带函数体的非常量函数。
#[proc_macro_attribute]
pub fn log_call(args: TokenStream, item: TokenStream) -> TokenStream {
    // 属性参数和被标注的函数分别由编译器传入。
    let mut function = parse_macro_input!(item as ItemFn);
    let name = &function.sig.ident;
    let label = if args.is_empty() {
        LitStr::new(&name.to_string(), name.span())
    } else {
        parse_macro_input!(args as LitStr)
    };

    if let Some(const_token) = &function.sig.constness {
        return syn::Error::new_spanned(const_token, "#[log_call] 不能用于 const fn")
            .into_compile_error()
            .into();
    }

    // 直接插入语句，保留原函数的签名、作用域和返回逻辑。
    let log = parse_quote! { ::std::println!("[调用] {}", #label); };
    function.block.stmts.insert(0, log);

    quote!(#function).into()
}

/// 在同步函数进入时打印函数名，并在返回前打印结果。
///
/// 返回值需实现 `Debug`。通过 `return` 或 `?` 提前返回时也会执行后置通知。
#[proc_macro_attribute]
pub fn aop(args: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(args as Nothing);
    let mut function = parse_macro_input!(item as ItemFn);

    if let Some(span) = function.sig.asyncness.map(|token| token.span) {
        return syn::Error::new(span, "#[aop] 适用于同步函数")
            .into_compile_error()
            .into();
    }

    if let Some(span) = function.sig.constness.map(|token| token.span) {
        return syn::Error::new(span, "#[aop] 适用于非常量函数")
            .into_compile_error()
            .into();
    }

    let name = &function.sig.ident;
    let body = &function.block;
    // 把函数的返回类型传给闭包，让 return 和 ? 保留原有的类型推断。
    let output = match &function.sig.output {
        ReturnType::Default => quote!(::<()>),
        ReturnType::Type(_, ty) => match ty.as_ref() {
            Type::ImplTrait(_) | Type::Never(_) => quote!(),
            _ => quote!(::<#ty>),
        },
    };

    function.block = parse_quote! {{
        fn __rust_study_aop_run<Output: ::std::fmt::Debug>(
            name: &str,
            body: impl ::std::ops::FnOnce() -> Output,
        ) -> Output {
            ::std::println!("[前置] {}", name);
            let result = body();
            ::std::println!("[后置] {} -> {:?}", name, &result);
            result
        }

        __rust_study_aop_run #output (::std::stringify!(#name), move || #body)
    }};

    quote!(#function).into()
}

/// 校验被标注的项是结构体。
///
/// 标注函数会产生 `expected struct` 编译错误。
///
/// ```compile_fail
/// use study_macros::struct_only;
///
/// #[struct_only]
/// fn add(left: i32, right: i32) -> i32 {
///     left + right
/// }
/// ```
#[proc_macro_attribute]
pub fn struct_only(args: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(args as Nothing);
    let structure = parse_macro_input!(item as ItemStruct);
    quote!(#structure).into()
}
