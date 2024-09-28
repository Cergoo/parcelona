//! u8 implementation
//! all these functions DO NOT PANIC

use crate::parser_combinators::{*};
use byteorder::{ByteOrder, BE};
use std::result::Result;
pub use crate::parser_combinators::take_record;

 
pub const SPACE:(u8,u8)          = (0,32);
pub const NO_SPACE:(u8,u8)       = (33,255);
pub const ALPHA_UPPER:(u8,u8)    = (65,90);
pub const ALPHA_LOWER:(u8,u8)    = (97,122);
pub const DEC_DIGIT:(u8,u8)      = (48,57);
pub const OCT_DIGIT:(u8,u8)      = (48,55);
pub const HEX_DIGIT:&[(u8,u8)]   = &[DEC_DIGIT, (65,70), (97,102)];
pub const ALPHA_NUM:&[(u8,u8)]   = &[DEC_DIGIT, ALPHA_UPPER, ALPHA_LOWER];


pub const EOL: u8 = 10;  
pub const EOL2: [u8; 2] = [13,10];   //  \r\n

#[inline]
pub fn is_eol(i:&u8) -> bool { EOL2.contains(i) }

#[inline]
pub fn is_no_eol(i:&u8) -> bool { !EOL2.contains(i) }

#[inline]
pub fn is_space(i:&u8) -> bool { *i < 33 }

#[inline]
pub fn is_tab(i:&u8) -> bool { *i == 9 }

#[inline]
pub fn is_space_noeol(i:&u8) -> bool { !(is_eol(i)) && is_space(i) }

#[inline]
pub fn is_alpha(i:&u8) -> bool { (*i >= 0x41 && *i <= 0x5A) || (*i >= 0x61 && *i <= 0x7A) }

#[inline]
pub fn is_alpha_upper(i:&u8) -> bool { *i >= 65 && *i <= 90 }

#[inline]
pub fn is_alpha_lower(i:&u8) -> bool { *i >= 97 && *i <= 122 }

#[inline]
pub fn is_alphanum(i:&u8) -> bool { is_alpha(i) || is_dec_digit(i) }

#[inline]
pub fn is_dec_digit(i:&u8) -> bool { *i >= 48 && *i <= 57 }

#[inline]
pub fn is_hex_digit(i:&u8) -> bool {
    (*i >= 0x30 && *i <= 0x39) || (*i >= 0x41 && *i <= 0x46) || (*i >= 0x61 && *i <= 0x66)
}

#[inline]
pub fn is_oct_digit(i:&u8) -> bool { *i >= 0x30 && *i <= 0x37 }

/// Turns uppercase into lowercase, but also modifies '@' and '<'..='_' if not check input
#[inline]
pub fn to_lowercase(a: u8) -> u8 { if is_alpha_upper(&a) { a | 0b010_0000 } else { a } }

#[inline]
pub fn to_upperrcase(a: u8) -> u8 { if is_alpha_lower(&a) { a - 32 } else { a } }






// read only len Big Endian
pub fn take_len_be_u8(b: &[u8]) -> Result<(&[u8], usize), PErr<u8>> {
    if b.is_empty() { return Err(PErr::new(b)); }
    Ok((&b[1..], b[0] as usize))
}

// read only len Big Endian
pub fn take_len_be_u16(b: &[u8]) -> Result<(&[u8], usize), PErr<u8>> {
	if b.len() < 2 { return Err(PErr::new(b)); }
	let (new_b, c) = split_at_revers(b, 2);
	Ok((new_b, BE::read_u16(c) as usize))
}

// read only len Big Endian
pub fn take_len_be_u24(b: &[u8]) -> Result<(&[u8], usize), PErr<u8>> {
	if b.len() < 3 { return Err(PErr::new(b)); }
	let (new_b, c) = split_at_revers(b, 3);
	Ok((new_b, BE::read_u24(c) as usize))
}

// read only len Big Endian
pub fn take_len_be_u32(b: &[u8]) -> Result<(&[u8], usize), PErr<u8>> {
	if b.len() < 4 { return Err(PErr::new(b)); }
	let (new_b, c) = split_at_revers(b, 4);
	Ok((new_b, BE::read_u32(c) as usize))
}

// read only len Big Endian
pub fn take_len_be_u48(b: &[u8]) -> Result<(&[u8], usize), PErr<u8>> {
	if b.len() < 6 { return Err(PErr::new(b)); }
	let (new_b, c) = split_at_revers(b, 6);
	Ok((new_b, BE::read_u48(c) as usize))
}

// read only len Big Endian
pub fn take_len_be_u64(b: &[u8]) -> Result<(&[u8], usize), PErr<u8>> {
	if b.len() < 8 { return Err(PErr::new(b)); }
	let (new_b, c) = split_at_revers(b, 8);
	Ok((new_b, BE::read_u64(c) as usize))
}

/// read record Big Endian
pub fn take_record_be_u8(b: &[u8]) -> Result<(&[u8], &[u8]), PErr<u8>> {
	if b.len() < 2 { return Err(PErr::new(b)); }
	if b.len() < b[0] as usize { return Err(PErr::new(b)); }
	Ok(split_at_revers(&b[1..], b[0] as usize))
}

/// read record Big Endian
pub fn take_record_be_u16(b: &[u8]) -> Result<(&[u8], &[u8]), PErr<u8>> {
	let (new_b, l) = take_len_be_u16(b)?;
	take_record(new_b, l)
}

pub fn take_record_be_u24(b: &[u8]) -> Result<(&[u8], &[u8]), PErr<u8>> {
	let (new_b, l) = take_len_be_u24(b)?;
	take_record(new_b, l)
}

pub fn take_record_be_u32(b: &[u8]) -> Result<(&[u8], &[u8]), PErr<u8>> {
	let (new_b, l) = take_len_be_u32(b)?;
	take_record(new_b, l)
}

pub fn take_record_be_u48(b: &[u8]) -> Result<(&[u8], &[u8]), PErr<u8>> {
	let (new_b, l) = take_len_be_u48(b)?;
	take_record(new_b, l)
}

pub fn take_record_be_u64(b: &[u8]) -> Result<(&[u8], &[u8]), PErr<u8>> {
	let (new_b, l) = take_len_be_u64(b)?;
	take_record(new_b, l)
}

/// the most common variant of separated list of &[u8],clear space enable
pub fn sep_list_common<'a,Pe,Re,Ps,Rs>(elem:Pe,sep:Ps) -> impl Parser<'a,u8,Vec<Re>>
where
	Pe:  Parser<'a,u8,Re>,
	Ps:  Parser<'a,u8,Rs>,
{   
	let space = seq(is_space);
	sep_list( 
	between_opt(space,elem,space),
	sep,
	left(right_opt(space,elem), alt((space,data_end))),
	)
}
