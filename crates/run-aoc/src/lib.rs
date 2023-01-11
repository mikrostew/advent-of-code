extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Span, TokenTree};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

// create a runner function to wrap the input function and Display its output
#[proc_macro_attribute]
pub fn runner_fn(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let original_fn = parse_macro_input!(input as ItemFn);
    // this contains the function name, and params
    let signature = original_fn.sig.clone();

    // function name
    let ident = signature.ident;
    let ident_name = format!("{}", ident);

    let runner_name = if ident_name == "part1" || ident_name == "part2" {
        Ident::new(&format!("__{}_runner", ident_name), Span::call_site())
    } else {
        return syn::Error::new(
            ident.span(),
            "runner function must be named 'part1' or 'part2'",
        )
        .to_compile_error()
        .into();
    };

    let num_args = signature.inputs.len();

    match num_args {
        1 => TokenStream::from(quote!(
            #original_fn

            // only takes one arg, doesn't expect params
            pub fn #runner_name(file_contents: String, _p: Option<std::collections::HashMap<String, String>>) -> String {
                let result = #ident(file_contents);
                format!("{}", result)
            }
        )),
        2 => TokenStream::from(quote!(
            #original_fn

            pub fn #runner_name(file_contents: String, p: Option<std::collections::HashMap<String, String>>) -> String {
                let result = #ident(file_contents, p);
                format!("{}", result)
            }
        )),
        _ => syn::Error::new(
            signature.paren_token.span,
            format!(
                "runner functions take 1 or 2 arguments, found {}",
                signature.inputs.len()
            ),
        )
        .to_compile_error()
        .into(),
    }
}

// generate a test function, for making sure things work when I do refactorings and such
#[proc_macro]
pub fn test_fn(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    // collect and remove the punctuation, doesn't matter
    let tt: Vec<TokenTree> = input
        .into_iter()
        .filter(|tt| match tt {
            TokenTree::Punct(_) => false,
            _ => true,
        })
        .collect();

    if tt.len() == 4 {
        // no params
        // ex: aoc_test!(day1, part1, example, 24000);
        match (&tt[0], &tt[1], &tt[2], &tt[3]) {
            (
                TokenTree::Ident(day),
                TokenTree::Ident(part),
                TokenTree::Ident(variation),
                TokenTree::Literal(expected),
            ) => {
                let test_name = Ident::new(&format!("{}_{}", part, variation), Span::call_site());
                let file = format!("inputs/{}-{}.txt", day, variation);
                let file_name = Literal::string(&file);
                let fail_literal = Literal::string(&format!("failed to read file '{}'", file));
                let part_fn = Ident::new(&format!("{}", part), Span::call_site());
                TokenStream::from(quote!(
                    #[test]
                    fn #test_name() {
                        let file = #file_name;
                        let input = std::fs::read_to_string(&file).expect(#fail_literal);
                        assert_eq!(super::#part_fn(input), #expected);
                    }
                ))
            }
            _ => syn::Error::new(
                Span::call_site(),
                "expected args of type (ident, ident, ident, literal), found something else",
            )
            .to_compile_error()
            .into(),
        }
    } else if tt.len() == 5 {
        // with params
        match (&tt[0], &tt[1], &tt[2], &tt[3], &tt[4]) {
            (
                TokenTree::Ident(day),
                TokenTree::Ident(part),
                TokenTree::Ident(variation),
                TokenTree::Literal(params),
                TokenTree::Literal(expected)
            ) => {
                let test_name = Ident::new(
                    &format!("{}_{}", part, variation),
                    Span::call_site(),
                );
                let file = format!("inputs/{}-{}.txt", day, variation);
                let file_name = Literal::string(&file);
                let fail_literal = Literal::string(&format!(
                    "failed to read file '{}'",
                    file
                ));
                let part_fn = Ident::new(&format!("{}", part), Span::call_site());

                TokenStream::from(quote!(
                    #[test]
                    fn #test_name() {
                        let file = #file_name;
                        let params = crate::parse_params(#params);
                        let input = std::fs::read_to_string(&file).expect(#fail_literal);
                        assert_eq!(super::#part_fn(input, Some(params)), #expected);
                    }
                ))
            },
            _ => syn::Error::new(
                Span::call_site(),
                "expected args of type (ident, ident, ident, literal, literal), found something else",
            )
                .to_compile_error()
                .into()
        }
    } else {
        syn::Error::new(
            Span::call_site(),
            format!("this macro takes 4 or 5 arguments, found {}", tt.len()),
        )
        .to_compile_error()
        .into()
    }
}
