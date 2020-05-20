//! Initializes an array with constant values calculated by a `const fn`
//!
//! # Examples
//!
//! ```
//! use array_const_fn_init::array_const_fn_init;
//!
//! const fn const_double_it(i: usize) -> usize {
//!     i * 2
//! }
//! const ARRAY: [usize; 10] = array_const_fn_init![const_double_it; 10];
//! assert_eq!(ARRAY, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
//! ```
//!
//! ```
//! use array_const_fn_init::array_const_fn_init;
//!
//! const fn const_vecs(i: usize) -> (u8, u8, u8) {
//!     (i as u8, i as u8, i as u8)
//! }
//! const ARRAY: [(u8, u8, u8); 4] = array_const_fn_init![const_vecs; 4];
//! assert_eq!(ARRAY, [(0, 0, 0), (1, 1, 1), (2, 2, 2), (3, 3, 3)]);
//! ```
extern crate proc_macro;

use core::iter;
use core::str::FromStr;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn array_const_fn_init(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut iter = input.into_iter();

    let func_name = match iter.next() {
        Some(proc_macro::TokenTree::Ident(i)) => i.to_string(),
        Some(other) => panic!("Expected function name, found {}", other),
        None => panic!("Unexpected end of macro input"),
    };

    match iter.next() {
        Some(proc_macro::TokenTree::Punct(ref p)) if p.as_char() == ';' => {}
        Some(other) => panic!("Expected ';', found {}", other),
        None => panic!("Unexpected end of macro input"),
    };

    let array_size: usize = match iter.next() {
        Some(proc_macro::TokenTree::Literal(ref p)) => {
            usize::from_str(&p.to_string()).expect("Expected <usize>")
        }
        Some(other) => panic!("Expected <usize>, found {}", other),
        None => panic!("Unexpected end of macro input"),
    };

    match iter.next() {
        None => {}
        Some(_) => panic!("Unexpected trailing tokens in macro"),
    }

    let mut ts = TokenStream::new();
    let span: Span = Span::call_site();
    ts.extend({
        let mut g = Group::new(Delimiter::Bracket, {
            let mut ts = TokenStream::new();
            (0..array_size).for_each(|i| {
                ts.extend(iter::once(TokenTree::from(Ident::new(&func_name, span))));
                ts.extend({
                    let mut g = Group::new(Delimiter::Parenthesis, {
                        let mut ts = TokenStream::new();
                        let _span: Span = span;
                        ts.extend(iter::once(TokenTree::from(Literal::usize_suffixed(i))));
                        ts
                    });
                    g.set_span(span);
                    Some(TokenTree::from(g))
                });
                ts.extend(iter::once(TokenTree::from(Punct::new(',', Spacing::Alone))));
            });
            ts
        });
        g.set_span(span);
        Some(TokenTree::from(g))
    });
    ts
}
