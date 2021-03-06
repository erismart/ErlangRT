//!
//! Defines header tag values used on heap to define the header type.
//!
use bit_field::BitField;

use defs;
use defs::Word;
use term::primary;
use term::primary::PRIM_VALUE_FIRST;


/// Bit position for the header tag following the primary tag.
const HEADER_TAG_FIRST: u8 = PRIM_VALUE_FIRST;
const HEADER_TAG_LAST: u8 = HEADER_TAG_FIRST + 4;


/// Bit position for the value after the primary and the header tag.
const HEADER_VALUE_FIRST: u8 = HEADER_TAG_LAST;
const HEADER_VALUE_LAST: u8 = defs::WORD_BITS as u8;


// 0?
/// TAG_HEADER* marks the type, which follows the header word.
pub const TAG_HEADER_TUPLE: Word = 1;
pub const TAG_HEADER_BIGNEG: Word = 2;
pub const TAG_HEADER_BIGPOS: Word = 3;
pub const TAG_HEADER_REF: Word = 4;
pub const TAG_HEADER_FUN: Word = 5;
pub const TAG_HEADER_FLOAT: Word = 6;
pub const TAG_HEADER_EXPORT: Word = 7;
pub const TAG_HEADER_REFCBIN: Word = 8;
pub const TAG_HEADER_HEAPBIN: Word = 9;
pub const TAG_HEADER_SUBBIN: Word = 10;
// 11?
pub const TAG_HEADER_EXTPID: Word = 12;
pub const TAG_HEADER_EXTPORT: Word = 13;
pub const TAG_HEADER_EXTREF: Word = 14;

const HEADER_TAG_TUPLE_RAW: Word = (primary::TAG_HEADER as Word)
      | (TAG_HEADER_TUPLE << HEADER_TAG_FIRST);

const HEADER_TAG_BIGNEG_RAW: Word = (primary::TAG_HEADER as Word)
    | (TAG_HEADER_BIGNEG << HEADER_TAG_FIRST);

const HEADER_TAG_BIGPOS_RAW: Word = (primary::TAG_HEADER as Word)
    | (TAG_HEADER_BIGPOS << HEADER_TAG_FIRST);


#[inline]
fn make_header_raw(val: Word, premade: Word) -> Word {
  let mut premade1 = premade;
  *premade1.set_bits(HEADER_VALUE_FIRST..HEADER_VALUE_LAST, val)
}


#[inline]
pub fn make_tuple_header_raw(sz: Word) -> Word {
  make_header_raw(sz, HEADER_TAG_TUPLE_RAW)
}


#[inline]
pub fn make_bignum_neg_header_raw(sz: Word) -> Word {
  make_header_raw(sz, HEADER_TAG_BIGNEG_RAW)
}


#[inline]
pub fn make_bignum_pos_header_raw(sz: Word) -> Word {
  make_header_raw(sz, HEADER_TAG_BIGPOS_RAW)
}

#[inline]
pub fn get_tag(v: Word) -> Word {
  assert_eq!(primary::get_tag(v), primary::TAG_HEADER);
  v.get_bits(HEADER_TAG_FIRST..HEADER_TAG_LAST) as Word
}

#[inline]
pub fn get_arity(v: Word) -> Word {
  assert_eq!(primary::get_tag(v), primary::TAG_HEADER);
  v.get_bits(HEADER_VALUE_FIRST..HEADER_VALUE_LAST)
}
