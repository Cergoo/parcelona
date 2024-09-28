extern crate parcelona;
use parcelona::parser_combinators::{*};
use parcelona::u8::{*};
use std::str::{from_utf8, Utf8Error};
use std::error::Error;
use std::default::Default;


const DATA: &[u8] = 
br#"<poet author="Byron" title="The Girl of Cadiz" date="1809">


Oh never talk again to me
Of northern climes and British ladies;
It has not been your lot to see,
Like me, the lovely Girl of Cadiz.
Although her eye be not of blue,
Nor fair her locks, like English lasses,
How far its own expressive hue
The languid azure eye surpasses!


</poet>"#;  

#[derive(Debug)]
struct Tag<'a> {
	name: &'a str,
	attributes: Vec<(&'a str,&'a str)>,
	text: &'a str,
}

const OPEN_TAG_NOTFOUND:  &str = r#""<" opent tag not found"#;
const CLOSE_TAG_NOTFOUND: &str = r#"">" close tag not found"#;
const NAME_TAG_NOTFOUND:  &str = r#"name tag not found"#;
const SEP_NOTFOUND:       &str = r#""=" not found"#;
const END_TAG_NOTFOUND:   &str = r#"end tag not found"#;

fn parse_tag(input: &[u8]) -> Result<Tag, Box<dyn Error + '_>> {

    let mut name: ClassOfSymbols<u8> = Default::default();
    name.range_enable_push(ALPHA_NUM)
        .one_enable_push(&[45,46,95]); // - . _

    let mut value: ClassOfSymbols<u8> = Default::default();
    value.one_disable_push(&[34]) // "
		.default_enable_one(true);

    let mut text: ClassOfSymbols<u8> = Default::default();
    text.one_disable_push(br#"<>\"#)  // <
		.parts_enable_push(&[br#"\\"#, br#"\<"#, br#"\>"#])
		.default_enable_one(true);   // if iten of slice is not disable then is enable

	let space  = seq(is_space);
	let open   = between_opt(space, starts_with(b"<"), space).msg_err(OPEN_TAG_NOTFOUND);
	let close  = between_opt(space, starts_with(b">"), space).msg_err(CLOSE_TAG_NOTFOUND);
	let sep    = starts_with(b"=").msg_err(SEP_NOTFOUND);
	let quotes = between_opt(space, starts_with(b"\""), space);
	let name_parser  = between_opt(space, &name, space);
	let value_parser = between(quotes, value.msg_err("value parse error_1"), quotes).msg_err("value parse error");

	let attrs = frmap(sep_pair(name_parser, sep, value_parser),|x|{Ok::<(&str, &str), Utf8Error>((from_utf8(x.0)?, from_utf8(x.1)?))})
		.msg_err("pars attr error")
		.more()
		.msg_err("pars attr more eror");

	let (input, (tag_name, tag_attrs)) = between(open, pair(name_parser, attrs), close)
		.msg_err("first line pars eror")
		.strerr()
		.parse(input)?;

	let (input, tag_text) = fmap(text.msg_err("text parse error").strerr(), <[u8]>::trim_ascii).parse(input)?;

	let _ = between(open, pair(any(b"/"), starts_with(tag_name)), close)
		.msg_err(END_TAG_NOTFOUND)
		.strerr()
		.parse(input)?;

	Ok(Tag {
		name: from_utf8(tag_name)?,
		attributes: tag_attrs,
		text: from_utf8(tag_text)?,
	})
}


fn main() {
	let r = parse_tag(DATA);
	match r {
		Ok(r)  => println!("{:#?}", r),
		Err(r) => println!("{}", r),
	}
}
