//! Module containing Bitfield128.

use crate::{
    bitfield::Bitfield,
    iter::Bits,
    prelude::{Bitfield16, Bitfield32, Bitfield64, Bitfield8, BitfieldIndex, Flagenum},
};
use std::{
    collections::BTreeSet,
    fmt::{Binary, Display, LowerHex, Octal, UpperHex},
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
        ShrAssign,
    },
};

type Inner = u128;
type BIndex = BitfieldIndex<Bitfield128>;
const BITS: usize = 128;

/// Bitfield of size 128.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bitfield128(Inner);

impl Bitfield128 {
    #[inline(always)]
    pub fn into_inner(&self) -> Inner {
        self.0
    }
}

impl Bitfield for Bitfield128 {
    const BIT_SIZE: usize = BITS;
    const ONE: Self = Self(1);
    const NONE: Self = Self(Inner::MIN);
    const ALL: Self = Self(Inner::MAX);

    #[inline(always)]
    fn count_ones(&self) -> usize {
        self.0.count_ones() as usize
    }

    #[inline(always)]
    fn count_zeros(&self) -> usize {
        self.0.count_zeros() as usize
    }
}

impl From<Inner> for Bitfield128 {
    #[inline(always)]
    fn from(value: Inner) -> Self {
        Self(value)
    }
}

impl From<Bitfield128> for Inner {
    #[inline(always)]
    fn from(value: Bitfield128) -> Self {
        value.0
    }
}

impl From<BIndex> for Bitfield128 {
    #[inline(always)]
    fn from(value: BIndex) -> Self {
        Self(1) << value
    }
}

impl From<Bitfield8> for Bitfield128 {
    #[inline(always)]
    fn from(value: Bitfield8) -> Self {
        Self(value.into_inner() as Inner)
    }
}

impl From<Bitfield16> for Bitfield128 {
    #[inline(always)]
    fn from(value: Bitfield16) -> Self {
        Self(value.into_inner() as Inner)
    }
}

impl From<Bitfield32> for Bitfield128 {
    #[inline(always)]
    fn from(value: Bitfield32) -> Self {
        Self(value.into_inner() as Inner)
    }
}

impl From<Bitfield64> for Bitfield128 {
    #[inline(always)]
    fn from(value: Bitfield64) -> Self {
        Self(value.into_inner() as Inner)
    }
}

impl Not for Bitfield128 {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAnd for Bitfield128 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitfield128 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for Bitfield128 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitfield128 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXor for Bitfield128 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitfield128 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Shl<BIndex> for Bitfield128 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: BIndex) -> Self::Output {
        Self::from(self.0.shl(rhs.into_inner()))
    }
}

impl ShlAssign<BIndex> for Bitfield128 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: BIndex) {
        *self = self.shl(rhs)
    }
}

impl Shr<BIndex> for Bitfield128 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: BIndex) -> Self::Output {
        Self::from(self.0.shr(rhs.into_inner()))
    }
}

impl ShrAssign<BIndex> for Bitfield128 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: BIndex) {
        *self = self.shr(rhs)
    }
}

impl IntoIterator for Bitfield128 {
    type Item = bool;

    type IntoIter = Bits<Self>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self, BIndex::MIN)
    }
}

impl FromIterator<bool> for Bitfield128 {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut bitfield: Self = Self::from(0);
        for (i, bit) in iter.into_iter().take(BITS).enumerate() {
            bitfield.0 |= (if bit { 1 } else { 0 }) << (i as Inner);
        }
        bitfield
    }
}

impl<A> FromIterator<A> for Bitfield128
where
    A: Flagenum<Bitfield = Self>,
    BIndex: From<A>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut bitfield = Self::NONE;
        let mut seen_indices = BTreeSet::new();

        for e in iter {
            let index = BIndex::from(e);
            if !seen_indices.contains(&index) {
                seen_indices.insert(index);
                bitfield |= Self(1) << index;
            }
        }

        bitfield
    }
}

impl Display for Bitfield128 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0128b}", self.0)
    }
}

impl Binary for Bitfield128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#0130b}", self.0)
    }
}

impl Octal for Bitfield128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#050o}", self.0)
    }
}

