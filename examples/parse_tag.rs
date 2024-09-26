extern crate parcelona;
use parcelona::parser_combinators::{*};
use parcelona::u8::{*};
use std::str::from_utf8;
use std::error::Error;

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
	params: Vec<(&'a str,&'a str)>,
	text: &'a str,
}

fn name_symbols(i:&u8) -> bool { 
	*i == 45 || *i == 46 || *i == 95 || is_alphanum(i) 
}

// allowed character classes
fn text_symbols(i:&[u8]) -> usize {
	// one class
	let smbl: &[u8] = &[92, 60, 62];   // \, <, > 
	if !check_starts_with_any_element(smbl, i) { return 1; }
  // second class
	let escaped_smbl: &[&[u8]] = &[br#"\\"#, br#"\<"#, br#"\>"#,]; 
	let c = check_starts_with_any_part(escaped_smbl, i);
	c
} 

// allowed character classes
fn value_symbols(i:&[u8]) -> usize {
	// one class
	let smbl: &[u8] = &[34];   // \, <, > 
	if !check_starts_with_any_element(smbl, i) { return 1; }
  0
} 
  

const OPEN_TAG_NOTFOUND:  &str = r#""<" opent tag not found"#;
const CLOSE_TAG_NOTFOUND: &str = r#"">" close tag not found"#;
const NAME_TAG_NOTFOUND:  &str = r#"name tag not found"#;
const SEP_NOTFOUND:       &str = r#""=" not found"#;
const END_TAG_NOTFOUND:   &str = r#"end tag not found"#;

fn parse_tag(input: &[u8]) -> Result<Tag, Box<dyn Error + '_>> {

	let space = seq(is_space);
	let text = fmap(seq_ext(text_symbols), <[u8]>::trim_ascii);
	let p_val = fmap(seq_ext(value_symbols), <[u8]>::trim_ascii).msg_err("error parse value of paramet");

	let open  = starts_with(b"<").msg_err(OPEN_TAG_NOTFOUND);
	let close = starts_with(b">").msg_err(CLOSE_TAG_NOTFOUND);
	let sep   = starts_with(b"=").msg_err(SEP_NOTFOUND);
	let name  = between_opt(space, seq(name_symbols).msg_err(NAME_TAG_NOTFOUND), space);
	let speech_mark  = starts_with(b"\"");
	let param_value  = between(speech_mark, p_val, speech_mark).msg_err("err pars param value");
	let params = sep_pair(name, sep, param_value).msg_err("err pars params").more().msg_err("err pars params more");

   
	let (input, (t_name,t_params)) = between(open, pair(name, params.msg_err("err pars params1")).msg_err("err pars pair"), close).msg_err("err pars first line").strerr().parse(input)?;
	let (input, t_text) = text.parse(input)?;
	let _ = between(open,between_opt(space, pair(any(b"/"), starts_with(t_name)), space),close)
					.msg_err(END_TAG_NOTFOUND)
					.strerr()
					.parse(input)?;

	let mut t_params_true = Vec::<(&str,&str)>::new(); 
	for i in t_params {
		t_params_true.push((from_utf8(i.0)?, from_utf8(i.1)?));
	}

	Ok(Tag {
		name: from_utf8(t_name)?,
		params: t_params_true,
		text: from_utf8(t_text)?,
	})
}


fn main() {

	let r = parse_tag(DATA);

	match r {
		Ok(r)  => println!("{:#?}", r),
		Err(r) => println!("{}", r),
	}
	

}
