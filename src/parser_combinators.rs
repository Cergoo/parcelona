//!  Parcelona minimalistic elegance parser combinator library.
//!
use parcelona_macros_derive::{alt_impl,permut_impl};
use std::{fmt,mem,cmp,default,error};
use bstr::ByteSlice;

pub type ParseResult<'a,I,O> = std::result::Result<(&'a [I],O),PErr<'a,I>>;

///
#[derive(Debug,Clone)] 
pub enum Msg<'a> {
    Str(&'a str),
    String(String),
}

/// type Error for parser
#[derive(Debug,Clone)]
pub struct PErr<'a,I> {
    input: &'a[I],
    user_msg: Vec<Msg<'a>>,
    to_srt: bool,
}

impl<'a,I> default::Default for PErr<'a,I> {
    fn default() -> Self {
        Self { input: &[], user_msg: Vec::new(), to_srt: false, }
    }
}

impl<'a,I:'a+std::fmt::Debug> error::Error for PErr<'a,I> {}

impl<'a,I:'a> PErr<'a,I> {
    /// constructor of new PErr
    pub fn new(input: &'a[I]) -> Self {
        Self { input: input, user_msg: Vec::<Msg>::new(), to_srt: false, }
    } 
    /// set type to str for Display
    pub fn fmt_str(mut self) -> Self { self.to_srt=true; self }  
    pub fn user_msg_push(mut self, m: Msg<'a>) -> Self { self.user_msg.push(m); self }
}

impl<'a,I:'a+fmt::Debug> fmt::Display for PErr<'a, I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_ln:usize = 95;
        let part: &[I];
        if self.input.len()>max_ln { part = &self.input[..95]; }
        else { part = self.input; }
        if self.to_srt {
            unsafe {
                let b = mem::transmute::<&[I], &[u8]>(part);
                writeln!(f, "Err: {:?}", b.as_bstr())?;
            }
        } else { writeln!(f, "Err: {:?}", part)?; }
        for i in self.user_msg.iter().rev() {
            writeln!(f, "{:?}", i)?;
        }
        Ok(())
    }
}

impl<'a,I:cmp::PartialEq> PartialEq for PErr<'a,I>  {
    fn eq(&self, other: &Self) -> bool {
        self.input == other.input
    }
}


 
/// Main parser definition
pub trait Parser<'a,I:'a,O>: Copy {
    fn parse(&self, input:&'a [I]) -> ParseResult<'a,I,O>;
    fn option(self) -> impl Parser<'a,I,Option<O>>                    { option(self) }
    fn more_max(self,c:usize) -> impl Parser<'a,I,Vec<O>>             { more_max(self,c) }
    fn more_min(self,c:usize) -> impl Parser<'a,I,Vec<O>>             { more_min(self,c) }
    fn more_exact(self,c:usize) -> impl Parser<'a,I,Vec<O>>           { more_exact(self,c) }
    fn more_range(self,c:(usize,usize)) -> impl Parser<'a,I,Vec<O>>   { more_range(self,c) }
    fn more(self) -> impl Parser<'a,I,Vec<O>>                         { more(self) }
    fn not(self) -> impl Parser<'a,I,()>                              { not(self) }
    fn msg_err(self, msg:&'a str) -> impl Parser<'a,I,O>              { msg_err(self,msg) }
    fn strerr(self) -> impl Parser<'a,I,O>                            { strerr(self) }
}

impl<'a,I:'a,F,O> Parser<'a,I,O> for F
where
    F: Fn(&'a[I]) -> ParseResult<'a,I,O>+Copy,
{
    fn parse(&self, input:&'a [I]) -> ParseResult<'a,I,O> {  self(input)  }
}

#[derive(Debug,Clone,Default)]
pub struct StaticClassOfSymbols<I: 'static> {
    one_enable:    &'static[I],
    one_disable:   &'static[I],
    parts_enable:  &'static[&'static[I]],
    parts_disable: &'static[&'static[I]],
    range_enable:  &'static[(I,I)],
    range_disable: &'static[(I,I)],
    default_enable_one: bool,
}

impl<I:Copy> StaticClassOfSymbols<I> {
    pub const fn new() -> Self {
        StaticClassOfSymbols {    
            one_enable:    &[],
            one_disable:   &[],
            parts_enable:  &[],
            parts_disable: &[],
            range_enable:  &[],
            range_disable: &[],
            default_enable_one: false,
        }
    }

