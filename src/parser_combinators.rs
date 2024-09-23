//!  Parcelona minimalistic elegance parser combinator library.
//!
use parcelona_macros_derive::{alt_impl,permut_impl};

pub type ParseResult<'a,I,O> = std::result::Result<(&'a [I],O),&'a [I]>;
 
/// Main parser definition
pub trait Parser<'a,I:'a,O>: Copy {
    fn parse(&self, input:&'a [I]) -> ParseResult<'a,I,O>;
    fn option(self) -> impl Parser<'a,I,Option<O>>                    { option(self) }
    fn more_max(self,c:usize) -> impl Parser<'a,I,Vec<O>>             { more_max(self,c) }
    fn more_min(self,c:usize) -> impl Parser<'a,I,Vec<O>>             { more_min(self,c) }
    fn more_exact(self,c:usize) -> impl Parser<'a,I,Vec<O>>           { more_exact(self,c) }
    fn more_range(self,c:(usize,usize)) -> impl Parser<'a,I,Vec<O>>   { more_range(self,c) }
    fn more(self) -> impl Parser<'a,I,Vec<O>>                         { more(self) }
    fn more_zero(self) -> impl Parser<'a,I,Vec<O>>                    { more_zero(self) }
    fn not(self) -> impl Parser<'a,I,()>                              { not(self) }
}

impl<'a,I:'a,F,O> Parser<'a,I,O> for F
where
    F: Fn(&'a[I]) -> ParseResult<'a,I,O>+Copy,
{
    fn parse(&self, input:&'a [I]) -> ParseResult<'a,I,O> {  self(input)  }
}

/// Alt trait combinator, it is implement for tuples default max 16 elements
/// You can set flag `feature = "alt_tuple_32"` for up to tuple max 32 elements or `feature = "alt_tuple_64"` for up to tuple max 64 elements
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
/// You can set flag `feature = "alt_tuple_32"` for up to tuple max 32 elements or `feature = "alt_tuple_64"` for up to tuple max 64 elements
pub trait Permut<'a,I:'a,O,Oo>: Copy {
    fn permutation_part(&self, input:&'a [I]) -> ParseResult<'a,I,O>;
    fn permutation(&self, input:&'a [I]) -> ParseResult<'a,I,Oo>;
    /// (P1,P2,P3).permut_part() -> impl Parser<'a,I,(bool,(Option<O1>,Option<O2>,Option<O3>))>
    /// `bool` element is `true` if all parts of tuple is Some
    fn permut_part(self) -> impl Parser<'a,I,O> { move |i| { self.permutation_part(i) } }
    /// (P1,P2,P3).permut() -> impl Parser<'a,I,(O1,O2,O3)>     
    fn permut(self) -> impl Parser<'a,I,Oo>     { move |i| { self.permutation(i) } }
}

#[cfg(feature = "alt_tuple_32")]
permut_impl!(32);  

#[cfg(all(feature = "alt_tuple_64", not(feature = "alt_tuple_32")))] 
permut_impl!(64);  

#[cfg(not(any(feature = "alt_tuple_32", feature = "alt_tuple_64")))]
permut_impl!(16);  //max 255


/// parser `data end`
pub fn data_end<T>(a:&[T]) -> Result<(&[T],&[T]), &[T]> {
    if !a.is_empty() {  Err(a) } else { Ok((a,a)) }
}

/// parser 'any'
pub fn any<'a,T:'a+Eq+Clone>(pattern: &'a[T]) -> impl Parser<'a,T,&'a[T]> {
    move |input:&'a[T]| { 
        if !input.is_empty() && pattern.contains(&input[0]) { 
            return Ok(split_at_revers(input, 1));
        } 
        Err(input)
    }
}

/// parser 'starts_with'
pub fn starts_with<'a,T:'a+Eq+Clone>(pattern: &'a[T]) -> impl Parser<'a,T,&'a[T]> {
    move |input:&'a[T]| { 
        if input.starts_with(pattern) {
           return  Ok(split_at_revers(input, pattern.len()));
        } 
        Err(input) 
    }
}

///  parser 'starts_with_any'
pub fn starts_with_any<'a,T:'a+Eq+Clone>(patterns: &'a[&'a[T]]) -> impl Parser<'a,T,&'a[T]> {
    move |input:&'a[T]| {
        for i in patterns { 
            if input.starts_with(i) { return  Ok(split_at_revers(input, i.len())); }
        }
        Err(input)
    }
}

/// parser `sequence maximum`
pub fn seq_max<'a,P,T:'a+Eq+Clone>(p: P, count_max:usize) -> impl Parser<'a,T,&'a[T]>
where
    P: Fn(& T) -> bool+Copy+'a,
{
     move |input:&'a[T]| {  
         let mut c:usize = 0;    
         for i in input { if c<count_max&&p(i) {c+=1;} else {break;} }
         if c>0 { Ok(split_at_revers(input, c)) } else { Err(input) }
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
         if c<count_min { Err(input) } else { Ok(split_at_revers(input, c)) } 
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
         if c<range.0 { Err(input) } else { Ok(split_at_revers(input, c)) } 
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
         if c<count_exact { Err(input) } else { Ok(split_at_revers(input, c)) } 
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
         if c<1 { Err(input) } else { Ok(split_at_revers(input, c)) } 
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
            Ok(_) => Err(input),
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

/// combinator map
pub fn map<'a,T:'a,F,P,R1,R2>(parser: P, map_fn: F) -> impl Parser<'a,T,R2>
where
    P: Parser<'a,T,R1>,
    F: Fn((&'a[T],R1)) -> ParseResult<'a,T,R2> + Copy,
{
    move |input| { map_fn(parser.parse(input)?) }
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
            if s.is_ok() { return Err(input); }
            (new_input,_) = take_record(new_input,1).map_err(|_|input)?;
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
            (new_input,_) = take_record(new_input,1).map_err(|_|input)?;
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
            let Ok((next_input2,r)) = p.parse(next_input1) else { break; };
            result.push(r);
            if result.len()==count_max { break; } 
            next_input1 = next_input2;
        }
        if result.is_empty() { Err(input) } else { Ok((next_input1, result)) }
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
            let Ok((next_input2,r)) = p.parse(next_input1) else { break; };
            result.push(r);
            next_input1 = next_input2;
        }
        if result.len()<count_min { Err(input) } else { Ok((next_input1, result)) }
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
            let Ok((next_input2,r)) = p.parse(next_input1) else { break; };
            result.push(r);            
            if result.len()==range.1 { break; }
            next_input1 = next_input2;
        }
        if result.len()<range.0 { Err(input) } else { Ok((next_input1, result)) }
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
            let Ok((next_input2,r)) = p.parse(next_input1) else { break; };
            result.push(r);
            if result.len()==count_exact { break; }
            next_input1 = next_input2;
        }
        if result.len()<count_exact { Err(input) } else { Ok((next_input1, result)) }
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
            let Ok((next_input2,r)) = p.parse(next_input1) else { break; };
            result.push(r);
            next_input1 = next_input2;
        }
        if result.is_empty() { Err(input) } else { Ok((next_input1, result)) }
    }
}

