//! 素数と素因数分解に関するアルゴリズムです。

/// エラトステネスの篩で素数と最小素因数を前計算します。
///
/// `0..=limit` の各整数について最小素因数を保持します。
/// `is_prime` は `O(1)`、`prime_factorize` は素因数の重複度を含めて
/// `O(log N)` で計算できます。
///
/// 構築の計算量は `O(N log log N)`、使用メモリは `O(N)` です。
///
/// # Examples
///
/// ```
/// use atcoder::math::sieve::Sieve;
///
/// let sieve = Sieve::new(100);
///
/// assert!(sieve.is_prime(97));
/// assert!(!sieve.is_prime(1));
///
/// assert_eq!(
///     sieve.prime_factorize(84),
///     vec![(2, 2), (3, 1), (7, 1)],
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Sieve {
    smallest_prime_factor: Vec<usize>,
}

impl Sieve {
    /// `0..=limit` の各整数について最小素因数を前計算します。
    ///
    /// 計算量は `O(limit log log limit)`、使用メモリは `O(limit)` です。
    pub fn new(limit: usize) -> Self {
        let mut smallest_prime_factor: Vec<usize> = (0..=limit).collect();

        for prime in 2..=limit {
            // 最小素因数が自分自身なら prime は素数である。
            if smallest_prime_factor[prime] != prime {
                continue;
            }

            // prime * prime のオーバーフローを避ける。
            if prime > limit / prime {
                continue;
            }

            // prime 未満の倍数は、より小さい素因数によって
            // すでに処理されているため prime^2 から始める。
            for multiple in (prime * prime..=limit).step_by(prime) {
                if smallest_prime_factor[multiple] == multiple {
                    smallest_prime_factor[multiple] = prime;
                }
            }
        }

        Self {
            smallest_prime_factor,
        }
    }

    /// 前計算した最大の整数を返します。
    pub fn limit(&self) -> usize {
        self.smallest_prime_factor.len() - 1
    }

    /// `n` が素数かどうかを返します。
    ///
    /// 計算量は `O(1)` です。
    ///
    /// # Panics
    ///
    /// `n > limit` の場合は panic します。
    pub fn is_prime(&self, n: usize) -> bool {
        self.assert_in_range(n);

        n >= 2 && self.smallest_prime_factor[n] == n
    }

    /// `0..=limit` に含まれる素数を昇順に列挙します。
    ///
    /// 全要素を列挙する計算量は `O(limit)` です。
    pub fn primes(&self) -> impl Iterator<Item = usize> + '_ {
        (2..=self.limit()).filter(|&n| self.smallest_prime_factor[n] == n)
    }

    /// `n` を素因数分解し、`(素因数, 指数)` の列を昇順に返します。
    ///
    /// `n = 1` の場合は空の `Vec` を返します。
    /// 計算量は素因数の重複度を含めて `O(log n)` です。
    ///
    /// # Panics
    ///
    /// - `n = 0` の場合
    /// - `n > limit` の場合
    pub fn prime_factorize(&self, mut n: usize) -> Vec<(usize, usize)> {
        assert!(n > 0, "zero does not have a prime factorization");

        self.assert_in_range(n);

        let mut factors = Vec::new();

        while n > 1 {
            let prime = self.smallest_prime_factor[n];
            let mut exponent = 0;

            while n % prime == 0 {
                n /= prime;
                exponent += 1;
            }

            factors.push((prime, exponent));
        }

        factors
    }

    fn assert_in_range(&self, n: usize) {
        assert!(
            n <= self.limit(),
            "n exceeds the precomputed range: n = {n}, limit = {}",
            self.limit(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::Sieve;

    #[test]
    fn identifies_primes() {
        let sieve = Sieve::new(20);

        assert!(!sieve.is_prime(0));
        assert!(!sieve.is_prime(1));
        assert!(sieve.is_prime(2));
        assert!(sieve.is_prime(19));
        assert!(!sieve.is_prime(20));

        assert_eq!(
            sieve.primes().collect::<Vec<_>>(),
            vec![2, 3, 5, 7, 11, 13, 17, 19],
        );
    }

    #[test]
    fn factorizes_numbers() {
        let sieve = Sieve::new(100);

        assert_eq!(sieve.prime_factorize(1), vec![]);
        assert_eq!(sieve.prime_factorize(2), vec![(2, 1)]);

        assert_eq!(sieve.prime_factorize(84), vec![(2, 2), (3, 1), (7, 1)],);

        assert_eq!(sieve.prime_factorize(97), vec![(97, 1)],);
    }
}
