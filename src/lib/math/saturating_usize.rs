//! コンパイル時に上限を指定する飽和 `usize` です。

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};
use std::str::FromStr;

use super::integer::IntegerExt;

/// `0..=MAX` の範囲で飽和する `usize` です。
///
/// 加算・乗算は `MAX`、減算は `0` で飽和します。`usize` との四則演算、比較、変換にも
/// 対応します。
///
/// # Examples
///
/// ```
/// use atcoder::math::saturating_usize::SaturatingUsize;
///
/// type Capacity = SaturatingUsize<10>;
///
/// assert_eq!(Capacity::new(15), 10_usize);
/// assert_eq!(Capacity::new(8) + 5, 10_usize);
/// assert_eq!(Capacity::new(3) - 5, 0_usize);
///
/// let capacity: usize = Capacity::new(7).into();
/// assert_eq!(capacity, 7);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SaturatingUsize<const MAX: usize> {
    value: usize,
}

impl<const MAX: usize> SaturatingUsize<MAX> {
    /// `0` です。
    pub const ZERO: Self = Self { value: 0 };

    /// この型が保持できる最大値です。
    pub const MAX: Self = Self { value: MAX };

    /// `value` を `MAX` 以下に飽和させて作ります。
    pub const fn new(value: usize) -> Self {
        Self {
            value: if value > MAX { MAX } else { value },
        }
    }

    /// 飽和済みの値を返します。
    pub const fn value(self) -> usize {
        self.value
    }

    /// 飽和済みの値を返します。
    ///
    /// [`value`](Self::value) と同じです。
    pub const fn val(&self) -> usize {
        self.value
    }

    /// `self` の `exponent` 乗を `MAX` で飽和させて返します。
    pub fn pow(self, mut exponent: u32) -> Self {
        let mut result = Self::new(1);
        let mut base = self;
        while exponent > 0 {
            if exponent & 1 == 1 {
                result *= base;
            }
            base *= base;
            exponent >>= 1;
        }
        result
    }
}

impl<const MAX: usize> IntegerExt for SaturatingUsize<MAX> {}

impl<const MAX: usize> From<usize> for SaturatingUsize<MAX> {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl<const MAX: usize> From<SaturatingUsize<MAX>> for usize {
    fn from(value: SaturatingUsize<MAX>) -> Self {
        value.value
    }
}

impl<const MAX: usize> AsRef<usize> for SaturatingUsize<MAX> {
    fn as_ref(&self) -> &usize {
        &self.value
    }
}

impl<const MAX: usize> Borrow<usize> for SaturatingUsize<MAX> {
    fn borrow(&self) -> &usize {
        &self.value
    }
}

impl<const MAX: usize> Deref for SaturatingUsize<MAX> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<const MAX: usize> Display for SaturatingUsize<MAX> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

impl<const MAX: usize> FromStr for SaturatingUsize<MAX> {
    type Err = <usize as FromStr>::Err;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse().map(Self::new)
    }
}

impl<const MAX: usize> PartialEq<usize> for SaturatingUsize<MAX> {
    fn eq(&self, rhs: &usize) -> bool {
        self.value == *rhs
    }
}

impl<const MAX: usize> PartialEq<SaturatingUsize<MAX>> for usize {
    fn eq(&self, rhs: &SaturatingUsize<MAX>) -> bool {
        *self == rhs.value
    }
}

impl<const MAX: usize> PartialOrd<usize> for SaturatingUsize<MAX> {
    fn partial_cmp(&self, rhs: &usize) -> Option<Ordering> {
        self.value.partial_cmp(rhs)
    }
}

impl<const MAX: usize> PartialOrd<SaturatingUsize<MAX>> for usize {
    fn partial_cmp(&self, rhs: &SaturatingUsize<MAX>) -> Option<Ordering> {
        self.partial_cmp(&rhs.value)
    }
}

macro_rules! impl_saturating_binary_op {
    ($trait:ident, $method:ident, $usize_method:ident) => {
        impl<const MAX: usize> $trait for SaturatingUsize<MAX> {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                Self::new(self.value.$usize_method(rhs.value))
            }
        }

        impl<const MAX: usize> $trait<usize> for SaturatingUsize<MAX> {
            type Output = Self;

            fn $method(self, rhs: usize) -> Self::Output {
                Self::new(self.value.$usize_method(rhs))
            }
        }

        impl<const MAX: usize> $trait<SaturatingUsize<MAX>> for usize {
            type Output = SaturatingUsize<MAX>;

            fn $method(self, rhs: SaturatingUsize<MAX>) -> Self::Output {
                SaturatingUsize::new(self.$usize_method(rhs.value))
            }
        }
    };
}