impl UpperHex for Bitfield128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#034X}", self.0)
    }
}

impl LowerHex for Bitfield128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#034x}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Tested = Bitfield128;

    #[test]
    fn construction() {
        let bitfield = Tested::new()
            .set(0.try_into().unwrap(), true)
            .check_bit(1.try_into().unwrap())
            .uncheck_bit(0.try_into().unwrap());

        assert_eq!(bitfield, 0b00000010.into());
    }

    #[test]
    fn conversion_from_integer() {
        let bitfield: Tested = 0b10101010.into();

        assert_eq!(bitfield.0, 0b10101010);
    }

    #[test]
    fn conversion_from_index() {
        let bitfield = Tested::from(BitfieldIndex::<Tested>::MIN);

        assert_eq!(bitfield.0, 1);
    }

    #[test]
    fn value() {
        let bitfield: Tested = 0b10101010.into();

        assert_eq!(bitfield.0, bitfield.into_inner());
    }

    #[test]
    fn bit_set_to_true() {
        let mut bitfield: Tested = 0b10101010.into();

        bitfield.set(6.try_into().unwrap(), true);

        assert_eq!(bitfield.0, 0b11101010);
    }

    #[test]
    fn bit_set_to_false() {
        let mut bitfield: Tested = 0b10101010.into();

        bitfield.set(7.try_into().unwrap(), false);

        assert_eq!(bitfield.0, 0b00101010);
    }

    #[test]
    fn get_bit() {
        let bitfield: Tested = 0b10101010.into();

        assert_eq!(bitfield.get(0.try_into().unwrap()), false);
        assert_eq!(bitfield.get(1.try_into().unwrap()), true);
    }

    #[test]
    fn bit_check() {
        let mut bitfield: Tested = 0b10101010.into();

        bitfield.check_bit(6.try_into().unwrap());

        assert_eq!(bitfield.0, 0b11101010);
    }

    #[test]
    fn bit_uncheck() {
        let mut bitfield: Tested = 0b10101010.into();

        bitfield.uncheck_bit(7.try_into().unwrap());

        assert_eq!(bitfield.0, 0b00101010);
    }

    #[test]
    fn count_set() {
        let bitfield: Tested = 0b11100000.into();

        assert_eq!(bitfield.count_ones(), 3);
    }

    #[test]
    fn count_unset() {
        let bitfield: Tested = 0b11100000.into();

        assert_eq!(bitfield.count_zeros(), 125);
    }

    #[test]
    fn shl() {
        let bitfield: Tested = 0b00000001.into();

        assert_eq!(bitfield << 1.try_into().unwrap(), 0b00000010.into());

        let mut bitfield: Tested = 0b00000001.into();
        bitfield <<= 1.try_into().unwrap();

        assert_eq!(bitfield, 0b00000010.into());
    }

    #[test]
    fn shr() {
        let bitfield: Tested = 0b00000010.into();

        assert_eq!(bitfield >> 1.try_into().unwrap(), 0b00000001.into());

        let mut bitfield: Tested = 0b00000010.into();
        bitfield >>= 1.try_into().unwrap();

        assert_eq!(bitfield, 0b00000001.into());
    }

    #[test]
    fn not() {
        let a: Tested = Tested::NONE;

        assert_eq!(!a, Tested::ALL);
    }

    #[test]
    fn bit_and() {
        let a: Tested = 0b11110000.into();
        let b: Tested = 0b11001100.into();

        assert_eq!(a & b, 0b11000000.into());

        let mut a: Tested = 0b11110000.into();
        let b: Tested = 0b11001100.into();
        a &= b;

        assert_eq!(a, 0b11000000.into());
    }

    #[test]
    fn bit_or() {
        let a: Tested = 0b11110000.into();
        let b: Tested = 0b11001100.into();

        assert_eq!(a | b, 0b11111100.into());

        let mut a: Tested = 0b11110000.into();
        let b: Tested = 0b11001100.into();
        a |= b;

        assert_eq!(a, 0b11111100.into());
    }

    #[test]
    fn bit_xor() {
        let a: Tested = 0b11110000.into();
        let b: Tested = 0b11001100.into();

        assert_eq!(a ^ b, 0b00111100.into());

        let mut a: Tested = 0b11110000.into();
        let b: Tested = 0b11001100.into();
        a ^= b;

        assert_eq!(a, 0b00111100.into());
    }

    #[test]
    fn complement() {
        let a: Tested = Tested::NONE;

        assert_eq!(a.complement(), Tested::ALL);
    }

    #[test]
    fn intersection() {
        let a: Tested = 0b11110000.into();
        let b: Tested = 0b11001100.into();

        assert_eq!(a.intersection(b), 0b11000000.into());
    }

    #[test]
    fn union() {
        let a: Tested = 0b11110000.into();
        let b: Tested = 0b11001100.into();

        assert_eq!(a.union(b), 0b11111100.into());
    }

    #[test]
    fn difference() {
        let a: Tested = 0b11110000.into();
        let b: Tested = 0b11001100.into();

        assert_eq!(a.difference(b), 0b00110000.into());
    }

    #[test]
    fn sym_difference() {
        let a: Tested = 0b11110000.into();
        let b: Tested = 0b11001100.into();

        assert_eq!(a.sym_difference(b), 0b00111100.into());
    }

    #[test]
    fn bit_iter() {
        let bitfield: Tested = 0b11110000.into();
        let mut bit_iter = bitfield.bits();

        assert_eq!(bit_iter.next(), Some(false));
        assert_eq!(bit_iter.next(), Some(false));
        assert_eq!(bit_iter.next(), Some(false));
        assert_eq!(bit_iter.next(), Some(false));
        assert_eq!(bit_iter.next(), Some(true));
        assert_eq!(bit_iter.next(), Some(true));
        assert_eq!(bit_iter.next(), Some(true));
        assert_eq!(bit_iter.next(), Some(true));

        for _ in 8..128 {
            assert_eq!(bit_iter.next(), Some(false));
        }

        assert_eq!(bit_iter.next(), None);
    }

    #[test]
    fn collect_from_bit_iter() {
        let a: Tested = 0b11110000.into();
        let bit_iter = a.bits();
        let b: Tested = bit_iter.collect();

        assert_eq!(b, 0b11110000.into());

        let arr = [true, false, true, false, true, false, true, false];
        let bitfield: Tested = arr
            .into_iter()
            // Need to reverse to get the same visual representation, because
            // array's .into_iter() makes iterator from left to right,
            // but .collect() will collect from right to left here.
            .rev()
            .collect();

        assert_eq!(bitfield, 0b10101010.into());
    }

    #[test]
    fn set_pos_iter() {
        let bitfield: Tested = 0b11110000.into();
        let mut set_pos_iter = bitfield.set_indeces();

        assert_eq!(set_pos_iter.next(), Some(4.try_into().unwrap()));
        assert_eq!(set_pos_iter.next(), Some(5.try_into().unwrap()));
        assert_eq!(set_pos_iter.next(), Some(6.try_into().unwrap()));
        assert_eq!(set_pos_iter.next(), Some(7.try_into().unwrap()));
        assert_eq!(set_pos_iter.next(), None);
    }

    #[test]
    fn unset_pos_iter() {
        let bitfield: Tested = 0b11110000.into();
        let mut unset_pos_iter = bitfield.unset_indeces();

        assert_eq!(unset_pos_iter.next(), Some(0.try_into().unwrap()));
        assert_eq!(unset_pos_iter.next(), Some(1.try_into().unwrap()));
        assert_eq!(unset_pos_iter.next(), Some(2.try_into().unwrap()));
        assert_eq!(unset_pos_iter.next(), Some(3.try_into().unwrap()));

        for i in 8..128 {
            assert_eq!(unset_pos_iter.next(), Some(i.try_into().unwrap()));
        }

        assert_eq!(unset_pos_iter.next(), None);
    }

    #[test]
    fn from_slice() {
        // Same index order
        let slice: &[bool] = &[true, false, true, false, true, false, true, false];
        let bitfield: Tested = Tested::from_slice_bool(slice);

        assert_eq!(bitfield, 0b01010101.into());
    }

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Tested>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Tested>();
    }
}