    pub const fn one_enable_set(mut self, p:&'static[I]) -> Self {
        self.one_enable = p;
        self
    }

    pub const fn one_disable_set(mut self, p:&'static[I]) -> Self {
        self.one_disable = p;
        self
    }

    pub const fn parts_enable_set(mut self, p:&'static[&'static[I]]) -> Self {
        self.parts_enable = p;
        self
    }

    pub const fn parts_disable_set(mut self, p:&'static[&'static[I]]) -> Self {
        self.parts_disable = p;
        self
    }

    pub const fn range_enable_set(mut self, p:&'static[(I,I)]) -> Self {
        self.range_enable = p;
        self
    }

    pub const fn range_disable_set(mut self, p:&'static[(I,I)]) -> Self {
        self.range_disable = p;
        self
    }

    pub const fn default_enable_one(mut self, p:bool) -> Self {
        self.default_enable_one = p;
        self
    }
}

/// ClassOfSymbols it is an universal parser, alternative to seq_ext
/// for declarative programming style 
#[derive(Debug,Clone,Default)]
pub struct ClassOfSymbols<I> {
    one_enable:    Vec<I>,
    one_disable:   Vec<I>,
    parts_enable:  Vec<Vec<I>>,
    parts_disable: Vec<Vec<I>>,
    range_enable:  Vec<(I,I)>,
    range_disable: Vec<(I,I)>,
    /// if item of a slice is not disable then it is enable if `true`
    /// or if item of a slice is not enable then it is disable if `false`
    default_enable_one: bool,
}

impl<I:Copy> ClassOfSymbols<I> {

    pub fn one_enable_push(&mut self, p:&[I]) -> &mut Self {
        _=self.one_enable.splice(0..0, p.into_iter().copied());
        self
    }

    pub fn one_disable_push(&mut self, p:&[I]) -> &mut Self {
        _=self.one_disable.splice(0..0, p.into_iter().copied());
        self
        
    }

    pub fn range_enable_push(&mut self, p:&[(I,I)]) -> &mut Self {
        _=self.range_enable.splice(0..0, p.into_iter().copied());
        self
    }

    pub fn range_disable_push(&mut self, p:&[(I,I)]) -> &mut Self {
        _=self.range_disable.splice(0..0, p.into_iter().copied());
        self
    }

    pub fn parts_enable_push(&mut self, p:&[&[I]]) -> &mut Self {
        _=self.parts_enable.splice(0..0, p.into_iter().map(|x|x.into_iter().copied().collect::<Vec<_>>()));
        self
    }

    pub fn parts_disable_push(&mut self, p:&[&[I]]) -> &mut Self {
        _=self.parts_disable.splice(0..0, p.into_iter().map(|x|x.into_iter().copied().collect::<Vec<_>>()));
        self
    }

    pub fn default_enable_one(&mut self, b:bool) -> &mut Self {
        self.default_enable_one = b;
        self
    }
    
}