impl_saturating_binary_op!(Add, add, saturating_add);
impl_saturating_binary_op!(Sub, sub, saturating_sub);
impl_saturating_binary_op!(Mul, mul, saturating_mul);

macro_rules! impl_regular_binary_op {
    ($trait:ident, $method:ident, $operator:tt) => {
        impl<const MAX: usize> $trait for SaturatingUsize<MAX> {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                Self::new(self.value $operator rhs.value)
            }
        }

        impl<const MAX: usize> $trait<usize> for SaturatingUsize<MAX> {
            type Output = Self;

            fn $method(self, rhs: usize) -> Self::Output {
                Self::new(self.value $operator rhs)
            }
        }

        impl<const MAX: usize> $trait<SaturatingUsize<MAX>> for usize {
            type Output = SaturatingUsize<MAX>;

            fn $method(self, rhs: SaturatingUsize<MAX>) -> Self::Output {
                SaturatingUsize::new(self $operator rhs.value)
            }
        }
    };
}

impl_regular_binary_op!(Div, div, /);
impl_regular_binary_op!(Rem, rem, %);

macro_rules! impl_assign_op {
    ($trait:ident, $method:ident, $operator:tt) => {
        impl<const MAX: usize> $trait for SaturatingUsize<MAX> {
            fn $method(&mut self, rhs: Self) {
                *self = *self $operator rhs;
            }
        }

        impl<const MAX: usize> $trait<usize> for SaturatingUsize<MAX> {
            fn $method(&mut self, rhs: usize) {
                *self = *self $operator rhs;
            }
        }
    };
}

impl_assign_op!(AddAssign, add_assign, +);
impl_assign_op!(SubAssign, sub_assign, -);
impl_assign_op!(MulAssign, mul_assign, *);
impl_assign_op!(DivAssign, div_assign, /);
impl_assign_op!(RemAssign, rem_assign, %);

impl<const MAX: usize> Sum for SaturatingUsize<MAX> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a, const MAX: usize> Sum<&'a Self> for SaturatingUsize<MAX> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl<const MAX: usize> Product for SaturatingUsize<MAX> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(1), Mul::mul)
    }
}

impl<'a, const MAX: usize> Product<&'a Self> for SaturatingUsize<MAX> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

#[cfg(test)]
mod tests {
    use super::SaturatingUsize;
    use crate::math::integer::IntegerExt;

    type Capacity = SaturatingUsize<10>;

    #[test]
    fn clamps_construction_and_converts_to_usize() {
        assert_eq!(Capacity::new(7).value(), 7);
        assert_eq!(Capacity::new(20).value(), 10);
        assert_eq!(usize::from(Capacity::new(7)), 7);
        assert_eq!("20".parse::<Capacity>().unwrap(), 10);
    }

    #[test]
    fn saturates_arithmetic() {
        assert_eq!(Capacity::new(8) + Capacity::new(5), 10);
        assert_eq!(Capacity::new(8) + 5, 10);
        assert_eq!(8 + Capacity::new(5), 10);
        assert_eq!(Capacity::new(3) - 5, 0);
        assert_eq!(3_usize - Capacity::new(5), 0);
        assert_eq!(Capacity::new(4) * 3, 10);
        assert_eq!(Capacity::new(2).pow(4), 10);
    }

    #[test]
    fn supports_regular_usize_operations_and_comparisons() {
        assert_eq!(Capacity::new(9) / 2, 4);
        assert_eq!(25 / Capacity::new(4), 6);
        assert_eq!(Capacity::new(9) % 4, 1);
        assert!(Capacity::new(7) < 8);
        assert!(6 < Capacity::new(7));
    }

    #[test]
    fn supports_assignment_iteration_and_integer_ext() {
        let mut value = Capacity::new(8);
        value += 5;
        assert_eq!(value, Capacity::MAX);
        assert!(!value.chmax(Capacity::new(7)));
        assert!(value.chmin(Capacity::new(6)));

        assert_eq!([Capacity::new(4); 3].into_iter().sum::<Capacity>(), 10);
        assert_eq!([Capacity::new(2); 4].into_iter().product::<Capacity>(), 10);
    }

    #[test]
    fn supports_zero_maximum() {
        type Zero = SaturatingUsize<0>;

        assert_eq!(Zero::new(1), 0);
        assert_eq!(Zero::new(1) + usize::MAX, 0);
        assert_eq!(Zero::new(2).pow(0), 0);
    }

    #[test]
    fn avoids_usize_overflow_at_the_native_maximum() {
        type FullRange = SaturatingUsize<{ usize::MAX }>;

        assert_eq!(FullRange::MAX + 1, usize::MAX);
        assert_eq!(FullRange::MAX * 2, usize::MAX);
    }
}
