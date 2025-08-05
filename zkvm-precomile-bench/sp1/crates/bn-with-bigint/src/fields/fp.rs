use crate::arith::{mono_invert, mono_mul, u256_set_bit};
use crate::fields::u512::U512;
use crate::fields::FieldElement;
use alloc::vec::Vec;
use core::ops::{Add, Mul, Neg, Sub};
use crypto_bigint::{Zero, U256};
use rand::Rng;

macro_rules! field_impl {
    ($name:ident, $modulus:expr, $rsquared:expr, $rcubed:expr, $one:expr, $inv:expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        #[repr(C)]
        pub struct $name(U256);

        impl From<$name> for U256 {
            #[inline]
            fn from(mut a: $name) -> Self {
                mono_mul(&mut a.0, &U256::ONE, &U256::from($modulus), $inv);

                a.0
            }
        }

        impl $name {
            pub fn from_str(s: &str) -> Option<Self> {
                let ints: Vec<_> = {
                    let mut acc = Self::zero();
                    (0..11)
                        .map(|_| {
                            let tmp = acc;
                            acc = acc + Self::one();
                            tmp
                        })
                        .collect()
                };

                let mut res = Self::zero();
                for c in s.chars() {
                    match c.to_digit(10) {
                        Some(d) => {
                            res = res * ints[10];
                            res = res + ints[d as usize];
                        }
                        None => {
                            return None;
                        }
                    }
                }

                Some(res)
            }

            /// Converts a U256 to an Fp so long as it's below the modulus.
            pub fn new(mut a: U256) -> Option<Self> {
                if a < U256::from($modulus) {
                    mono_mul(&mut a, &U256::from($rsquared), &U256::from($modulus), $inv);

                    Some($name(a))
                } else {
                    None
                }
            }

            /// Converts a U256 to an Fr regardless of modulus.
            pub fn new_mul_factor(mut a: U256) -> Self {
                mono_mul(&mut a, &U256::from($rsquared), &U256::from($modulus), $inv);
                $name(a)
            }

            pub fn interpret(buf: &[u8; 64]) -> Self {
                $name::new(U512::interpret(buf).divrem(&U256::from($modulus)).1).unwrap()
            }

            /// Returns the modulus
            #[inline]
            #[allow(dead_code)]
            pub fn modulus() -> U256 {
                U256::from($modulus)
            }

            #[inline]
            #[allow(dead_code)]
            pub fn inv(&self) -> u128 {
                $inv
            }

            pub fn raw(&self) -> &U256 {
                &self.0
            }

            pub fn set_bit(&mut self, bit: usize, to: bool) {
                u256_set_bit(&mut self.0, bit, to);
            }
        }

        impl FieldElement for $name {
            #[inline]
            fn zero() -> Self {
                $name(U256::from([0, 0, 0, 0]))
            }

            #[inline]
            fn one() -> Self {
                $name(U256::from($one))
            }

            fn random<R: Rng>(rng: &mut R) -> Self {
                // $name(U256::random(rng, &U256::from($modulus)))
                todo!()
            }

            #[inline]
            fn is_zero(&self) -> bool {
                self.0 == U256::ZERO
            }

            fn inverse(mut self) -> Option<Self> {
                if self.is_zero() {
                    None
                } else {
                    mono_invert(&mut self.0, &U256::from($modulus));
                    mono_mul(
                        &mut self.0,
                        &U256::from($rcubed),
                        &U256::from($modulus),
                        $inv,
                    );

                    Some(self)
                }
            }
        }

        impl Add for $name {
            type Output = $name;

            #[inline]
            fn add(mut self, other: $name) -> $name {
                self.0 = self.0.add_mod(&other.0, &U256::from($modulus));

                self
            }
        }

        impl Sub for $name {
            type Output = $name;

            #[inline]
            fn sub(mut self, other: $name) -> $name {
                self.0 = self.0.sub_mod(&other.0, &U256::from($modulus));

                self
            }
        }

        impl Mul for $name {
            type Output = $name;

            #[inline]
            fn mul(mut self, other: $name) -> $name {
                mono_mul(&mut self.0, &other.0, &U256::from($modulus), $inv);

                self
            }
        }

        impl Neg for $name {
            type Output = $name;

            #[inline]
            fn neg(mut self) -> $name {
                self.0 = self.0.neg_mod(&U256::from($modulus));

                self
            }
        }
    };
}