impl<'a,I:'a+cmp::PartialEq+cmp::PartialOrd> Parser<'a,I,&'a[I]> for &StaticClassOfSymbols<I> {
    fn parse(&self, input:&'a [I]) -> ParseResult<'a,I,&'a[I]> {
    let mut new_input = input;
    let mut c:usize = 0;
    let mut inner_c:usize = c;
    'outer: loop {  
        if new_input.is_empty() { break; }
        for i in self.parts_enable  { if new_input.starts_with(&i) { new_input = &new_input[i.len()..]; c+=i.len(); } }
        for i in self.parts_disable { if new_input.starts_with(&i) { break 'outer; } }
        for i in self.range_enable  { if i.0<=new_input[0] && i.1>=new_input[0] { new_input = &new_input[1..]; c+=1; } }
        for i in self.range_disable { if i.0<=new_input[0] && i.1>=new_input[0] { break 'outer; } }
        if self.one_enable.contains(&new_input[0])  { new_input = &new_input[1..]; c+=1; } 
        if self.one_disable.contains(&new_input[0]) { break 'outer; }  
        if self.default_enable_one { new_input = &new_input[1..]; c+=1; }
        if inner_c==c { break; }
        inner_c = c;
    }
    if c>0 { Ok(split_at_revers(input, c)) } else { Err(PErr::new(input)) }   
}
}

impl<'a,I:'a+cmp::PartialEq+cmp::PartialOrd> Parser<'a,I,&'a[I]> for &ClassOfSymbols<I> {
        fn parse(&self, input:&'a [I]) -> ParseResult<'a,I,&'a[I]> {
        let mut new_input = input;
        let mut c:usize = 0;
        let mut inner_c:usize = c;
        'outer: loop {  
            if new_input.is_empty() { break; }
            for i in &self.parts_enable  { if new_input.starts_with(&i) { new_input = &new_input[i.len()..]; c+=i.len(); } }
            for i in &self.parts_disable { if new_input.starts_with(&i) { break 'outer; } }
            for i in &self.range_enable  { if i.0<=new_input[0] && i.1>=new_input[0] { new_input = &new_input[1..]; c+=1; } }
            for i in &self.range_disable { if i.0<=new_input[0] && i.1>=new_input[0] { break 'outer; } }
            if self.one_enable.contains(&new_input[0])  { new_input = &new_input[1..]; c+=1; } 
            if self.one_disable.contains(&new_input[0]) { break 'outer; }  
            if self.default_enable_one { new_input = &new_input[1..]; c+=1; }
            if inner_c==c { break; }
            inner_c = c;
        }
        if c>0 { Ok(split_at_revers(input, c)) } else { Err(PErr::new(input)) }   
    }
}

/// Alt trait combinator, it is implement for tuples default max 16 elements
/// You can set cargo.toml flag `feature = "alt_tuple_32"` for up to tuple max 32 elements or `feature = "alt_tuple_64"` for up to tuple max 64 elements
pub trait Alt<'a,I:'a,O>: Copy {
    fn choice(&self, input:&'a [I]) -> ParseResult<'a,I,O>;
    fn alt(self) -> impl Parser<'a,I,O> {
        move |i| self.choice(i)
    }
}

impl<'a,I:'a,O,P1,P2> Alt<'a,I,O> for (P1,P2)
where
    P1: Parser<'a,I,O>,
    P2: Parser<'a,I,O>,
{
    fn choice(&self, input: &'a[I]) -> ParseResult<'a,I,O> {
        self.0.parse(input).or(self.1.parse(input))
    }
}

/// alt combinator
pub fn alt<'a,I:'a,O,T:Alt<'a,I,O>>(input: T) -> impl Parser<'a,I,O> {
    move |i| input.choice(i)
}

//

#[cfg(feature = "alt_tuple_32")]
alt_impl!(32);  

#[cfg(all(feature = "alt_tuple_64", not(feature = "alt_tuple_32")))] 
alt_impl!(64);  

#[cfg(not(any(feature = "alt_tuple_32", feature = "alt_tuple_64")))]
alt_impl!(16);  //max 255


/// Permut trait combinator, it is emplement for typles default max 16 elements
/// You can set cargo.toml flag `feature = "alt_tuple_32"` for up to tuple max 32 elements or `feature = "alt_tuple_64"` for up to tuple max 64 elements
pub trait Permut<'a,I:'a,O,Oo>: Copy {
    fn permutation_part(&self, input:&'a [I]) -> ParseResult<'a,I,O>;
    fn permutation(&self, input:&'a [I]) -> ParseResult<'a,I,Oo>;
    /// (P1,P2,P3).permut_part() -> impl Parser<'a,I,(bool,(Option<O1>,Option<O2>,Option<O3>))>
    /// `bool` element is `true` if all parts of tuple is Some
    fn permut_part(self) -> impl Parser<'a,I,O> { move |i| { self.permutation_part(i) } }
    /// (P1,P2,P3).permut() -> impl Parser<'a,I,(O1,O2,O3)>     
    fn permut(self) -> impl Parser<'a,I,Oo>     { move |i| { self.permutation(i) } }
}

/// permut combinator
pub fn permut<'a,I:'a,O,Oo,T:Permut<'a,I,O,Oo>>(input: T) -> impl Parser<'a,I,Oo> {
    move |i| input.permutation(i)
}

/// permut_part combinator
pub fn permut_part<'a,I:'a,O,Oo,T:Permut<'a,I,O,Oo>>(input: T) -> impl Parser<'a,I,O> {
    move |i| input.permutation_part(i)
}


#[cfg(feature = "alt_tuple_32")]
permut_impl!(32);  

#[cfg(all(feature = "alt_tuple_64", not(feature = "alt_tuple_32")))] 
permut_impl!(64);  

#[cfg(not(any(feature = "alt_tuple_32", feature = "alt_tuple_64")))]
permut_impl!(16);  //max 255


/// parser `data end`
pub fn data_end<'a,T>(a:&'a[T]) -> Result<(&[T],&[T]), PErr<'a,T>> {
    if !a.is_empty() { Err(PErr::new(a)) } else { Ok((a,a)) }
}

/// parser 'any'
pub fn any<'a,T:'a+Eq+Clone>(pattern: &'a[T]) -> impl Parser<'a,T,&'a[T]> {
    move |input:&'a[T]| { 
        if check_starts_with_any_element(pattern, input) { Ok(split_at_revers(input, 1)) } 
        else { Err(PErr::new(input)) }
    }
}

/// parser 'starts_with'
pub fn starts_with<'a,T:'a+Eq+Clone>(pattern: &'a[T]) -> impl Parser<'a,T,&'a[T]> {
    move |input:&'a[T]| { 
        if input.starts_with(pattern) {
           return  Ok(split_at_revers(input, pattern.len()));
        } 
        Err(PErr::new(input)) 
    }
}

///  parser 'starts_with_any'
pub fn starts_with_any<'a,T:'a+Eq+Clone>(patterns: &'a[&'a[T]]) -> impl Parser<'a,T,&'a[T]> {
    move |input:&'a[T]| {
        let l = check_starts_with_any_part(patterns, input);
        if l>0 { return  Ok(split_at_revers(input, l)); }
        Err(PErr::new(input))
    }
}

///  function for sec_ext
pub fn check_starts_with_any_part<'a,T:'a+Eq+Clone>(patterns: &'a[&'a[T]], input:&'a[T]) -> usize {
    for i in patterns { if input.starts_with(i) { return i.len(); } }
    0
}

///  function for sec_ext
pub fn check_starts_with_any_element<'a,T:'a+Eq+Clone>(patterns: &'a[T], input:&'a[T]) -> bool {
    !input.is_empty() && patterns.contains(&input[0])
}



/// parser `sequence maximum`
pub fn seq_max<'a,P,T:'a+Eq+Clone>(p: P, count_max:usize) -> impl Parser<'a,T,&'a[T]>
where
    P: Fn(& T) -> bool+Copy+'a,
{
     move |input:&'a[T]| {  
         let mut c:usize = 0;    
         for i in input { if c<count_max&&p(i) {c+=1;} else {break;} }
         if c>0 { return Ok(split_at_revers(input, c)); }
         Err(PErr::new(input))
     }
}

/// parser `sequence minimum`
pub fn seq_min<'a,P,T:'a+Eq+Clone>(p: P, count_min:usize) -> impl Parser<'a,T,&'a[T]>
where
    P: Fn(& T) -> bool+Copy+'a,
{
     move |input:&'a[T]| {  
         let mut c:usize = 0;    
         for i in input { if p(i) {c+=1;} else {break;} }
         if c<count_min { Err(PErr::new(input)) } else { Ok(split_at_revers(input, c)) } 
     }
}

/// parser `sequence range`
pub fn seq_range<'a,P,T:'a+Eq+Clone>(p: P, range:(usize,usize)) -> impl Parser<'a,T,&'a[T]>
where
    P: Fn(& T) -> bool+Copy+'a,
{
     move |input:&'a[T]| {  
         let mut c:usize = 0;    
         for i in input { if c<range.1&&p(i) {c+=1;} else {break;} }
         if c<range.0 { Err(PErr::new(input)) } else { Ok(split_at_revers(input, c)) } 
     }
}

/// parser `sequence exact`
pub fn seq_exact<'a,P,T:'a+Eq+Clone>(p: P, count_exact:usize) -> impl Parser<'a,T,&'a[T]>
where
    P: Fn(& T) -> bool+Copy+'a,
{
     move |input:&'a[T]| {  
         let mut c:usize = 0;    
         for i in input { if c<count_exact&&p(i) {c+=1;} else {break;} }
         if c<count_exact { Err(PErr::new(input)) } else { Ok(split_at_revers(input, c)) } 
     }
}

