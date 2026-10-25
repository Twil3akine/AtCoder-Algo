//! 組合せ計算に関するアルゴリズムです。

use super::mod_int::ModInt;

/// 階乗と逆階乗を前計算し、順列・組合せを高速に計算します。
///
/// 法 `MOD` 上の階乗・順列・組合せを前計算して扱います。
///
/// `new(max_n)` は `0..=max_n` の階乗と逆階乗を保存します。以後、引数 `n` がこの範囲内なら
/// `n!`、順列 `nPr`、組合せ `nCr` を `O(1)` で求められます。`r > n` の順列・組合せは `0` です。
///
/// `MOD` は素数であり、`max_n < MOD` を満たす必要があります。
/// 構築の計算量は `O(max_n + log MOD)`、使用メモリは `O(max_n)` です。
///
/// # Panics
///
/// - `MOD <= 1` の場合
/// - `max_n >= MOD` の場合
/// - 前計算範囲を超える `n` を指定した場合
///
/// # Examples
///
/// ```
/// use atcoder::math::combinatorics::Combinatorics;
///
/// let comb = Combinatorics::<998_244_353>::new(10);
///
/// assert_eq!(comb.factorial(5).value(), 120);
/// assert_eq!(comb.permutation(5, 2).value(), 20);
/// assert_eq!(comb.combination(5, 2).value(), 10);
/// ```
#[derive(Debug, Clone)]
pub struct Combinatorics<const MOD: i64> {
    factorial: Vec<ModInt<MOD>>,
    inverse_factorial: Vec<ModInt<MOD>>,
}

impl<const MOD: i64> Combinatorics<MOD> {
    /// `0..=max_n` の階乗と逆階乗を法 `MOD` で前計算します。
    ///
    /// `MOD` は `1` より大きい素数、かつ `max_n < MOD` でなければなりません。
    ///
    /// 計算量は `O(max_n + log MOD)`、使用メモリは `O(max_n)` です。
    ///
    /// # Panics
    ///
    /// `MOD <= 1`、`max_n >= MOD`、または `max_n` が `i64` に収まらない場合に panic します。
    pub fn new(max_n: usize) -> Self {
        assert!(MOD > 1, "modulus must be greater than one");

        let max_n_i64 = i64::try_from(max_n).expect("max_n does not fit in i64");

        assert!(max_n_i64 < MOD, "max_n must be smaller than modulus");

        let mut factorial = vec![ModInt::new(1); max_n + 1];

        for i in 1..=max_n {
            factorial[i] = factorial[i - 1] * ModInt::new(i as i64);
        }

        // max_n! の逆元を求め、逆方向に掛け戻すことで
        // すべての逆階乗を O(max_n) で計算する。
        let mut inverse_factorial = vec![ModInt::new(1); max_n + 1];

        inverse_factorial[max_n] = factorial[max_n].inv();

        for i in (1..=max_n).rev() {
            inverse_factorial[i - 1] = inverse_factorial[i] * ModInt::new(i as i64);
        }

        Self {
            factorial,
            inverse_factorial,
        }
    }

    /// 前計算した最大の引数 `n` を返します。利用可能な範囲は `0..=max_n()` です。
    pub fn max_n(&self) -> usize {
        self.factorial.len() - 1
    }

    /// `n!` を法 `MOD` で返します。
    ///
    /// 計算量は `O(1)` です。
    ///
    /// # Panics
    ///
    /// `n > max_n()` の場合に panic します。
    pub fn factorial(&self, n: usize) -> ModInt<MOD> {
        self.assert_in_range(n);
        self.factorial[n]
    }

    /// 順列 `nPr = n! / (n-r)!` を法 `MOD` で返します。
    ///
    /// `r > n` の場合は `0` を返します。`n` は前計算範囲内である必要があります。
    /// 計算量は `O(1)` です。
    pub fn permutation(&self, n: usize, r: usize) -> ModInt<MOD> {
        self.assert_in_range(n);

        if r > n {
            return ModInt::new(0);
        }

        self.factorial[n] * self.inverse_factorial[n - r]
    }

    /// 組合せ `nCr` を法 `MOD` で返します。
    ///
    /// `r > n` の場合は `0` を返します。`n` は前計算範囲内である必要があります。
    /// 計算量は `O(1)` です。
    pub fn combination(&self, n: usize, r: usize) -> ModInt<MOD> {
        self.assert_in_range(n);

        if r > n {
            return ModInt::new(0);
        }

        self.factorial[n] * self.inverse_factorial[r] * self.inverse_factorial[n - r]
    }

    fn assert_in_range(&self, n: usize) {
        assert!(
            n <= self.max_n(),
            "n exceeds the precomputed range: n = {n}, max_n = {}",
            self.max_n(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::Combinatorics;

    type Comb = Combinatorics<998_244_353>;

    #[test]
    fn calculates_factorials_and_combinations() {
        let comb = Comb::new(10);

        assert_eq!(comb.factorial(0).value(), 1);
        assert_eq!(comb.factorial(5).value(), 120);

        assert_eq!(comb.permutation(5, 0).value(), 1);
        assert_eq!(comb.permutation(5, 2).value(), 20);
        assert_eq!(comb.permutation(5, 6).value(), 0);

        assert_eq!(comb.combination(5, 0).value(), 1);
        assert_eq!(comb.combination(5, 2).value(), 10);
        assert_eq!(comb.combination(5, 5).value(), 1);
        assert_eq!(comb.combination(5, 6).value(), 0);
    }

    #[test]
    fn satisfies_pascals_identity() {
        let comb = Comb::new(30);

        for n in 1..=30 {
            for r in 1..n {
                let lhs = comb.combination(n, r).value();

                let rhs = (comb.combination(n - 1, r - 1).value()
                    + comb.combination(n - 1, r).value())
                    % 998_244_353;

                assert_eq!(lhs, rhs, "n = {n}, r = {r}");
            }
        }
    }
}