field_impl!(
    Fr,
    [
        0x43e1f593f0000001,
        0x2833e84879b97091,
        0xb85045b68181585d,
        0x30644e72e131a029
    ],
    [
        0x1bb8e645ae216da7,
        0x53fe3ab1e35c59e3,
        0x8c49833d53bb8085,
        0x0216d0b17f4e44a5
    ],
    [
        0x5e94d8e1b4bf0040,
        0x2a489cbe1cfbb6b8,
        0x893cc664a19fcfed,
        0x0cf8594b7fcc657c
    ],
    [
        0xac96341c4ffffffb,
        0x36fc76959f60cd29,
        0x666ea36f7879462e,
        0xe0a77c19a07df2f
    ],
    0x6586864b4c6911b3c2e1f593efffffff
);

field_impl!(
    Fq,
    [
        0x3c208c16d87cfd47,
        0x97816a916871ca8d,
        0xb85045b68181585d,
        0x30644e72e131a029
    ],
    [
        0xf32cfc5b538afa89,
        0xb5e71911d44501fb,
        0x47ab1eff0a417ff6,
        0x06d89f71cab8351f
    ],
    [
        0xb1cd6dafda1530df,
        0x62f210e6a7283db6,
        0xef7f0b0c0ada0afb,
        0x20fd6e902d592544
    ],
    [
        0xd35d438dc58f0d9d,
        0xa78eb28f5c70b3d,
        0x666ea36f7879462c,
        0xe0a77c19a07df2f
    ],
    0x9ede7d651eca6ac987d20782e4866389
);

lazy_static::lazy_static! {

    static ref FQ: U256 = U256::from([
        0x3c208c16d87cfd47,
        0x97816a916871ca8d,
        0xb85045b68181585d,
        0x30644e72e131a029
    ]);

    pub static ref FQ_MINUS3_DIV4: Fq =
        Fq::new(U256::from_u32(3)).expect("3 is a valid field element and static; qed").neg() *
        Fq::new(U256::from_u32(4)).expect("4 is a valid field element and static; qed").inverse()
            .expect("4 has inverse in Fq and is static; qed");

    static ref FQ_MINUS1_DIV2: Fq =
        Fq::new(U256::from_u32(1)).expect("1 is a valid field element and static; qed").neg() *
        Fq::new(U256::from_u32(2)).expect("2 is a valid field element and static; qed").inverse()
            .expect("2 has inverse in Fq and is static; qed");

}

impl Fq {
    pub fn sqrt(&self) -> Option<Self> {
        let a1 = self.pow(*FQ_MINUS3_DIV4);
        let a1a = a1 * *self;
        let a0 = a1 * (a1a);

        let mut am1 = *FQ;
        am1 = am1.sub_mod(&U256::ONE, &*FQ);

        if a0 == Fq::new(am1).unwrap() {
            None
        } else {
            Some(a1a)
        }
    }
}

#[inline]
pub fn const_fq(i: [u64; 4]) -> Fq {
    Fq(U256::from(i))
}

#[test]
fn test_rsquared() {
    let rng = &mut ::rand::thread_rng();

    for _ in 0..1000 {
        let a = Fr::random(rng);
        let b: U256 = a.into();
        let c = Fr::new(b).unwrap();

        assert_eq!(a, c);
    }

    for _ in 0..1000 {
        let a = Fq::random(rng);
        let b: U256 = a.into();
        let c = Fq::new(b).unwrap();

        assert_eq!(a, c);
    }
}

#[test]
fn sqrt_fq() {
    // from zcash test_proof.cpp
    let fq1 = Fq::from_str(
        "5204065062716160319596273903996315000119019512886596366359652578430118331601",
    )
    .unwrap();
    let fq2 = Fq::from_str("348579348568").unwrap();

    assert_eq!(fq1, fq2.sqrt().expect("348579348568 is quadratic residue"));
}