/// parser `sequence`
pub fn seq<'a,P,T:'a+Eq+Clone>(p: P) -> impl Parser<'a,T,&'a[T]>
where
    P: Fn(& T) -> bool+Copy+'a,
{
     move |input:&'a[T]| {  
         let mut c:usize = 0;    
         for i in input { if p(i) {c+=1;} else {break;} }
         if c<1 { Err(PErr::new(input)) } else { Ok(split_at_revers(input, c)) } 
     }
}

/// parser `sequence extendet`
pub fn seq_ext<'a,P,T:'a+Eq+Clone>(p: P) -> impl Parser<'a,T,&'a[T]>
where
    P: Fn(& [T]) -> usize+Copy+'a,
{
     move |input:&'a[T]| {     
        let mut new_input = input;
        loop {
            let l = p(new_input); 
            if l<1 || new_input.len()<l {break;} 
            (new_input, _) = split_at_revers(new_input, l); 
        } 
        let c = input.len() - new_input.len();
        if c==0 { Err(PErr::new(input)) } else { Ok(split_at_revers(input, c)) } 
     }
}

#[inline]
pub fn is_any<T>(_:&T) -> bool { true }

/// parser `take`
pub fn take<'a,T:'a>(count:usize) -> impl Parser<'a,T,&'a[T]> {
    move |input:&'a[T]| { take_record(input, count) }
}

