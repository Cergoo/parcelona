//! u8 implementation

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

pub const ALPHA_UPPER:(u8,u8) = (65,90);
pub const ALPHA_LOWER:(u8,u8) = (97,122);
pub const DEC_DIGIT:(u8,u8)   = (48,57);


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