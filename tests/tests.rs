use parselona::parser_combinators::{*};
use parselona::u8::{*};
use atoi::FromRadix16;

#[test]
fn t1() {
let d = (" CONNECT linkedin.com 1/2/4".as_bytes(),
         ("".as_bytes(), 
          "CONNECT".as_bytes(),
          "linledin".as_bytes(),
          "com".as_bytes(),
           (1,2,4),
        ));
              
 let p1 = right_opt(take(seq(is_space,SeqCount::None)), take(seq(is_alpha_upper,SeqCount::None)));
 let r1 = p1.parse(d.0);
 assert_eq!(r1, Ok((&d.0[8..], &d.0[1..8])));
}
   
#[test]
fn t2() {    
let p=left_opt(take(seq(is_alpha_upper,SeqCount::Exact(3))), take(seq(is_space,SeqCount::None)));
let r=p.parse(b"GET HTTttp");
assert_eq!(Ok(("HTTttp".as_bytes(),"GET".as_bytes())), r);
}

#[test]
fn t3() {    
let p=take(seq(is_alpha_upper,SeqCount::None)).option();
let r=p.parse(b"GET HTTttp");
assert_eq!(Ok((" HTTttp".as_bytes(),Some("GET".as_bytes()))), r);
}

#[test]
fn t_find() {
let data="mnb mnbmb bmnm jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();     
let parser=find(take(seq(is_no_eol,SeqCount::Exact(1))),take(starts_with(b"CONNECT")));
let result=parser.parse(data);
assert_eq!(Ok((": 1 mnbnm mnmn/r/n nbn".as_bytes(),"CONNECT".as_bytes())), result);
}

#[test]
fn t_find_sep_pair() {
let data="mnb mnbmb bmnm jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();

let space = take(seq(is_space,SeqCount::None));  
let parser=find(
    take(seq(is_no_eol,SeqCount::Exact(1))),
    sep_pair(
        take(starts_with(b"CONNECT")),
        right_opt(space.clone(),take(any(b":"))),
        right_opt(space.clone(),take(seq(is_dec_digit,SeqCount::None)))
    ));
 
let result=parser.parse(data);
assert_eq!(Ok((" mnbnm mnmn/r/n nbn".as_bytes(),("CONNECT".as_bytes(),"1".as_bytes()))), result);
}

#[test]
fn t_more() {
let data="b:12 b:2 jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();

let space = take(seq(is_any,SeqCount::Exact(1)));  
let search_it = find( space.clone(), take(seq(is_dec_digit,SeqCount::None)) );
let p = search_it.more(NO_ZERO).parse(data);

assert_eq!(Ok((" mnbnm mnmn/r/n nbn".as_bytes(),Vec::from(["12".as_bytes(), "2".as_bytes(), "1".as_bytes()]))), p);
}

#[test]
fn t_find_alt() {
let data="b:12 b:2 jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();

let space = take(seq(is_any,SeqCount::Exact(1)));  
let search_it = find( space.clone(), take(seq(is_dec_digit,SeqCount::None)) );
let p = search_it.more(NO_ZERO).parse(data);

assert_eq!(Ok((" mnbnm mnmn/r/n nbn".as_bytes(),Vec::from(["12".as_bytes(), "2".as_bytes(), "1".as_bytes()]))), p);
}

#[test]
fn t_color() {
#[derive(Debug, PartialEq)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

let input = "#2F14DF".as_bytes();

let hex_color = take(seq(is_hex_digit,SeqCount::Exact(2)));
let (input,_) = take(starts_with(b"#")).parse(input).unwrap();
let (input,c) = hex_color.more(NO_ZERO).parse(input).unwrap();
let (r,_) = u8::from_radix_16(c[0]);
let (g,_) = u8::from_radix_16(c[1]);
let (b,_) = u8::from_radix_16(c[2]);
let color = Color{ red:r, green:g, blue:b };

assert_eq!(Color{red: 47, green: 20, blue: 223}, color);
}


