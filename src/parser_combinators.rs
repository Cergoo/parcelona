//!  Parcelona minimalistic elegance parser combinator library.
//!

pub type ParseResult<'a,I,O> = std::result::Result<(&'a [I],O),&'a [I]>;
 
/// Main parser definition
pub trait Parser<'a,I,O> {
    fn parse(&self, input:&'a [I]) -> ParseResult<'a,I,O>;
    fn option(self) -> impl Parser<'a,I,Option<O>> + Clone;
    fn more(self,no_zero:bool) -> impl Parser<'a,I,Vec<O>> + Clone;    
}

impl<'a,I:'a,F,O> Parser<'a,I,O> for F
where
    F: Fn(&'a[I]) -> ParseResult<'a,I,O> + Clone,
{
    fn parse(&self, input:&'a [I]) -> ParseResult<'a,I,O> {  self(input)  }
    fn option(self) -> impl Parser<'a,I,Option<O>>+Clone { option(self) }
    fn more(self,no_zero:bool) -> impl Parser<'a,I,Vec<O>>+Clone { more(self,no_zero) }
}

/// Alt trait combinator
pub trait Alt<'a,I:'a,O> {
    fn choice(&self, input:&'a [I]) -> ParseResult<'a,I,O>;
}

impl<'a,I:'a,O,P1,P2> Alt<'a,I,O> for (P1,P2)
where
    P1: Parser<'a,I,O>,
    P2: Parser<'a,I,O>,
   // I: Clone,
{
    fn choice(&self, input: &'a[I]) -> ParseResult<'a,I,O> {
        self.0.parse(input).or(self.1.parse(input))
    }
}

impl<'a,I:'a,O,P1,P2,P3> Alt<'a,I,O> for (P1,P2,P3)
where
    P1: Parser<'a,I,O>,
    P2: Parser<'a,I,O>,
    P3: Parser<'a,I,O>,
{
    fn choice(&self, input: &'a[I]) -> ParseResult<'a,I,O> {
        self.0.parse(input).or(self.1.parse(input)).or(self.2.parse(input))
    }
}












/// parser 'take', this is firs parser, but it can be parameterized by functions.
pub fn take<'a,T,P>(predicat: P) -> impl Parser<'a,T,&'a[T]>+Clone
where
     T: 'a,
     P: Fn(&'a[T]) -> usize + Clone,
{     
    move |input: &'a[T]| {
        let i = predicat(input);
        if i>0 { return Ok(split_at_revers(input, i)); }
        Err(input)
    }    
}

/// parser `data end` this is second parser, it detect end of data.
/// there are no other parsers, only `take` and `data_end` 
pub fn data_end<T>(a:&[T]) -> Result<(&[T],&[T]), &[T]> {
    if a.len() > 0 {  Err(a) } else { Ok((a,a)) }
}

/// function 'any' for parametrize parser `take`
pub fn any<'a,T:'a+Eq+Clone>(pattern: &'a[T]) -> impl Fn(&'a[T]) -> usize+'a+Clone {
    |input| { if input.len()>0 && pattern.contains(&input[0]) { 1 } else { 0 } }
}

/// function 'starts_with' for parametrize parser `take`
pub fn starts_with<'a,T:'a+Eq+Clone>(pattern: &'a[T]) -> impl Fn(&'a[T]) -> usize+'a+Clone {
    |input| { if input.starts_with(pattern) { pattern.len() } else { 0 } }
}

/// function 'starts_with_any' for parametrize parser `take`
pub fn starts_with_any<'a,T:'a+Eq+Clone>(pattern: &'a[&'a[T]]) -> impl Fn(&'a[T]) -> usize+'a+Clone {
    move |input| {
        for i in pattern {  if input.starts_with(i) { return pattern.len() }; };
        0
    }
}

/// enum for 'seq' function
#[derive(Clone)]
 pub enum SeqCount {
    Max(usize),
    Exact(usize),
    Range((usize,usize)),
    None,    
}

/// function 'seq'-sequence for parametrize parser `take`
pub fn seq<'a,P,T:'a+Eq+Clone>(p: P, count:SeqCount) -> impl Fn(&'a[T]) -> usize+'a+Clone
where
    P: Fn(& T) -> bool+Clone+'a,
{ move |input| {
    let mut c:usize = 0;
    match count {
        SeqCount::None        =>   for i in input { if p(i)      {c+=1;} else {break;} },
        SeqCount::Max(x)      =>   for i in input { if c<x&&p(i) {c+=1;} else {break;} },
        SeqCount::Exact(x)    => { for i in input { if c<x&&p(i) {c+=1;} else {break;} } if c!=x {c=0;}; },
        SeqCount::Range((x,y))=> { for i in input { if c<y&&p(i) {c+=1;} else {break;} } if c<x  {c=0;}; },
    }
    c
}}

/// `not` closur for `seq` a func parametr. not(predicat)
pub fn not<T>(f: impl Fn(& T) -> bool+Clone) -> impl Fn(& T) -> bool+Clone 
{ move |x| !f(x) }




/// combinators

/// combinator not(parser)
pub fn notp<'a,T:'a,P,R>(parser: P) -> impl Parser<'a,T,()>+Clone
where
    P: Parser<'a,T,R>+Clone,
{
    move |input| {
        match parser.parse(input) {
            Ok(_) => Err(input),
            _     => Ok((input,())),
    }}
}

/// combinator map
pub fn map<'a,T:'a,F,P,R1,R2>(parser: P, map_fn: F) -> impl Parser<'a,T,R2>+Clone
where
    P: Parser<'a,T,R1>+Clone,
    F: Fn(R1) -> R2+Clone,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

/// combinator option - allways return Ok, no Err
pub fn option<'a,T:'a,P,R>(parser: P) -> impl Parser<'a,T,Option<R>>+Clone
where
    P: Parser<'a,T,R>+Clone,
{
    move |input| {  
        match parser.parse(input) {
            Ok((input,r)) => Ok((input,Some(r))),
            _             => Ok((input,None))   
    }}
}

/// combinator pair
pub fn pair<'a,T:'a,P1,P2,R1,R2>(p1:P1,p2:P2) -> impl Parser<'a,T,(R1,R2)>+Clone
where
    P1: Parser<'a,T,R1>+Clone,
    P2: Parser<'a,T,R2>+Clone,
{
    move |input| {
        p1.parse(input).and_then(|(next_input,r1)| { 
        p2.parse(next_input).map(|(next_input,r2)| (next_input,(r1,r2))) })
    }
}

/// combinator left
pub fn left<'a,T:'a,P1,P2,R1,R2>(p1:P1,p2:P2) -> impl Parser<'a,T,R1>+Clone
where
    P1: Parser<'a,T,R1>+Clone,
    P2: Parser<'a,T,R2>+Clone,
{
    map(pair(p1,p2),|(l,_)|l)
}

/// combinator right
pub fn right<'a,T:'a,P1,P2,R1,R2>(p1:P1,p2:P2) -> impl Parser<'a,T,R2>+Clone
where
    P1: Parser<'a,T,R1>+Clone,
    P2: Parser<'a,T,R2>+Clone,
{
    map(pair(p1,p2),|(_,r)|r)
}

/// combinator right 'left'-is options, if left returns Error it is ignored
pub fn right_opt<'a,T:'a,P1,P2,R1,R2>(p1:P1,p2:P2) -> impl Parser<'a,T,R2>+Clone
where
    P1: Parser<'a,T,R1>+Clone,
    P2: Parser<'a,T,R2>+Clone,
{
    move |input| {
        if let Ok((input,_)) = p1.parse(input) { p2.parse(input) } 
        else { p2.parse(input) }
    }
}
 
/// combinator left 'right'-is options, if right returns Error it is ignored
pub fn left_opt<'a,T:'a,P1,P2,R1,R2>(p1:P1,p2:P2) -> impl Parser<'a,T,R1>+Clone
where
    P1: Parser<'a,T,R1>+Clone,
    P2: Parser<'a,T,R2>+Clone,
{
    move |input| {
        p1.parse(input).map(|(next_input, r1)| {
            match p2.parse(next_input) {
                Ok((next_input,_)) => (next_input,r1),
                _                  => (next_input,r1),
            }    
        })
    }
}

/// combinator find, be careful when choosing a parser 'step', in most cases it
/// should be a one step parser.
pub fn find<'a,T:'a,P1,P2,R1,R2>(step:P1,p:P2) -> impl Parser<'a,T,R2>+Clone
where
    P1: Parser<'a,T,R1>+Clone,
    P2: Parser<'a,T,R2>+Clone,
{
    move |input: &'a[T]| {
        let mut next_input1 = input;
        while let Ok((next_input2,_)) = step.parse(next_input1) {
            let r = p.parse(next_input1);
            if r.is_ok() { return r; }
            next_input1 = next_input2;
        }     
        Err(input)
    }
}

/// const for write more readable parsers
pub const NO_ZERO:bool = true;
pub const ZERO:bool    = false;

/// combinator more
pub fn more<'a,T:'a,P,R>(p:P,no_zero:bool) -> impl Parser<'a,T,Vec<R>>+Clone
where
    P: Parser<'a,T,R>+Clone,
{
    move |mut input: &'a[T]| {
        let mut result = Vec::new();
        while let Ok((next_input,item)) = p.parse(input) {
            input = next_input;
            result.push(item);
        }
        if no_zero && result.is_empty() { Err(input) }
        else { Ok((input,result)) }
    }
}

/// alt combinator
pub fn alt<'a,I:'a,O,T:Alt<'a,I,O>+Clone>(input: T) -> impl Parser<'a,I,O>+Clone {
    move |i| input.choice(i)
}

/// combinator separated pair
pub fn sep_pair<'a,T:'a,P1,P_,P2,R1,R2,R_>(p1:P1,sep:P_,p2:P2) -> impl Parser<'a,T,(R1,R2)>+Clone
where
    P1: Parser<'a,T,R1>+Clone,
    P2: Parser<'a,T,R2>+Clone,
    P_: Parser<'a,T,R_>+Clone,
{
    pair(left(p1,sep),p2)   
}

/// combinator element between
pub fn between<'a,T:'a,P1,P,P2,R1,R,R2>(p1:P1,p:P,p2:P2) -> impl Parser<'a,T,R>+Clone
where
    P1: Parser<'a,T,R1>+Clone,
    P: Parser<'a,T,R>+Clone,
    P2: Parser<'a,T,R2>+Clone,
{
    left(right(p1,p),p2)   
}

/// combinator element between optional
pub fn between_opt<'a,T:'a,P1,P,P2,R1,R,R2>(p1:P1,p:P,p2:P2) -> impl Parser<'a,T,R>+Clone
where
    P1: Parser<'a,T,R1>+Clone,
    P: Parser<'a,T,R>+Clone,
    P2: Parser<'a,T,R2>+Clone,
{
    left_opt(right_opt(p1,p),p2)   
}

/// combinator and_then
pub fn and_then<'a,T:'a,P1,P2,R1,R2,F,R3>(p1:P1,p2:P2,f:F) -> impl Parser<'a,T,R3>+Clone
where
    P1: Parser<'a,T,R1>+Clone,
    P2: Parser<'a,T,R2>+Clone,
    F: Fn((R1,R2)) -> R3+Clone,
{
    map(pair(p1,p2),f)
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
pub fn sep_list<'a,T:'a,Pe,Re,Ps,Rs,Ple>(elem:Pe,sep:Ps,last_elem:Ple) -> impl Parser<'a,T,Vec<Re>>+Clone
where
    Pe:  Parser<'a,T,Re>+Clone,
    Ps:  Parser<'a,T,Rs>+Clone,
    Ple: Parser<'a,T,Re>+Clone,
{
    and_then(
        more(left(elem,sep),ZERO), 
        last_elem.option(), 
        |(mut a,b)| {
            match b {
                Some(x) => {a.push(x); a},
                None    => a,
        }})   
}

/// just usefull function
#[inline]
pub fn split_at_revers<'a, T>(input: &'a [T], count: usize) -> (&'a [T], &'a [T]) {
    (&input[count..], &input[..count])
}

/// fust useful function
pub fn fflaten<T:Copy>(v:Vec<&[T]>) -> Vec<T> {
    v.into_iter().flatten().map(|x|*x).collect::<Vec<T>>()
}


// just drop all parser result
fn drop<I>(_: I) -> () {
    ()
}