/// `notp` closur for `seq` a func parametr. notp(predicat)
pub fn notp<T>(f: impl Fn(& T) -> bool) -> impl Fn(& T) -> bool
{ move |x| !f(x) }




/// combinators

/// combinator not(parser)
pub fn not<'a,T:'a,P,R>(parser: P) -> impl Parser<'a,T,()>
where
    P: Parser<'a,T,R>,
{
    move |input| {
        match parser.parse(input) {
            Ok(_) => Err(PErr::new(input)),
            _     => Ok((input,())),
    }}
}

/// combinator fmap
pub fn fmap<'a,T:'a,F,P,R1,R2>(parser: P, map_fn: F) -> impl Parser<'a,T,R2>
where
    P: Parser<'a,T,R1>,
    F: Fn(R1) -> R2 + Copy,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

pub fn frmap<'a,T:'a,F,P,R1,R2,E:ToString>(parser: P, map_fn: F) -> impl Parser<'a,T,R2>
where
    P: Parser<'a,T,R1>,
    F: Fn(R1) -> Result<R2,E> + Copy,
{
    move |input| {
        let r = parser.parse(input)?;
        match map_fn(r.1) {
            Ok(rr) => Ok((r.0, rr)),
            Err(e) => { 
                let ne = PErr::new(r.0)
                .user_msg_push(Msg::Str("Error frmap applying function to parsing result"))
                .user_msg_push(Msg::String(e.to_string()))
                .fmt_str();
                Err(ne)
            } 
        }
    }
}

