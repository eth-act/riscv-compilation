use alloc::vec::Vec;
use crypto_bigint::U256;
use crunchy::unroll;


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
        let bytes = self.int.to_le_bytes();
        let bits = bytes_to_bits_le(&bytes);
        if self.n == 0 {
            None
        } else {
            self.n -= 1;
            Some(bits[self.n])
        }
    }
}

/// Divide by two
#[inline]
fn div2(a: &mut [u128; 2]) {
    let tmp = a[1] << 127;
    a[1] >>= 1;
    a[0] >>= 1;
    a[0] |= tmp;
}

/// Multiply by two
#[inline]
fn mul2(a: &mut [u128; 2]) {
    let tmp = a[0] >> 127;
    a[0] <<= 1;
    a[1] <<= 1;
    a[1] |= tmp;
}

#[inline(always)]
fn split_u128(i: u128) -> (u128, u128) {
    (i >> 64, i & 0xFFFFFFFFFFFFFFFF)
}

#[inline(always)]
fn combine_u128(hi: u128, lo: u128) -> u128 {
    (hi << 64) | lo
}

#[inline]
fn adc(a: u128, b: u128, carry: &mut u128) -> u128 {
    let (a1, a0) = split_u128(a);
    let (b1, b0) = split_u128(b);
    let (c, r0) = split_u128(a0 + b0 + *carry);
    let (c, r1) = split_u128(a1 + b1 + c);
    *carry = c;

    combine_u128(r1, r0)
}

#[inline]
fn add_nocarry(a: &mut [u128; 2], b: &[u128; 2]) {
    let mut carry = 0;

    for (a, b) in a.into_iter().zip(b.iter()) {
        *a = adc(*a, *b, &mut carry);
    }

    debug_assert!(0 == carry);
}

#[inline]
fn sub_noborrow(a: &mut [u128; 2], b: &[u128; 2]) {
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
fn mac_digit(from_index: usize, acc: &mut [u128; 4], b: &[u128; 2], c: u128) {
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