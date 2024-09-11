//! these functions DO NOT PANIC

use crate::parser_combinators::split_at_revers;
use byteorder::{ByteOrder, BE};


// read only len Big Endian
pub fn take_len_be_u8(b: &[u8]) -> std::result::Result<(&[u8], usize), &[u8]> {
	if b.len() < 1 { return Err(b); }
	Ok((&b[1..], b[0] as usize))
}

// read only len Big Endian
pub fn take_len_be_u16(b: &[u8]) -> std::result::Result<(&[u8], usize), &[u8]> {
	if b.len() < 2 { return Err(b); }
	let (new_b, c) = split_at_revers(b, 2);
	Ok((new_b, BE::read_u16(c) as usize))
}

// read only len Big Endian
pub fn take_len_be_u24(b: &[u8]) -> std::result::Result<(&[u8], usize), &[u8]> {
	if b.len() < 3 { return Err(b); }
	let (new_b, c) = split_at_revers(b, 3);
	Ok((new_b, BE::read_u24(c) as usize))
}

// read only len Big Endian
pub fn take_len_be_u32(b: &[u8]) -> std::result::Result<(&[u8], usize), &[u8]> {
	if b.len() < 4 { return Err(b); }
	let (new_b, c) = split_at_revers(b, 4);
	Ok((new_b, BE::read_u32(c) as usize))
}

// read only len Big Endian
pub fn take_len_be_u48(b: &[u8]) -> std::result::Result<(&[u8], usize), &[u8]> {
	if b.len() < 6 { return Err(b); }
	let (new_b, c) = split_at_revers(b, 6);
	Ok((new_b, BE::read_u48(c) as usize))
}

// read only len Big Endian
pub fn take_len_be_u64(b: &[u8]) -> std::result::Result<(&[u8], usize), &[u8]> {
	if b.len() < 8 { return Err(b); }
	let (new_b, c) = split_at_revers(b, 8);
	Ok((new_b, BE::read_u64(c) as usize))
}

/// read record Big Endian
pub fn take_record(b: &[u8], l: usize) -> std::result::Result<(&[u8], &[u8]), &[u8]> {
	if b.len() < l { return Err(b); }
	Ok(split_at_revers(b, l))
}


/// read record Big Endian
pub fn take_record_be_u8(b: &[u8]) -> std::result::Result<(&[u8], &[u8]), &[u8]> {
	if b.len() < 2 { return Err(b); }
	if b.len() < b[0] as usize { return Err(b); }
	Ok(split_at_revers(&b[1..], b[0] as usize))
}

/// read record Big Endian
pub fn take_record_be_u16(b: &[u8]) -> std::result::Result<(&[u8], &[u8]), &[u8]> {
	let (new_b, l) = take_len_be_u16(b)?;
	take_record(new_b, l)
}

pub fn take_record_be_u24(b: &[u8]) -> std::result::Result<(&[u8], &[u8]), &[u8]> {
	let (new_b, l) = take_len_be_u24(b)?;
	take_record(new_b, l)
}

pub fn take_record_be_u32(b: &[u8]) -> std::result::Result<(&[u8], &[u8]), &[u8]> {
	let (new_b, l) = take_len_be_u32(b)?;
	take_record(new_b, l)
}

pub fn take_record_be_u48(b: &[u8]) -> std::result::Result<(&[u8], &[u8]), &[u8]> {
	let (new_b, l) = take_len_be_u48(b)?;
	take_record(new_b, l)
}

pub fn take_record_be_u64(b: &[u8]) -> std::result::Result<(&[u8], &[u8]), &[u8]> {
	let (new_b, l) = take_len_be_u64(b)?;
	take_record(new_b, l)
}