/// combinator map
pub fn map<'a,T:'a,F,P,R1,R2>(parser: P, map_fn: F) -> impl Parser<'a,T,R2>
where
    P: Parser<'a,T,R1>,
    F: Fn((&'a[T],R1)) -> ParseResult<'a,T,R2> + Copy,
{
    move |input| { map_fn(parser.parse(input)?) }
}

/// combinator map_err
pub fn msg_err<'a,T:'a,P,R>(parser: P, msg: &'a str) -> impl Parser<'a,T,R>
where
    P: Parser<'a,T,R>,
{
    move |input| { parser.parse(input).map_err(|mut x|{x.user_msg.push(Msg::Str(msg)); x}) }
}

/// combinator map_err
pub fn strerr<'a,T:'a,P,R>(parser: P) -> impl Parser<'a,T,R>
where
    P: Parser<'a,T,R>,
{
    move |input| { parser.parse(input).map_err(|mut x|{x.to_srt=true; x}) }
}

/// combinator option - allways return Ok, no Err
pub fn option<'a,T:'a,P,R>(parser: P) -> impl Parser<'a,T,Option<R>>
where
    P: Parser<'a,T,R>,
{
    move |input| {  
        match parser.parse(input) {
            Ok((input,r)) => Ok((input,Some(r))),
            _             => Ok((input,None))   
    }}
}

/// combinator pair
pub fn pair<'a,T:'a,P1,P2,R1,R2>(p1:P1,p2:P2) -> impl Parser<'a,T,(R1,R2)>
where
    P1: Parser<'a,T,R1>,
    P2: Parser<'a,T,R2>,
{
    move |input| {
        p1.parse(input).and_then(|(next_input,r1)| { 
        p2.parse(next_input).map(|(next_input,r2)| (next_input,(r1,r2))) })
    }
}

/// combinator or
pub fn or<'a,T:'a,P1,P2,R1,R2>(p1:P1,p2:P2) -> impl Parser<'a,T,(Option<R1>,Option<R2>)>
where
    P1: Parser<'a,T,R1>,
    P2: Parser<'a,T,R2>,
{
    move |input| {
        let rp = p1.parse(input);
        match rp {
            Ok((next_input,r1)) => {
                let rp = p2.parse(next_input);
                match rp {
                    Ok((next_input,r2)) => { return Ok((next_input,(Some(r1),Some(r2)))); },
                    _                   => { return Ok((next_input,(Some(r1),None))); },
                }
            },
            _  => {
                let rp = p2.parse(input);
                match rp {
                    Ok((next_input,r2)) => { return Ok((next_input,(None,Some(r2)))); },
                    Err(e)              => { return Err(e); },
                }
            },
        }
    }
}

/// combinator left
pub fn left<'a,T:'a,P1,P2,R1,R2>(p1:P1,p2:P2) -> impl Parser<'a,T,R1>
where
    P1: Parser<'a,T,R1>,
    P2: Parser<'a,T,R2>,
{
    fmap(pair(p1,p2),|(l,_)|l)
}

/// combinator right
pub fn right<'a,T:'a,P1,P2,R1,R2>(p1:P1,p2:P2) -> impl Parser<'a,T,R2>
where
    P1: Parser<'a,T,R1>,
    P2: Parser<'a,T,R2>,
{
    fmap(pair(p1,p2),|(_,r)|r)
}

/// combinator right 'left'-is options, if left returns Error it is ignored
pub fn right_opt<'a,T:'a,P1,P2,R1,R2>(p1:P1,p2:P2) -> impl Parser<'a,T,R2>
where
    P1: Parser<'a,T,R1>,
    P2: Parser<'a,T,R2>,
{
    move |input| {
        if let Ok((input,_)) = p1.parse(input) { p2.parse(input) } 
        else { p2.parse(input) }
    }
}
 
/// combinator left 'right'-is options, if right returns Error it is ignored
pub fn left_opt<'a,T:'a,P1,P2,R1,R2>(p1:P1,p2:P2) -> impl Parser<'a,T,R1>
where
    P1: Parser<'a,T,R1>,
    P2: Parser<'a,T,R2>,
{
    map(p1, move|(i, r1)| { match p2.parse(i) {
            Ok((i,_)) => Ok((i,r1)),
            _         => Ok((i,r1)),
    }})
}

/// combinator `find stop`
pub fn find_stop<'a,T:'a,P1,P2,R1,R2>(p:P1, stop:P2) -> impl Parser<'a,T,R1>
where
    P1: Parser<'a,T,R1>,
    P2: Parser<'a,T,R2>,
{
    move |input: &'a[T]| {
        let mut new_input = input;
        loop {    
            let r = p.parse(new_input);
            if r.is_ok() { return r; }
            let s = stop.parse(new_input);
            if s.is_ok() { return Err(PErr::new(new_input)); }
            (new_input,_) = take_record(new_input,1)?;
        }
}}

/// find combinator
pub fn find<'a,T:'a,P,R>(p:P) -> impl Parser<'a,T,R>
where
    P: Parser<'a,T,R>
{
    move |input: &'a[T]| {
        let mut new_input = input;
        loop {    
            let r = p.parse(new_input);
            if r.is_ok() { return r; }
            (new_input,_) = take_record(new_input,1)?;
        }
}}

/// combinator `more maximum`
pub fn more_max<'a,T:'a,P,R>(p:P, count_max:usize) -> impl Parser<'a,T,Vec<R>>
where
    P: Parser<'a,T,R>,
{   
    move |input: &'a[T]| {
        let mut result = Vec::new();
        let mut next_input1 = input;
        loop {
            match p.parse(next_input1) {
                Ok((next_input2,r)) => {               
                    result.push(r);
                    next_input1 = next_input2;
                    if result.len()==count_max { break; }  
                },
                Err(e) => { 
                    if result.is_empty() { return Err(e); }
                    break;
                },
        }}
        Ok((next_input1, result))
    }
}

