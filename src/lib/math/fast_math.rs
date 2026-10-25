//! 組み込み整数型向けの基本演算です。

/// 高速累乗、剰余逆元、最大公約数、最小公倍数を追加する拡張トレイトです。
///
/// `i32`、`i64`、`isize`、`u32`、`u64`、`usize` に実装されています。演算結果の
/// オーバーフローは検査しないため、各型に収まる入力を使ってください。
///
/// # Examples
///
/// ```
/// use atcoder::math::fast_math::FastMath;
///
/// assert_eq!(3_u64.fast_pow(4), 81);
/// assert_eq!(2_u64.mod_pow(10, 1_000), 24);
/// assert_eq!(12_u64.gcd(18), 6);
/// ```
pub trait FastMath: Sized + Copy {
    /// `self` の `exponent` 乗を返します。
    ///
    /// `exponent` が正の場合の計算量は `O(log exponent)` です。指数が `0` 以下なら
    /// `1` を返します（符号付き型の負指数も `1` になります）。積のオーバーフローは検査しません。
    fn fast_pow(self, exponent: Self) -> Self;

    /// 二分累乗法で `self.pow(exponent) mod modulus` を計算します。
    ///
    /// 非負の `exponent` と正の `modulus` を指定してください。計算量は `O(log exponent)`、
    /// `modulus == 0` の場合は panic します。剰余前の積は通常の整数乗算なので、型の範囲を
    /// 超えるとオーバーフローします。符号付き型では負数の剰余は非負に正規化されません。
    fn mod_pow(self, exponent: Self, modulus: Self) -> Self;

    /// `modulus` における乗法逆元を返します。
    ///
    /// `modulus >= 2` が素数で、`self` がその法で `0` でない場合に限り、
    /// フェルマーの小定理に基づく逆元を返します。計算量は `O(log modulus)` です。
    /// この条件を満たさない入力の結果は逆元として保証されません。
    fn mod_inv(self, modulus: Self) -> Self;

    /// ユークリッドの互除法で最大公約数を返します。通常の非負整数について計算量は
    /// `O(log min(self, rhs))` です。符号付き型では負数を渡さないでください。
    fn gcd(self, rhs: Self) -> Self;

    /// `self / gcd(self, rhs) * rhs` により最小公倍数を返します。いずれかが `0` なら `0` です。
    /// 乗算のオーバーフローは検査しません。符号付き型では非負の値を渡してください。
    fn lcm(self, rhs: Self) -> Self;
}

macro_rules! impl_fast_math {
    ($($integer:ty),* $(,)?) => {
        $(
            impl FastMath for $integer {
                fn fast_pow(mut self, mut exponent: Self) -> Self {
                    let mut result = 1;
                    while exponent > 0 {
                        if exponent & 1 == 1 {
                            result *= self;
                        }
                        self *= self;
                        exponent >>= 1;
                    }
                    result
                }

                fn mod_pow(mut self, mut exponent: Self, modulus: Self) -> Self {
                    self %= modulus;
                    let mut result = 1 % modulus;
                    while exponent > 0 {
                        if exponent & 1 == 1 {
                            result = result * self % modulus;
                        }
                        self = self * self % modulus;
                        exponent >>= 1;
                    }
                    result
                }

                fn mod_inv(self, modulus: Self) -> Self {
                    self.mod_pow(modulus - 2, modulus)
                }

                fn gcd(self, rhs: Self) -> Self {
                    let mut a = self;
                    let mut b = rhs;
                    while b != 0 {
                        let remainder = a % b;
                        a = b;
                        b = remainder;
                    }
                    a
                }

                fn lcm(self, rhs: Self) -> Self {
                    if self == 0 || rhs == 0 {
                        0
                    } else {
                        self / self.gcd(rhs) * rhs
                    }
                }
            }
        )*
    };
}

impl_fast_math!(i32, i64, isize, u32, u64, usize);

#[cfg(test)]
mod tests {
    use super::FastMath;

    #[test]
    fn computes_integer_operations() {
        assert_eq!(2_usize.fast_pow(10), 1024);
        assert_eq!(3_usize.mod_inv(7), 5);
        assert_eq!(21_usize.gcd(14), 7);
        assert_eq!(6_usize.lcm(15), 30);
    }
}