/// combinator `more zero`. Attention! - rezult is always Ok.
pub fn more_zero<'a,T:'a,P,R>(p:P) -> impl Parser<'a,T,Vec<R>>
where
    P: Parser<'a,T,R>,
{   
    move |input: &'a[T]| {
        let mut result = Vec::new();
        let mut next_input1 = input;
        loop {
            let Ok((next_input2,r)) = p.parse(next_input1) else { break; };
            result.push(r);
            next_input1 = next_input2;
        }
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
    map(and_then(more_zero(left(elem,sep)), last_elem.option(), 
        |(mut a,b)| { 
           if let Some(r) = b { a.push(r); }
           a 
        }),
        |(i,r)| { if r.is_empty() { Err(i) } else { Ok((i,r)) } }
    )   
}

/// just usefull function
#[inline]
pub fn split_at_revers<T>(input: &[T], count: usize) -> (&[T], &[T]) {
    (&input[count..], &input[..count])
}

/// just read record
pub fn take_record<T>(b: &[T], l: usize) -> Result<(&[T], &[T]), &[T]> {
	if b.len() < l { return Err(b); }
	Ok(split_at_revers(b, l))
}

/// just useful function
pub fn fflaten<T:Copy>(v:Vec<&[T]>) -> Vec<T> {
    v.into_iter().flatten().copied().collect::<Vec<T>>()
}


