//! Do not use this crate directly.

use proc_macro::{Group, TokenStream, TokenTree};

use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    visit_mut::{self, VisitMut},
    Expr, ExprAwait, Ident, LitStr, Token,
};

// Original bisync attribute macros
// ================================

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
            TokenTree::Literal(..) => {
                new.push(tokens[i].clone());
            }
        }
        i += 1;
    }
    new.into_iter().collect()
}

// Function-like Proc-Macro: suffix
// ====================================

struct SuffixMacroInput {
    suffix_str: LitStr,
    _comma: Token![,],
    expr: Expr,
}

impl Parse for SuffixMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(SuffixMacroInput {
            suffix_str: input.parse()?,
            _comma: input.parse()?,
            expr: input.parse()?,
        })
    }
}

struct AwaitMethodSuffixer<'a> {
    suffix: &'a str,
}

impl<'a> VisitMut for AwaitMethodSuffixer<'a> {
    fn visit_expr_await_mut(&mut self, expr_await: &mut ExprAwait) {
        if let Expr::MethodCall(method_call) = &mut *expr_await.base {
            let original_method_ident = method_call.method.clone();
            let new_method_name_str =
                format!("{}{}", original_method_ident.to_string(), self.suffix);
            method_call.method = Ident::new(&new_method_name_str, original_method_ident.span());
        }
        visit_mut::visit_expr_await_mut(self, expr_await);
    }

    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        visit_mut::visit_expr_mut(self, expr);
    }
}

#[proc_macro]
pub fn suffix(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as SuffixMacroInput);
    let suffix_value = parsed_input.suffix_str.value();

    let mut async_expr_transformed = parsed_input.expr.clone();
    let blocking_expr_original = parsed_input.expr;

    let mut suffixer = AwaitMethodSuffixer {
        suffix: &suffix_value,
    };
    suffixer.visit_expr_mut(&mut async_expr_transformed);

    let output = quote! {
        {
            #[cfg(feature = "async")]
            {
                #async_expr_transformed
            }
            #[cfg(all(feature = "blocking", not(feature = "async")))]
            {
                #blocking_expr_original
            }
        }
    };

    output.into()
}
