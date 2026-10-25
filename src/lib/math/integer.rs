//! 整数値の min/max と、組み込み整数型向けの小さな拡張を提供します。

use std::ops::{Div, Rem};

/// `lhs` と `rhs` のうち小さい方を返します。
///
/// 同じ値の場合は `lhs` を返します。比較可能な任意の型で使えます。
pub fn min<T: Ord>(lhs: T, rhs: T) -> T {
    if lhs <= rhs {
        lhs
    } else {
        rhs
    }
}

/// `lhs` と `rhs` のうち大きい方を返します。
///
/// 同じ値の場合は `lhs` を返します。比較可能な任意の型で使えます。
pub fn max<T: Ord>(lhs: T, rhs: T) -> T {
    if lhs >= rhs {
        lhs
    } else {
        rhs
    }
}

/// スライス内の最小値と最大値への参照を `(最小, 最大)` の順で返します。
///
/// 空スライスには `None`、要素がある場合は `Some((最小要素, 最大要素))` を返します。
/// 同値の要素が複数ある場合、どの同値要素への参照が選ばれるかは保証しません。
/// 要素数を `n` とすると比較回数は最大 `3 * floor(n / 2)` 回で、計算量は `O(n)` です。
pub fn minmax<T: Ord>(values: &[T]) -> Option<(&T, &T)> {
    if values.is_empty() {
        return None;
    }

    let (mut minimum, mut maximum, mut index) = if values.len() % 2 == 1 {
        (&values[0], &values[0], 1)
    } else if values[0] <= values[1] {
        (&values[0], &values[1], 2)
    } else {
        (&values[1], &values[0], 2)
    };

    while index < values.len() {
        let first = &values[index];
        let second = &values[index + 1];
        let (smaller, larger) = if first <= second {
            (first, second)
        } else {
            (second, first)
        };

        if smaller < minimum {
            minimum = smaller;
        }
        if larger > maximum {
            maximum = larger;
        }
        index += 2;
    }

    Some((minimum, maximum))
}

/// 組み込み整数型に便利な整数演算を追加する拡張トレイトです。
///
/// すべての符号付き・符号なし組み込み整数型に実装されています。
/// `div_rem` は通常の整数除算に従い、除数 `0` や符号付き最小値を `-1` で割る場合は
/// panic します。`chmin` と `chmax` は値を条件付きで更新し、その成否を `bool` で返します。
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
    /// 商と余りを `(self / rhs, self % rhs)` の順に返します。
    ///
    /// 符号付き整数の商は `0` 方向へ切り捨て、余りは `self` と同じ符号になります。
    /// 除数 `rhs` が `0` の場合など、通常の除算が panic する入力ではこのメソッドも panic します。
    fn div_rem(self, rhs: Self) -> (Self, Self)
    where
        Self: Div<Output = Self> + Rem<Output = Self>,
    {
        (self / rhs, self % rhs)
    }

    /// `rhs` が `self` より小さい場合に `self` を `rhs` で置き換えます。
    ///
    /// 値を置き換えた場合は `true`、そのままの場合は `false` を返します。
    fn chmin(&mut self, rhs: Self) -> bool {
        if rhs < *self {
            *self = rhs;
            true
        } else {
            false
        }
    }

    /// `rhs` が `self` より大きい場合に `self` を `rhs` で置き換えます。
    ///
    /// 値を置き換えた場合は `true`、そのままの場合は `false` を返します。
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
    use super::{max, min, minmax, IntegerExt};

    #[test]
    fn returns_minimum_and_maximum_of_two_values() {
        assert_eq!(min(3, 7), 3);
        assert_eq!(max(3, 7), 7);
    }

    #[test]
    fn returns_minimum_and_maximum_of_slice() {
        assert_eq!(minmax::<i32>(&[]), None);
        assert_eq!(minmax(&[5]), Some((&5, &5)));
        assert_eq!(minmax(&[3, 8, 1, 6, 4]), Some((&1, &8)));
        assert_eq!(minmax(&[3, 8, 1, 6]), Some((&1, &8)));
    }

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
