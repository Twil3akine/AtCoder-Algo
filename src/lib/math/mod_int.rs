//! コンパイル時に法を指定する剰余整数です。

use std::fmt::{self, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// 法 `MOD` 上の整数です。
///
/// 値は常に `0 <= value < MOD` に正規化されます。除算は `MOD` が素数で除数が `MOD`
/// と互いに素であることを前提とします。乗算結果が `i64` の範囲を超えないようにしてください。
///
/// # Panics
///
/// `MOD <= 0` の場合は構築時に panic します。
///
/// # Examples
///
/// ```
/// use atcoder::math::mod_int::Mod998;
///
/// let value = Mod998::new(-1);
/// assert_eq!(value.value(), 998_244_352);
/// assert_eq!((value + Mod998::new(2)).value(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModInt<const MOD: i64> {
    value: i64,
}

impl<const MOD: i64> ModInt<MOD> {
    /// `value` を法 `MOD` で正規化して作ります。計算量は `O(1)` です。
    pub fn new(value: i64) -> Self {
        assert!(MOD > 0, "modulus must be positive");
        Self {
            value: value.rem_euclid(MOD),
        }
    }

    /// 正規化済みの値を返します。
    pub const fn value(self) -> i64 {
        self.value
    }

    /// 正規化済みの値を返します。
    ///
    /// [`value`](Self::value) と同じです。
    pub const fn val(&self) -> i64 {
        self.value
    }

    /// `self` の乗法逆元を返します。計算量は `O(log MOD)` です。
    pub fn inv(self) -> Self {
        self.pow(MOD - 2)
    }

    /// `self` の `exponent` 乗を返します。
    ///
    /// `exponent` は非負である必要があります。計算量は `O(log exponent)` です。
    pub fn pow(self, mut exponent: i64) -> Self {
        assert!(exponent >= 0, "exponent must be non-negative");
        let mut result = 1_i64;
        let mut base = self.value;
        while exponent > 0 {
            if exponent & 1 == 1 {
                result = result * base % MOD;
            }
            base = base * base % MOD;
            exponent >>= 1;
        }
        Self::new(result)
    }
}

impl<const MOD: i64> Display for ModInt<MOD> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

impl<const MOD: i64> Add for ModInt<MOD> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl<const MOD: i64> Sub for ModInt<MOD> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl<const MOD: i64> Mul for ModInt<MOD> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}

impl<const MOD: i64> Div for ModInt<MOD> {
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<const MOD: i64> AddAssign for ModInt<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const MOD: i64> SubAssign for ModInt<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const MOD: i64> MulAssign for ModInt<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const MOD: i64> DivAssign for ModInt<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

/// 法 `998244353` の剰余整数です。
pub type Mod998 = ModInt<998_244_353>;
/// 法 `1_000_000_007` の剰余整数です。
pub type Mod107 = ModInt<1_000_000_007>;

#[cfg(test)]
mod tests {
    use super::Mod998;

    #[test]
    fn normalizes_and_calculates() {
        assert_eq!(Mod998::new(-1).value(), 998_244_352);
        assert_eq!((Mod998::new(2) / Mod998::new(2)).value(), 1);
        assert_eq!(Mod998::new(2).pow(10).value(), 1024);
    }
}
