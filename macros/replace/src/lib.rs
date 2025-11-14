#![feature(extend_one)]
#![allow(
    clippy::blocks_in_if_conditions,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::manual_find,
    clippy::manual_let_else,
    clippy::manual_map,
    clippy::map_unwrap_or,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::range_plus_one,
    clippy::single_match_else,
    clippy::struct_field_names,
    clippy::too_many_lines,
    clippy::wrong_self_convention
)]

extern crate proc_macro;

use proc_macro::{Group, Ident, TokenStream, TokenTree, token_stream};


#[proc_macro]
pub fn replace_ident(input: TokenStream) -> TokenStream {

    // dbg!(&input);
    let mut it = input.into_iter();

    let target = match it.next() {
        Some(TokenTree::Ident(i)) => i,
        _ => panic!("Expected identifier"),
    };
    let _comma = match it.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => p,
        _ => panic!("Expected identifier"),
    };
    let mut replace = TokenStream::new();

    while let Some(tt) = it.next() {
        match tt {
            TokenTree::Punct(punct) if punct.as_char() == ',' => break,
            other => replace.extend_one(other),
        }
    }

    // Return the remaining tokens, but replace identifiers.
    replace_ident2(target, replace, it)
}

fn replace_ident2(
    target: Ident,
    replace: TokenStream,
    mut input: token_stream::IntoIter,
) -> TokenStream {
    let mut result = TokenStream::new();

    while let Some(tt) = input.next() {
        match tt {
            // Comparing `Ident`s can only be done via string comparison right
            // now. Note that this ignores syntax contexts which can be a
            // problem in some situation.
            TokenTree::Ident(ref i) if i.to_string() == target.to_string() => {
                result.extend(replace.clone())
            }
            TokenTree::Group(group) => result.extend_one(TokenTree::Group(Group::new(
                group.delimiter(),
                replace_ident2(target.clone(), replace.clone(), group.stream().into_iter()),
            ))),
            // All other tokens (puncts and literals) are just forwarded
            other => result.extend_one(other),
        }
    }
    result
}
