//! コンパイル時に法を指定する剰余整数です。

use std::fmt::{self, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// コンパイル時定数 `MOD` を法とする剰余整数です。
///
/// 構築時に値を剰余で正規化し、保持値は常に `0 <= value < MOD` です。`MOD` は正である必要が
/// あります。`pow` は非負整数の指数に使います。`inv` と `/` は `MOD` が素数で、対象の値が
/// 法 `MOD` で `0` でない場合にのみ数学的な乗法逆元として機能します。
/// 加算・減算・乗算・累乗の途中計算は `i64` で行うため、中間値が `i64` に収まる `MOD` を
/// 選んでください。乗算については、少なくとも `(MOD - 1) * (MOD - 1)` が `i64::MAX` 以下に
/// なる法であることを確認してください。
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
    /// `value` を法 `MOD` で正規化した値を作ります。負の値も `0..MOD` に収めます。
    /// 計算量は `O(1)` です。
    ///
    /// # Panics
    ///
    /// `MOD <= 0` の場合に panic します。
    pub fn new(value: i64) -> Self {
        assert!(MOD > 0, "modulus must be positive");
        Self {
            value: value.rem_euclid(MOD),
        }
    }

    /// 正規化済みの値（`0..MOD`）を返します。
    pub const fn value(self) -> i64 {
        self.value
    }

    /// 正規化済みの値（`0..MOD`）を参照経由で返します。
    ///
    /// [`value`](Self::value) と同じです。
    pub const fn val(&self) -> i64 {
        self.value
    }

    /// フェルマーの小定理に基づき、`self` の乗法逆元候補を返します。
    ///
    /// `MOD` が素数かつ `self.value() != 0` の場合に、数学的な逆元になることを保証します。
    /// 素数判定やゼロの検査は行いません。`MOD == 1` では指数が負になるため panic します。
    /// それ以外の条件外の入力でも逆元としての正しさは保証しないため、呼び出し側で条件を保証してください。
    /// 計算量は `O(log MOD)` です。
    pub fn inv(self) -> Self {
        self.pow(MOD - 2)
    }

    /// `self` の `exponent` 乗を法 `MOD` で返します。
    ///
    /// `exponent` は非負である必要があります。計算量は `O(log exponent)` です。
    /// 乗算中間値を `i64` で計算するため、想定法に対してその値が範囲内に収まることが必要です。
    ///
    /// # Panics
    ///
    /// `exponent < 0` の場合に panic します。
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

/// 法 `998_244_353` の剰余整数です（競技プログラミングでよく使われる素数法）。
pub type Mod998 = ModInt<998_244_353>;
/// 法 `1_000_000_007` の剰余整数です（競技プログラミングでよく使われる素数法）。
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
