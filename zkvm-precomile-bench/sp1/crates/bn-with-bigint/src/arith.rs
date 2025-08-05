use alloc::vec::Vec;
use crunchy::unroll;
use crypto_bigint::U256;

/// U256/U512 errors
#[derive(Debug)]
pub enum Error {
    InvalidLength { expected: usize, actual: usize },
}

pub struct BitIterator<'a> {
    int: &'a U256,
    n: usize,
}

pub fn u256_bits(int: &U256) -> BitIterator {
    BitIterator { int, n: 256 }
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.n == 0 {
            None
        } else {
            self.n -= 1;

            u256_get_bit(self.int, self.n)
        }
    }
}

/// Divide by two
#[inline]
fn div2(inp: &mut U256) {
    let mut a = from_word_vec(&inp.to_words());

    let tmp = a[1] << 127;
    a[1] >>= 1;
    a[0] >>= 1;
    a[0] |= tmp;

    let res = to_word_vec(&a);
    *inp = U256::from_words(res);
}

/// Multiply by two
#[inline]
pub(crate) fn mul2(inp: &mut U256) {
    let mut a = from_word_vec(&inp.to_words());
    let tmp = a[0] >> 127;
    a[0] <<= 1;
    a[1] <<= 1;
    a[1] |= tmp;

    let res = to_word_vec(&a);
    *inp = U256::from_words(res);
}

#[inline(always)]
pub(crate) fn split_u128(i: u128) -> (u128, u128) {
    (i >> 64, i & 0xFFFFFFFFFFFFFFFF)
}

#[inline(always)]
pub(crate) fn combine_u128(hi: u128, lo: u128) -> u128 {
    (hi << 64) | lo
}

#[inline]
pub(crate) fn adc(a: u128, b: u128, carry: &mut u128) -> u128 {
    let (a1, a0) = split_u128(a);
    let (b1, b0) = split_u128(b);
    let (c, r0) = split_u128(a0 + b0 + *carry);
    let (c, r1) = split_u128(a1 + b1 + c);
    *carry = c;

    combine_u128(r1, r0)
}

#[inline]
fn add_nocarry(inp_a: &mut U256, b: &U256) {
    let mut a = from_word_vec(&inp_a.to_words());
    let b = from_word_vec(&b.to_words());

    let mut carry = 0;

    for (a, b) in a.iter_mut().zip(b.iter()) {
        *a = adc(*a, *b, &mut carry);
    }

    *inp_a = U256::from_words(to_word_vec(&a));

    debug_assert!(0 == carry);
}

#[inline]
pub(crate) fn sub_noborrow(inp_a: &mut U256, b: &U256) {
    let mut a = from_word_vec(&inp_a.to_words());
    let b = from_word_vec(&b.to_words());

    #[inline]
    fn sbb(a: u128, b: u128, borrow: &mut u128) -> u128 {
        let (a1, a0) = split_u128(a);
        let (b1, b0) = split_u128(b);
        let (b, r0) = split_u128((1 << 64) + a0 - b0 - *borrow);
        let (b, r1) = split_u128((1 << 64) + a1 - b1 - ((b == 0) as u128));

        *borrow = (b == 0) as u128;

        combine_u128(r1, r0)
    }

    let mut borrow = 0;

    for (a, b) in a.iter_mut().zip(b.iter()) {
        *a = sbb(*a, *b, &mut borrow);
    }

    *inp_a = U256::from_words(to_word_vec(&a));

    debug_assert!(0 == borrow);
}

#[inline]
pub(crate) fn sub_noborrow_raw(a: &mut [u128; 2], b: &[u128; 2]) {
    #[inline]
    fn sbb(a: u128, b: u128, borrow: &mut u128) -> u128 {
        let (a1, a0) = split_u128(a);
        let (b1, b0) = split_u128(b);
        let (b, r0) = split_u128((1 << 64) + a0 - b0 - *borrow);
        let (b, r1) = split_u128((1 << 64) + a1 - b1 - ((b == 0) as u128));

        *borrow = (b == 0) as u128;

        combine_u128(r1, r0)
    }

    let mut borrow = 0;

    for (a, b) in a.into_iter().zip(b.iter()) {
        *a = sbb(*a, *b, &mut borrow);
    }

    debug_assert!(0 == borrow);
}