/// combinator `more minimum`
pub fn more_min<'a,T:'a,P,R>(p:P, count_min:usize) -> impl Parser<'a,T,Vec<R>>
where
    P: Parser<'a,T,R>,
{   
    move |input: &'a[T]| {
        let mut result = Vec::new();
        let mut next_input1 = input;
        loop {
            match p.parse(next_input1) {
                Ok((next_input2,r)) => {
                    result.push(r);            
                    next_input1 = next_input2;
                },
                Err(e) => { 
                    if result.len()<count_min { return Err(e); }
                    break 
                }, 
        }}
        Ok((next_input1, result))
    }
}

/// combinator `more range`
pub fn more_range<'a,T:'a,P,R>(p:P, range:(usize,usize)) -> impl Parser<'a,T,Vec<R>>
where
    P: Parser<'a,T,R>,
{   
    move |input: &'a[T]| {
        let mut result = Vec::new();
        let mut next_input1 = input;
        loop {
            match p.parse(next_input1) {
                Ok((next_input2,r)) => {
                    result.push(r);            
                    next_input1 = next_input2;
                    if result.len()==range.1 { break; }
                },
                Err(e) => { 
                    if result.len()<range.0 { return Err(e); } 
                    break;
                },
        }}
        Ok((next_input1, result))
    }
}

/// combinator `more exact`
pub fn more_exact<'a,T:'a,P,R>(p:P, count_exact:usize) -> impl Parser<'a,T,Vec<R>>
where
    P: Parser<'a,T,R>,
{   
    move |input: &'a[T]| {
        let mut result = Vec::new();
        let mut next_input1 = input;
        loop {
            match p.parse(next_input1) {
                Ok((next_input2,r)) => {
                    result.push(r);
                    next_input1 = next_input2;
                    if result.len()==count_exact { break; }
                },
                Err(e) => { return Err(e); },
        }}    
        Ok((next_input1, result))
    }
}

/// combinator `more no zero`
pub fn more<'a,T:'a,P,R>(p:P) -> impl Parser<'a,T,Vec<R>>
where
    P: Parser<'a,T,R>,
{   
    move |input: &'a[T]| {
        let mut result = Vec::new();
        let mut next_input1 = input;
        loop {
            match p.parse(next_input1) {
                Ok((next_input2,r)) => {   
                    result.push(r);
                    next_input1 = next_input2; },
                Err(e) => { 
                    if result.is_empty() { return Err(e); }
                    break; 
                },
        }}
        Ok((next_input1, result))
    }
}

