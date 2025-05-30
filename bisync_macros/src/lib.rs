//! Do not use this crate directly.

use proc_macro::{Group, TokenStream};
use quote::quote;
use syn::{parse_macro_input, visit_mut::VisitMut, Expr, ItemFn};

#[proc_macro_attribute]
pub fn internal_noop(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn internal_delete(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn internal_strip_async(attr: TokenStream, item: TokenStream) -> TokenStream {
    use proc_macro::TokenTree;

    let mut new = Vec::new();
    let tokens: Vec<TokenTree> = item.into_iter().collect();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            TokenTree::Group(group) => {
                let new_inner = internal_strip_async(attr.clone(), group.stream());
                let mut new_group = Group::new(group.delimiter(), new_inner);
                new_group.set_span(group.span());
                new.push(TokenTree::Group(new_group));
            }
            TokenTree::Ident(ident) => {
                if ident.to_string() == "async" {
                    if i + 1 < tokens.len() && tokens[i + 1].to_string() == "move" {
                        i += 1;
                    }
                } else {
                    new.push(tokens[i].clone());
                }
            }
            TokenTree::Punct(punct) => {
                if punct.as_char() == '.'
                    && i + 1 < tokens.len()
                    && tokens[i + 1].to_string() == "await"
                {
                    i += 2;
                    continue;
                } else {
                    new.push(tokens[i].clone());
                }
            }
            TokenTree::Literal(..) => new.push(tokens[i].clone()),
        }
        i += 1;
    }

    new.into_iter().collect()
}

struct MethodTransformer {
    suffix: String,
}

impl VisitMut for MethodTransformer {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Await(await_expr) = expr {
            if let Expr::MethodCall(method_call) = &mut *await_expr.base {
                let new_method_name = format!("{}_{}", method_call.method, self.suffix);
                method_call.method = syn::Ident::new(&new_method_name, method_call.method.span());
            }
        }
        syn::visit_mut::visit_expr_mut(self, expr);
    }
}

#[proc_macro_attribute]
pub fn internal_transform_async(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);
    let attr_str = attr.to_string();
    let suffix = if attr_str.contains("async_suffix") {
        let parts: Vec<&str> = attr_str.split('=').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            parts[1].trim_matches('"').to_string()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };

    if !suffix.is_empty() {
        let mut transformer = MethodTransformer { suffix };
        transformer.visit_item_fn_mut(&mut input);
    }

    let output = quote! {
        #input
    };
    output.into()
}