// TODO: Make `from_index` a const param
#[inline(always)]
pub(crate) fn mac_digit(from_index: usize, acc: &mut [u128; 4], b: &[u128; 2], c: u128) {
    #[inline]
    fn mac_with_carry(a: u128, b: u128, c: u128, carry: &mut u128) -> u128 {
        let (b_hi, b_lo) = split_u128(b);
        let (c_hi, c_lo) = split_u128(c);

        let (a_hi, a_lo) = split_u128(a);
        let (carry_hi, carry_lo) = split_u128(*carry);
        let (x_hi, x_lo) = split_u128(b_lo * c_lo + a_lo + carry_lo);
        let (y_hi, y_lo) = split_u128(b_lo * c_hi);
        let (z_hi, z_lo) = split_u128(b_hi * c_lo);
        // Brackets to allow better ILP
        let (r_hi, r_lo) = split_u128((x_hi + y_lo) + (z_lo + a_hi) + carry_hi);

        *carry = (b_hi * c_hi) + r_hi + y_hi + z_hi;

        combine_u128(r_lo, x_lo)
    }

    if c == 0 {
        return;
    }

    let mut carry = 0;

    debug_assert_eq!(acc.len(), 4);
    unroll! {
        for i in 0..2 {
            let a_index = i + from_index;
            acc[a_index] = mac_with_carry(acc[a_index], b[i], c, &mut carry);
        }
    }
    unroll! {
        for i in 0..2 {
            let a_index = i + from_index + 2;
            if a_index < 4 {
                let (a_hi, a_lo) = split_u128(acc[a_index]);
                let (carry_hi, carry_lo) = split_u128(carry);
                let (x_hi, x_lo) = split_u128(a_lo + carry_lo);
                let (r_hi, r_lo) = split_u128(x_hi + a_hi + carry_hi);

                carry = r_hi;

                acc[a_index] = combine_u128(r_lo, x_lo);
            }
        }
    }

    debug_assert!(carry == 0);
}

#[inline]
fn mul_reduce(this: &mut [u128; 2], by: &[u128; 2], modulus: &[u128; 2], inv: u128) {
    // The Montgomery reduction here is based on Algorithm 14.32 in
    // Handbook of Applied Cryptography
    // <http://cacr.uwaterloo.ca/hac/about/chap14.pdf>.

    let mut res = [0; 2 * 2];
    unroll! {
        for i in 0..2 {
            mac_digit(i, &mut res, by, this[i]);
        }
    }

    unroll! {
        for i in 0..2 {
            let k = inv.wrapping_mul(res[i]);
            mac_digit(i, &mut res, modulus, k);
        }
    }

    this.copy_from_slice(&res[2..]);
}

#[inline]
pub fn bytes_to_bits_le(bytes: &[u8]) -> Vec<bool> {
    let mut bits = Vec::with_capacity(bytes.len() * 8);
    for byte in bytes {
        for i in 0..8 {
            // Check if the i-th bit is set
            let bit = (byte >> i) & 1 == 1;
            bits.push(bit);
        }
    }
    bits
}

#[inline]
pub(crate) fn to_word_vec(a: &[u128; 2]) -> [u64; 4] {
    [
        a[0] as u64,         // lower 64 bits of a[0]
        (a[0] >> 64) as u64, // upper 64 bits of a[0]
        a[1] as u64,         // lower 64 bits of a[1]
        (a[1] >> 64) as u64, // upper 64 bits of a[1]
    ]
}

#[inline]
pub(crate) fn from_word_vec(d: &[u64; 4]) -> [u128; 2] {
    let mut a = [0u128; 2];
    a[0] = (d[1] as u128) << 64 | d[0] as u128;
    a[1] = (d[3] as u128) << 64 | d[2] as u128;
    a
}

#[inline]
pub(crate) fn u256_get_bit(data: &U256, index: usize) -> Option<bool> {
    let p_data = from_word_vec(&data.to_words());

    if index >= 256 {
        None
    } else {
        let part = index / 128;
        let bit = index - (128 * part);

        Some(p_data[part] & (1 << bit) > 0)
    }
}

#[inline]
pub(crate) fn u256_set_bit(data: &mut U256, n: usize, to: bool) -> bool {
    let mut p_data = from_word_vec(&data.to_words());

    if n >= 256 {
        false
    } else {
        let part = n / 128;
        let bit = n - (128 * part);
        if to {
            p_data[part] |= 1 << bit;
        } else {
            p_data[part] &= !(1 << bit);
        }
        *data = U256::from_words(to_word_vec(&p_data));

        true
    }
}

pub fn mono_mul(oprand: &mut U256, other: &U256, modulo: &U256, inv: u128) {
    let mut p_oprand = from_word_vec(&oprand.to_words());

    mul_reduce(
        &mut p_oprand,
        &from_word_vec(&other.to_words()),
        &from_word_vec(&modulo.to_words()),
        inv,
    );

    if *oprand >= *modulo {
        sub_noborrow_raw(&mut p_oprand, &from_word_vec(&modulo.to_words()));
    }

    *oprand = U256::from_words(to_word_vec(&p_oprand));
}

