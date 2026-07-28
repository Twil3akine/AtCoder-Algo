//! 組み込み整数型向けの拡張です。

use std::ops::{Div, Rem};

/// 組み込み整数型に便利な整数演算を追加する拡張トレイトです。
///
/// すべての符号付き・符号なし整数型に実装されています。
///
/// # Examples
///
/// ```
/// use atcoder::math::integer::IntegerExt;
///
/// let mut value = 10_usize;
/// assert!(value.chmin(7));
/// assert!(!value.chmin(9));
/// assert_eq!(value, 7);
///
/// assert_eq!(10_usize.div_rem(3), (3, 1));
/// ```
pub trait IntegerExt: Sized + Copy + Ord {
    /// 切り捨て除算の商と余りを `(self / rhs, self % rhs)` として返します。
    fn div_rem(self, rhs: Self) -> (Self, Self)
    where
        Self: Div<Output = Self> + Rem<Output = Self>,
    {
        (self / rhs, self % rhs)
    }

    /// `rhs` が `self` より小さければ `self` を更新し、更新したかを返します。
    fn chmin(&mut self, rhs: Self) -> bool {
        if rhs < *self {
            *self = rhs;
            true
        } else {
            false
        }
    }

    /// `rhs` が `self` より大きければ `self` を更新し、更新したかを返します。
    fn chmax(&mut self, rhs: Self) -> bool {
        if rhs > *self {
            *self = rhs;
            true
        } else {
            false
        }
    }
}

macro_rules! impl_integer_ext {
    ($($integer:ty),* $(,)?) => {
        $(impl IntegerExt for $integer {})*
    };
}

impl_integer_ext!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,);

#[cfg(test)]
mod tests {
    use super::IntegerExt;

    #[test]
    fn calculates_quotient_and_remainder() {
        assert_eq!(10_usize.div_rem(3), (3, 1));
        assert_eq!((-10_i32).div_rem(3), (-3, -1));
    }

    #[test]
    fn updates_minimum_and_maximum() {
        let mut value = 10_i32;

        assert!(value.chmin(7));
        assert_eq!(value, 7);
        assert!(!value.chmin(9));
        assert_eq!(value, 7);

        assert!(value.chmax(12));
        assert_eq!(value, 12);
        assert!(!value.chmax(8));
        assert_eq!(value, 12);
    }
}
