use parcelona::parser_combinators::{*};
use parcelona::u8::{*};
use parcelona::u8ext::{*};
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
              
 let p1 = right_opt(seq(is_space,SeqCount::None), seq(is_alpha_upper,SeqCount::None));
 let r1 = p1.parse(d.0);
 assert_eq!(r1, Ok((&d.0[8..], &d.0[1..8])));
}
   
#[test]
fn t2() {    
    let p=left_opt(seq(is_alpha_upper,SeqCount::Exact(3)), seq(is_space,SeqCount::None));
    let r=p.parse(b"GET HTTttp");
    assert_eq!(Ok(("HTTttp".as_bytes(),"GET".as_bytes())), r);
}

#[test]
fn t3() {    
    let p=seq(is_alpha_upper,SeqCount::None).option();
    let r=p.parse(b"GET HTTttp");
    assert_eq!(Ok((" HTTttp".as_bytes(),Some("GET".as_bytes()))), r);
}

#[test]
fn t_find() {
    let data="mnb mnbmb bmnm jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();     
    let parser=find(seq(is_no_eol,SeqCount::Exact(1)), starts_with(b"CONNECT"));
    let result=parser.parse(data);
    assert_eq!(Ok((": 1 mnbnm mnmn/r/n nbn".as_bytes(),"CONNECT".as_bytes())), result);
}

#[test]
fn t_find_sep_pair() {
let data="mnb mnbmb bmnm jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();

let space = seq(is_space,SeqCount::None);  
let parser=find(
    seq(is_no_eol,SeqCount::Exact(1)),
    sep_pair(
        starts_with(b"CONNECT"),
        right_opt(space, any(b":")),
        right_opt(space, seq(is_dec_digit,SeqCount::None))
    ));
 
let result=parser.parse(data);
assert_eq!(Ok((" mnbnm mnmn/r/n nbn".as_bytes(),("CONNECT".as_bytes(),"1".as_bytes()))), result);
}

#[test]
fn t_more() {
let data="b:12 b:2 jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();

let space = seq(is_any,SeqCount::Exact(1));  
let search_it = find(space, seq(is_dec_digit,SeqCount::None));
let p = search_it.more(NO_ZERO).parse(data);

assert_eq!(Ok((" mnbnm mnmn/r/n nbn".as_bytes(),Vec::from(["12".as_bytes(), "2".as_bytes(), "1".as_bytes()]))), p);
}

#[test]
fn t_find1() {
let data="b:12 b:2 jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();

let space = seq(is_any,SeqCount::Exact(1));  
let search_it = find(space, seq(is_dec_digit,SeqCount::None));
let p = search_it.more(NO_ZERO).parse(data);

assert_eq!(Ok((" mnbnm mnmn/r/n nbn".as_bytes(),Vec::from(["12".as_bytes(), "2".as_bytes(), "1".as_bytes()]))), p);
}

#[test]
fn t_alt() {
    let data="b:12 b:2 jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();

    let s1 = seq(is_any,SeqCount::Exact(1));  
    let s2 = seq(is_any,SeqCount::Exact(2)); 
    let (_i,r) = (s2,s1).choice(data).unwrap();
    assert_eq!(r, b"b:");
    ()
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

let hex_color = seq(is_hex_digit,SeqCount::Exact(2));
let (input,_) = starts_with(b"#").parse(input).unwrap();
let (_input,c) = hex_color.more(NO_ZERO).parse(input).unwrap();
let (r,_) = u8::from_radix_16(c[0]);
let (g,_) = u8::from_radix_16(c[1]);
let (b,_) = u8::from_radix_16(c[2]);
let color = Color{ red:r, green:g, blue:b };

assert_eq!(Color{red: 47, green: 20, blue: 223}, color);
}

#[test]
fn t_exact() {
let data="bb".as_bytes();
let p = seq(is_any,SeqCount::Exact(3)).parse(data).ok();  
assert_eq!(None, p);
}

#[test]
fn t_exact1() {
    use byteorder::{ByteOrder, BE}; 
    let data = [4, 7];
    let p = map(seq(is_any,SeqCount::Exact(3)),|x|{BE::read_u24(x) as usize});
    assert_eq!(None, p.parse(&data).ok());
}

#[test]
fn t_u8ext() { 
    let data: &[u8] = &[0, 2, 8, 9, 0, 2, 7, 8];
    let p = pair(take_record_be_u16, take_record_be_u16);
    assert_eq!(Some(([].as_slice(), ([8_u8,9].as_slice(), [7_u8,8].as_slice()))), p.parse(&data).ok());
}

#[test]
fn t_sep_list() { 
    let space   = seq(is_space,SeqCount::None); 
    let element = seq(is_alpha,SeqCount::Exact(1));  
    let separ   = starts_with(b",");  
    let list = sep_list( 
            between_opt(space,element,space),
            separ,
            left(right_opt(space,element), (space,data_end).alt()),
        );

    let data: &[u8] = b"h , h , h , h hh";
    let (_i,r) = list.parse(&data).unwrap();
    let r: Vec<u8> = fflaten(r);
    assert_eq!(b"hhhh".to_vec(), r);

    let data: &[u8] = b"h , h , h , hhh";
    let (_i,r) = list.parse(&data).unwrap();
    let r: Vec<u8> = fflaten(r);
    assert_eq!(b"hhh".to_vec(), r);

    let data: &[u8] = b"h , h , h h hh";
    let (_i,r) = list.parse(&data).unwrap();
    let r: Vec<u8> = fflaten(r);
    assert_eq!(b"hhh".to_vec(), r);

    let data: &[u8] = b"h , h , h";
    let (_i,r) = list.parse(&data).unwrap();
    let r: Vec<u8> = fflaten(r);
    assert_eq!(b"hhh".to_vec(), r);

    let data: &[u8] = b"h , h , h ,";
    let (_i,r) = list.parse(&data).unwrap();
    let r: Vec<u8> = fflaten(r);
    assert_eq!(b"hhh".to_vec(), r);

    let data: &[u8] = b" h ";
    let (_i,r) = list.parse(&data).unwrap();
    let r: Vec<u8> = fflaten(r);
    assert_eq!(b"h".to_vec(), r);

    let data: &[u8] = b" h , ";
    let (_i,r) = list.parse(&data).unwrap();
    let r: Vec<u8> = fflaten(r);
    assert_eq!(b"h".to_vec(), r);
}

#[test]
fn t_t_f() { 
    let data: &[u8] = b"true|false truefalse";
    let p_true =  map(starts_with(b"true"), |_|true);
    let p_false = map(starts_with(b"false"), |_|false);
    let step = seq(is_any,SeqCount::Exact(1));
    let (_input, result) = find(step, (p_false,p_true).alt()).more(ZERO).parse(data).unwrap();
    assert_eq!(vec![true,false,true,false], result);
}

#[test]
fn t_simple() { 
    let data: &[u8] = b"true";
    let c:usize = 2;
    let t1 = take(c);
    let t2 = take(c);
    let (data, r1) = t1.parse(data).unwrap();
    let (_data, r2) = t2.parse(data).unwrap();

    assert_eq!(b"tr", r1);
    assert_eq!(b"ue", r2);
}