pub(crate) fn u256_is_even(data: &U256) -> bool {
    let p_data = from_word_vec(&data.to_words());

    p_data[0] & 1 == 0
}

#[inline]
pub(crate) fn u256_sub_mod(mut oprand: &mut U256, other: &U256, modulo: &U256) {
    if *oprand < *other {
        add_nocarry(&mut oprand, modulo);
    }

    sub_noborrow(&mut oprand, other);
}

#[inline]
pub(crate) fn u256_add_mod(mut oprand: &mut U256, other: &U256, modulo: &U256) {
    add_nocarry(&mut oprand, other);

    if *oprand >= *modulo {
        sub_noborrow(&mut oprand, modulo);
    }
}

#[inline]
pub(crate) fn u256_neg_mod(mut oprand: &mut U256, modulo: &U256) {
    if *oprand > U256::ZERO {
        let mut tmp = modulo.clone();
        sub_noborrow(&mut tmp, &oprand);

        *oprand = tmp;
    }
}

pub fn mono_invert(oprand: &mut U256, modulo: &U256) {
    // Guajardo Kumar Paar Pelzl
    // Efficient Software-Implementation of Finite Fields with Applications to Cryptography
    // Algorithm 16 (BEA for Inversion in Fp)

    let mut u = *oprand;
    let mut v = *modulo;
    let mut b = U256::ONE;
    let mut c = U256::ZERO;

    while u != U256::ONE && v != U256::ONE {
        while u256_is_even(&u) {
            div2(&mut u);

            if u256_is_even(&b) {
                div2(&mut b);
            } else {
                add_nocarry(&mut b, &modulo);
                div2(&mut b);
            }
        }
        while u256_is_even(&v) {
            div2(&mut v);

            if u256_is_even(&c) {
                div2(&mut c);
            } else {
                add_nocarry(&mut c, &modulo);
                div2(&mut c);
            }
        }

        if u >= v {
            sub_noborrow(&mut u, &v);
            u256_sub_mod(&mut b, &c, modulo);
        } else {
            sub_noborrow(&mut v, &u);
            u256_sub_mod(&mut c, &b, modulo);
        }
    }

    if u == U256::ONE {
        *oprand = b;
    } else {
        *oprand = c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::U256;

    #[test]
    fn test_mono_mul_basic() {
        // Example values (these should be chosen to be meaningful for your use case)
        let mut oprand = U256::from_words(to_word_vec(&[
            197819666991454437478780191616651403388,
            17407853256822004207887426165839118766,
        ]));
        let other = U256::from_words(to_word_vec(&[
            241790196284204419545734427407138093705,
            9100138014801027929810694757406572534,
        ]));
        let modulo = U256::from_words(to_word_vec(&[
            201385395114098847380338600778089168199,
            64323764613183177041862057485226039389,
        ]));

        // For Montgomery multiplication, inv is usually computed as -modulus^{-1} mod R
        // For this simple test, let's just use 1 as a placeholder
        let inv: u128 = 211173256549385567650468519415768310665;

        mono_mul(&mut oprand, &other, &modulo, inv);

        mono_invert(&mut oprand, &modulo);

        // Compute expected result: (5 * 7) % 13 = 35 % 13 = 9
        let expected = U256::from_words(to_word_vec(&[
            162169656296878901026329651626330491773,
            45202910072195800725160941991901349318,
        ]));

        assert_eq!(oprand, expected, "mono_mul did not produce expected result");
    }

    #[test]
    fn test_mono_sub_basic() {
        // Example values (these should be chosen to be meaningful for your use case)
        let mut oprand = U256::from_words(to_word_vec(&[
            197819666991454437478780191616651403388,
            17407853256822004207887426165839118766,
        ]));
        let mut other = U256::from_words(to_word_vec(&[
            241790196284204419545734427407138093705,
            9100138014801027929810694757406572534,
        ]));
        let modulo = U256::from_words(to_word_vec(&[
            201385395114098847380338600778089168199,
            64323764613183177041862057485226039389,
        ]));

        u256_sub_mod(&mut oprand, &other, &modulo);

        u256_sub_mod(&mut other, &oprand, &modulo);

        // Compute expected result: (5 * 7) % 13 = 35 % 13 = 9
        let expected_oprand = U256::from_words(to_word_vec(&[
            296311837628188481396420371641281521139,
            8307715242020976278076731408432546231,
        ]));
        let expected_other = U256::from_words(to_word_vec(&[
            285760725576954401612688663197624784022,
            792422772780051651733963348974026302,
        ]));

        assert_eq!(
            oprand, expected_oprand,
            "mono_mul did not produce expected result"
        );
        assert_eq!(
            other, expected_other,
            "mono_mul did not produce expected result"
        );
    }
}
