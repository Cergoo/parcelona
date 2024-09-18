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
              
 let p1 = right_opt(seq(is_space), seq(is_alpha_upper));
 let r1 = p1.parse(d.0);
 assert_eq!(r1, Ok((&d.0[8..], &d.0[1..8])));
}
   
#[test]
fn t2() {    
    let p=left_opt(seq_exact(is_alpha_upper,3), seq(is_space));
    let r=p.parse(b"GET HTTttp");
    assert_eq!(Ok(("HTTttp".as_bytes(),"GET".as_bytes())), r);
}

#[test]
fn t3() {    
    let p=seq(is_alpha_upper).option();
    let r=p.parse(b"GET HTTttp");
    assert_eq!(Ok((" HTTttp".as_bytes(),Some("GET".as_bytes()))), r);
}

#[test]
fn t_find() {
    let data="mnb mnbmb bmnm jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();     
    let parser=find(starts_with(b"CONNECT"));
    let result=parser.parse(data);
    assert_eq!(Ok((": 1 mnbnm mnmn/r/n nbn".as_bytes(),"CONNECT".as_bytes())), result);
}

#[test]
fn t_find_sep_pair() {
let data="mnb mnbmb bmnm jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();

let space = seq(is_space);  
let parser=find(
    sep_pair(
        starts_with(b"CONNECT"),
        right_opt(space, any(b":")),
        right_opt(space, seq(is_dec_digit))
    ));
 
let result=parser.parse(data);
assert_eq!(Ok((" mnbnm mnmn/r/n nbn".as_bytes(),("CONNECT".as_bytes(),"1".as_bytes()))), result);
}

#[test]
fn t_more() {
let data="b:12 b:2 jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();  
let search_it = find(seq(is_dec_digit));
let p = search_it.more().parse(data);

assert_eq!(Ok((" mnbnm mnmn/r/n nbn".as_bytes(),Vec::from(["12".as_bytes(), "2".as_bytes(), "1".as_bytes()]))), p);
}

#[test]
fn t_find1() {
let data="b:12 b:2 jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();  
let search_it = find(seq(is_dec_digit));
let p = search_it.more().parse(data);

assert_eq!(Ok((" mnbnm mnmn/r/n nbn".as_bytes(),Vec::from(["12".as_bytes(), "2".as_bytes(), "1".as_bytes()]))), p);
}

#[test]
fn t_alt() {
    let data="b:12 b:2 jkmn CONNECT: 1 mnbnm mnmn/r/n nbn".as_bytes();

    let s1 = seq_exact(is_any,1);  
    let s2 = seq_exact(is_any,2); 
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

let hex_color = fmap(seq_exact(is_hex_digit,2),|x| {let (r,_) = u8::from_radix_16(x); r});
let (_input,c) = right(starts_with(b"#"), hex_color.more_exact(3)).parse(input).unwrap();
let color = Color{ red:c[0], green:c[1], blue:c[2] };

assert_eq!(Color{red: 47, green: 20, blue: 223}, color);
}

#[test]
fn t_exact() {
let data="bb".as_bytes();
let p = seq_exact(is_any,3).parse(data).ok();  
assert_eq!(None, p);
}

#[test]
fn t_exact1() {
    use byteorder::{ByteOrder, BE}; 
    let data = [4, 7];
    let p = fmap(seq_exact(is_any,3),|x|{BE::read_u24(x) as usize});
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
    let space   = seq(is_space); 
    let element = seq_exact(is_alpha,1);  
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

    let data: &[u8] = b" 1 2";
    let r = list.parse(&data);
    assert_eq!(Err(data), r);
}

#[test]
fn t_t_f() { 
    let data: &[u8] = b"true|false truefalse";
    let p_true =  fmap(starts_with(b"true"), |_|true);
    let p_false = fmap(starts_with(b"false"), |_|false);
    let (_input, result) = find((p_false,p_true).alt()).more_zero().parse(data).unwrap();
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