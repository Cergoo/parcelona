# parcelona

minimalistic elegant parser combinators library
- full zero copy
- parsing over &[T] 
- no have partial parsing


```rust
#[test]
fn t_color() {
use atoi::FromRadix16;

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
```

how to parse utf8 &str ? use crate [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation)

## doc
[ru](https://github.com/Cergoo/parcelona/tree/main/doc/doc_ru.md) 

### parser_combinator
This core of library and has parsers:
- `data_end`
- `any`
- `starts_with`
- `starts_with_any`
- `take`
- `seq`
- `seq_exact`
- `seq_max`
- `seq_min`
- `seq_range`
- `seq_ext`
- `ClassOfSymbols`
- `StaticClassOfSymbols`

and has many parser combinators:
- `not (parser)`
- `map (parser,Fn)`
- `fmap (parser,Fn)`
- `frmap (parser,Fn)`
- `option (parser)`
- `pair (parser,parser)`
- `or (parser,parser)`
- `left (parser,parser)`
- `right (parser,parser)`
- `left_opt (parser,parser)`
- `right_opt (parser,parser)`
- `more (parser)`
- `more_max (parser,usize)`
- `more_min (parser,usize)`
- `more_exact (parser,usize)`
- `more_range (parser,(usize,usize))`
- `alt ((tuple of parsers))`
- `permut ((tuple of parsers))`
- `permut_part ((tuple of parsers))`
- `find_stop (parser,parser)`
- `find (parser)`
- `sep_pair (parser,parser,parser)`
- `sep_pair_opt (parser,parser,parser)`
- `between (parser,parser,parser)`
- `between_opt (parser,parser,parser)`
- `and_then (parser,parser,Fn)`
- `or_then (parser,parser,Fn)`
- `sep_list (parser,parser,parser)`

### u8
This functions for `u8`

### examples
- see `examples`, cargo run --example parse_tag
- [take_sni](https://github.com/Cergoo/take_sni) tls sni hand shake parsing
