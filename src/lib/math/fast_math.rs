//! 組み込み整数型向けの基本演算です。

/// 高速累乗、剰余逆元、最大公約数、最小公倍数を追加する拡張トレイトです。
///
/// `i32`、`i64`、`isize`、`u32`、`u64`、`usize` に実装されています。
/// 累乗の指数は非負である必要があります。符号付き整数で負の指数を渡した場合は `1` を
/// 返します。乗算の overflow は検査しません。
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
    /// `self` の `exponent` 乗を返します。計算量は `O(log exponent)` です。
    fn fast_pow(self, exponent: Self) -> Self;

    /// `self.pow(exponent) mod modulus` を返します。
    ///
    /// 計算量は `O(log exponent)` です。`modulus == 0` の場合は panic します。
    fn mod_pow(self, exponent: Self, modulus: Self) -> Self;

    /// `modulus` における乗法逆元を返します。
    ///
    /// `modulus` が素数で `self` と互いに素であることを前提に、フェルマーの小定理を
    /// 使います。計算量は `O(log modulus)` です。
    fn mod_inv(self, modulus: Self) -> Self;

    /// ユークリッドの互除法で最大公約数を返します。計算量は `O(log min(self, rhs))` です。
    fn gcd(self, rhs: Self) -> Self;

    /// 最小公倍数を返します。いずれかが `0` なら `0` です。
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
