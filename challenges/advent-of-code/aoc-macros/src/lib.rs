use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{
    braced, parse_macro_input, token::Group, DeriveInput, Expr, Field, Ident, LitInt, LitStr,
    Result, Token,
};

enum AoCItem {
    Problem(AoCProblem),
}

struct AoCProblem {
    pub year: LitInt,
    pub day: LitInt,
    pub name: LitStr,
    pub input: LitStr,
}

impl Parse for AoCProblem {
    fn parse(input: ParseStream) -> Result<Self> {
        let year: LitInt = input.parse()?;
        let _comma = input.parse::<Token![,]>()?;

        let day: LitInt = input.parse()?;
        let _comma = input.parse::<Token![,]>()?;

        let name: LitStr = input.parse()?;
        let _comma = input.parse::<Token![,]>()?;

        let input: LitStr = input.parse()?;

        Ok(AoCProblem {
            year,
            day,
            name,
            input,
        })
    }
}

#[proc_macro]
pub fn aoc_problem(args: TokenStream) -> TokenStream {
    let group = parse_macro_input!(args as proc_macro2::Group);
    let group_stream = group.stream().into();

    let aoc_problem = parse_macro_input!(group_stream as AoCProblem);

    let name = format!("AoCProblem{}{}", aoc_problem.year, aoc_problem.day);

    let var_name = syn::Ident::new(&name, proc_macro2::Span::call_site());

    let AoCProblem {
        year,
        day,
        name,
        input,
    } = aoc_problem;

    let tokens = quote! {

        struct #var_name {
            year: i32,
            day: i32,
            name: String,
            input: String,
        }

        impl #var_name {
            fn new() -> Self {
                Self {
                    year: #year as i32,
                    day: #day as i32,
                    name: #name.to_string(),
                    input: #input.to_string(),
                }
            }
        }
    };

    proc_macro::TokenStream::from(tokens)
}
