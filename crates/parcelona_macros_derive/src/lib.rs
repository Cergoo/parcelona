use proc_macro2::{Ident, Span, TokenStream, Literal};
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
    if value_max < 3 { return Default::default(); }
    let mut gen = TokenStream::new();

    let for_vec_origin: Vec<Ident> = (0..=value_max)
        .into_iter()
        .map(|a| Ident::new(&("P".to_owned()+&a.to_string()), Span::call_site()))
        .collect();

    for value in 3..value_max {
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



/*
// This is what we want to get using macro
impl<'a,I:'a,P0,O0,P1,O1,> Permut<'a, I, (bool, (Option<O0>, Option<O1>)), (O0, O1)> for (P0, P1)
    where
        P0: Parser<'a, I, O0>,
        P1: Parser<'a, I, O1>,
    {
        fn permutation_part(
            &self,
            input: &'a [I],
        ) -> ParseResult<'a, I, (bool, (Option<O0>, Option<O1>))> {
            let mut v: Vec<usize> = Vec::with_capacity(2);
            for i in 0..2 {
                v.push(i);
            }
            let mut r_tuple = (None::<O0>, None::<O1>);
            let mut count: usize = 0;
            let mut count_old: usize = 0;
            let mut new_input = input;
            let mut er: PErr<'a, I> = Default::default();
            loop {
                if v[0] == 0 {
                    match self.0.parse(new_input) {
                        Ok((inp, r)) => {
                            r_tuple.0 = Some(r);
                            v[0] = usize::MAX;
                            new_input = inp;
                            count += 1;
                        }
                        Err(e) => er = e,
                    }
                }
                if v[1] == 1 {
                    match self.1.parse(new_input) {
                        Ok((inp, r)) => {
                            r_tuple.1 = Some(r);
                            v[1] = usize::MAX;
                            new_input = inp;
                            count += 1;
                        }
                        Err(e) => er = e,
                    }
                }
                if count == count_old {
                    break;
                }
                count_old = count;
            }
            match count {
                c if c == 2 => Ok((new_input, (true, r_tuple))),
                c if c > 0 => Ok((new_input, (false, r_tuple))),
                _ => Err(er),
            }
        }

        fn permutation(&self, input: &'a [I]) -> ParseResult<'a, I, (O0, O1)> {
            let mut v: Vec<usize> = Vec::with_capacity(2);
            for i in 0..2 {
                v.push(i);
            }
            let mut r_tuple = (None::<O0>, None::<O1>);
            let mut count: usize = 0;
            let mut count_old: usize = 0;
            let mut new_input = input;
            let mut er: PErr<'a, I> = Default::default();
            loop {
                if v[0] == 0 {
                    match self.0.parse(new_input) {
                        Ok((inp, r)) => {
                            r_tuple.0 = Some(r);
                            v[0] = usize::MAX;
                            new_input = inp;
                            count += 1;
                        }
                        Err(e) => er = e,
                    }
                }
                if v[1] == 1 {
                    match self.1.parse(new_input) {
                        Ok((inp, r)) => {
                            r_tuple.1 = Some(r);
                            v[1] = usize::MAX;
                            new_input = inp;
                            count += 1;
                        }
                        Err(e) => er = e,
                    }
                }
                if count == count_old {
                    break;
                }
                count_old = count;
            }
            if count == 2 {
                Ok((new_input, (r_tuple.0.unwrap(), r_tuple.1.unwrap())))
            } else {
                Err(er)
            }
        }
    }
*/
/// permut_impl!(90); max val 255 elemets tuple (A, B, ...)
#[proc_macro]
pub fn permut_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let i = parse_macro_input!(input as LitInt);
    let value_max = i.base10_parse::<u8>().unwrap()+1;
    if value_max < 2 { return Default::default(); }
    let mut gen = TokenStream::new();

    let for_vec_p: Vec<Ident> = (0..=value_max)
        .into_iter()
        .map(|a| Ident::new(&("P".to_owned()+&a.to_string()), Span::call_site()))
        .collect();

    let for_vec_o: Vec<Ident> = (0..=value_max)
        .into_iter()
        .map(|a| Ident::new(&("O".to_owned()+&a.to_string()), Span::call_site()))
        .collect();

    let for_num: Vec<Literal> = (0..=value_max)
        .into_iter()
        .map(|a| Literal::usize_unsuffixed(a.into()))
        .collect();    

    for value in 2..value_max as usize {
        let vec_o = &for_vec_o[..value];
        let vec_p = &for_vec_p[..value];
        let num = &for_num[..value];
        let v = Literal::usize_unsuffixed(value);

        let gen_part = quote! {
            impl<'a,I:'a,#(#vec_p,#vec_o),*> Permut<'a,I,(bool,(#(Option<#vec_o>),*)),(#(#vec_o),*)> for (#(#vec_p),*)
            where
            #(#vec_p: Parser<'a,I,#vec_o>),*
            {
                fn permutation_part(&self, input: &'a[I]) -> ParseResult<'a,I,(bool,(#(Option<#vec_o>),*))> {
                    let mut v:Vec<usize> = Vec::with_capacity(#v);
                    for i in 0..#v { v.push(i); }
                    let mut r_tuple = (#(None::<#vec_o>),*);
                    let mut count:usize = 0;
                    let mut count_old:usize = 0;
                    let mut new_input = input;
                    let mut er:PErr<'a,I> = Default::default();
                    loop {
                        #(if v[#num] == #num { 
                            match self.#num.parse(new_input) {
                                Ok((inp,r)) => { r_tuple.#num = Some(r); v[#num] = usize::MAX; new_input = inp; count+=1; },
                                Err(e)      => er = e, 
                            }
                        })*
                        if count == count_old { break; }
                        count_old = count;
                    }
                    match count {
                        c if c == #v     => Ok((new_input,(true, r_tuple))),
                        c if c>0         => Ok((new_input,(false, r_tuple))),
                        _                => Err(er),  
                    }
                }
            
                fn permutation(&self, input: &'a[I]) -> ParseResult<'a,I,(#(#vec_o),*)> {
                    let mut v:Vec<usize> = Vec::with_capacity(#v);
                    for i in 0..#v { v.push(i); }
                    let mut r_tuple = (#(None::<#vec_o>),*);
                    let mut count:usize = 0;
                    let mut count_old:usize = 0;
                    let mut new_input = input;
                    let mut er:PErr<'a,I> = Default::default();
                    loop {
                        #(if v[#num] == #num { 
                            match self.#num.parse(new_input) {
                                Ok((inp,r)) => { r_tuple.#num = Some(r); v[#num] = usize::MAX; new_input = inp; count+=1; },
                                Err(e)      => er = e, 
                            }
                        })*
                        if count == count_old { break; }
                        count_old = count;
                    }
                    if count == #v { Ok((new_input, ( #(r_tuple.#num.unwrap()),* ))) } else { Err(er) }
                }


            }
        };

        gen.append_all(gen_part);
    }

    //-shugar-// gen.into()
    proc_macro::TokenStream::from(gen)
}