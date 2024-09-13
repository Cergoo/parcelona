use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, Expr, LitInt};

/// helper function
/// example:
/// call:    fn_alt_body(3);
/// result:  "self.0.parse(input).or(self.1.parse(input)).or(self.2.parse(input))  
fn fn_alt_body(i: u8) -> Vec<u8> {
    let mut head: Vec<u8> = "self.0.parse(input)".into();
    let part1: Vec<u8> = ".or(self.".into();
    let part2: Vec<u8> = ".parse(input))".into();
    for n in 1..i {
        head.extend(&part1);
        head.append(&mut n.to_string().into());
        head.extend(&part2);
    }
    head
}

/*
// This is what we want to get using macro
impl<'a,I:'a,O,P1,P2> Alt<I,O> for (P1, P2)
where
    P1: Parser<'a,I,O>,
    P2: Parser<'a,I,O>,
{
    fn choice(&self, input: Ip) -> ParseResult<'a,I,O> {
        self.0.parse(input).or(self.1.parse(input))
    }
}
*/
/// alt_impl!(90); max val 255 elemets tuple (A, B, ...)
#[proc_macro]
pub fn alt_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let i = parse_macro_input!(input as LitInt);
    let value_max = i.base10_parse::<u8>().unwrap()+1;
    if value_max < 4 { return Default::default(); }
    let mut gen = TokenStream::new();

    let for_vec_origin: Vec<Ident> = (0..=value_max)
        .into_iter()
        .map(|a| Ident::new(&("P".to_owned()+&a.to_string()), Span::call_site()))
        .collect();

    for value in 4..value_max {
        let body_str = String::from_utf8(fn_alt_body(value)).unwrap();
        let body_part = syn::parse_str::<Expr>(&body_str).unwrap();

        let for_vec = &for_vec_origin[..value.into()];

        let gen_part = quote! {
                  impl<'a,I:'a,O, #(#for_vec),*> Alt<'a,I,O> for (#(#for_vec),*)
                  where
                        #(#for_vec: Parser<'a,I,O>),*
                  {
                    fn choice(&self, input: &'a[I]) -> ParseResult<'a,I,O> {
                          #body_part
                    }
                  }
        };

        gen.append_all(gen_part);
    }

    //-shugar-// gen.into()
    proc_macro::TokenStream::from(gen)
}