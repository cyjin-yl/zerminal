// zerminal_todo 属性宏
// 来源: spec §8.1 — 迁移完成前不允许 cargo build 通过
// 机制: 宏始终展开为 inventory::submit!，build script 统计剩余洞数
// "修好一个洞" = "删掉这个 #[zerminal_todo] 属性"

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, LitStr, Token};

/// 宏参数解析: category (必需), description (可选)
struct ZerminalTodoArgs {
    category: LitStr,
    description: Option<LitStr>,
}

impl Parse for ZerminalTodoArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let category: LitStr = input.parse()?;
        let description = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(input.parse::<LitStr>()?)
        } else {
            None
        };
        Ok(ZerminalTodoArgs {
            category,
            description,
        })
    }
}

/// 标记迁移洞的位置。
///
/// 用法: `#[zerminal_todo("removed-crate", "workspace 不再依赖 project::worktree")]`
///
/// 宏始终展开为 inventory::submit! 注册一个 ZerminalTodo 条目。
/// build script (count_todos 二进制) 收集所有条目并报告数量。
/// 当所有洞都被修复（属性被删除），编译通过。
#[proc_macro_attribute]
pub fn zerminal_todo(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let args: ZerminalTodoArgs = syn::parse_macro_input!(attrs as ZerminalTodoArgs);
    let item: proc_macro2::TokenStream = item.into();
    let category = args.category.value();
    let description = args
        .description
        .map(|description| description.value())
        .unwrap_or_default();
    let file = file!();
    let line = line!();

    let expanded = quote! {
        inventory::submit! {
            zerminal_macros_types::ZerminalTodo {
                category: #category,
                description: #description,
                file: #file,
                line: #line,
            }
        }
        #item
    };

    expanded.into()
}
