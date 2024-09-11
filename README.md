# parcelona

minimalistic elegance parser combinator library

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

let hex_color = take(seq(is_hex_digit,SeqCount::Exact(2)));
let (input,_) = take(starts_with(b"#")).parse(input).unwrap();
let (input,c) = hex_color.more(NO_ZERO).parse(input).unwrap();
let (r,_) = u8::from_radix_16(c[0]);
let (g,_) = u8::from_radix_16(c[1]);
let (b,_) = u8::from_radix_16(c[2]);
let color = Color{ red:r, green:g, blue:b };

assert_eq!(Color{red: 47, green: 20, blue: 223}, color);
}
```

how to parse utf8 &str ? use crate [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation)