/// combinator separated pair
pub fn sep_pair<'a,T:'a,P1,P_,P2,R1,R2,R_>(p1:P1,sep:P_,p2:P2) -> impl Parser<'a,T,(R1,R2)>
where
    P1: Parser<'a,T,R1>,
    P2: Parser<'a,T,R2>,
    P_: Parser<'a,T,R_>,
{
    pair(left(p1,sep),p2)   
}

/// combinator separated pair optional
pub fn sep_pair_opt<'a,T:'a,P1,P_,P2,R1,R2,R_>(p1:P1,sep:P_,p2:P2) -> impl Parser<'a,T,(R1,R2)>
where
    P1: Parser<'a,T,R1>,
    P2: Parser<'a,T,R2>,
    P_: Parser<'a,T,R_>,
{
    pair(left_opt(p1,sep),p2)   
}

/// combinator element between
pub fn between<'a,T:'a,P1,P,P2,R1,R,R2>(p1:P1,p:P,p2:P2) -> impl Parser<'a,T,R>
where
    P1: Parser<'a,T,R1>,
    P: Parser<'a,T,R>,
    P2: Parser<'a,T,R2>,
{
    left(right(p1,p),p2)   
}

/// combinator element between optional
pub fn between_opt<'a,T:'a,P1,P,P2,R1,R,R2>(p1:P1,p:P,p2:P2) -> impl Parser<'a,T,R>
where
    P1: Parser<'a,T,R1>,
    P: Parser<'a,T,R>,
    P2: Parser<'a,T,R2>,
{
    left_opt(right_opt(p1,p),p2)   
}

/// combinator and_then
pub fn and_then<'a,T:'a,P1,P2,R1,R2,F,R3>(p1:P1,p2:P2,f:F) -> impl Parser<'a,T,R3>
where
    P1: Parser<'a,T,R1>,
    P2: Parser<'a,T,R2>,
    F: Fn((R1,R2)) -> R3 + Copy,
{
    fmap(pair(p1,p2),f)
}

/// combinator and_then
pub fn or_then<'a,T:'a,P1,P2,R1,R2,F,R3>(p1:P1,p2:P2,f:F) -> impl Parser<'a,T,R3>
where
    P1: Parser<'a,T,R1>,
    P2: Parser<'a,T,R2>,
    F: Fn((Option<R1>,Option<R2>)) -> R3 + Copy,
{
    fmap(or(p1,p2),f)
}

/// combinator separated list
///1) h, h, h, h hh
///   ----------
///2) h, h, h, hh hhh
///   --------
///3) h, h, h, hb hhh
///   --------
///4) h, h, h h
///   -------
pub fn sep_list<'a,T:'a,Pe,Re,Ps,Rs,Ple>(elem:Pe,sep:Ps,last_elem:Ple) -> impl Parser<'a,T,Vec<Re>>
where
    Pe:  Parser<'a,T,Re>,
    Ps:  Parser<'a,T,Rs>,
    Ple: Parser<'a,T,Re>,
{
    or_then(more(left(elem,sep)), last_elem, 
        |(a,b)| {
           match (a,b) {
               (Some(mut a), Some(b)) => { a.push(b); a },
               (Some(a), None)        =>  a, 
               (None, Some(b))        => { let r=vec![b]; r },
               _                      => { panic!("newer do sep_list!"); },
           }  
    })   
}

/// just usefull function
#[inline]
pub fn split_at_revers<T>(input: &[T], count: usize) -> (&[T], &[T]) {
    (&input[count..], &input[..count])
}

/// just read record
pub fn take_record<'a,T>(b: &'a[T], l: usize) -> Result<(&[T], &[T]), PErr<'a,T>> {
	if b.len() < l { return Err(PErr::new(b)); }
	Ok(split_at_revers(b, l))
}

/// just useful function
pub fn fflaten<T:Copy>(v:Vec<&[T]>) -> Vec<T> {
    v.into_iter().flatten().copied().collect::<Vec<T>>()
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
