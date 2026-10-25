//! 床関数の和に関するアルゴリズムです。

/// 等差数列の各項を `m` で割った商（床）の総和を返します。
///
/// 次の値を `O(log m)` で計算します。
///
/// ```text
/// sum_{i = 0}^{n - 1} floor((a * i + b) / m)
/// ```
///
/// `n` は項数（和の対象は `i = 0..n` の半開区間）、`m` は除数、`a` は係数、`b` は定数項です。
/// すべての引数は `u64` なので非負であり、`m` は正でなければなりません。
///
/// 計算量は `O(log m)` です。戻り値は `u128` ですが、非常に大きな入力では総和が `u128` に
/// 収まらないことがあります。正しい結果を得るには総和が `u128` に収まり、計算途中の値も
/// `u128` の範囲内であることが必要です。積の計算には `u128` を使います。
///
/// # Panics
///
/// `m` が `0` の場合は panic します。
///
/// # Examples
///
/// ```
/// use atcoder::floor_sum::floor_sum;
///
/// let sum = floor_sum(6, 5, 3, 2);
///
/// // floor(2 / 5) + floor(5 / 5) + floor(8 / 5)
/// // + floor(11 / 5) + floor(14 / 5) + floor(17 / 5)
/// // = 0 + 1 + 1 + 2 + 2 + 3
/// assert_eq!(sum, 9);
/// ```
pub fn floor_sum(mut n: u64, mut m: u64, mut a: u64, mut b: u64) -> u128 {
    assert!(m > 0, "modulus must be positive");
    if n == 0 {
        return 0;
    }

    let mut sum = 0u128;

    loop {
        // a = q * m + r と分解する。
        //
        // floor((a * i + b) / m)
        // = q * i + floor((r * i + b) / m)
        //
        // であるため、q * (0 + 1 + ... + n - 1) を先に加える。
        if a >= m {
            let quotient = a / m;

            sum += n as u128 * (n - 1) as u128 / 2 * quotient as u128;

            a %= m;
        }

        // b = q * m + r と分解する。
        //
        // 各項に q が含まれるため、q * n を先に加える。
        if b >= m {
            let quotient = b / m;

            sum += n as u128 * quotient as u128;
            b %= m;
        }

        // 半開区間の右端 i = n における一次式の値を求める。
        let y_max = a as u128 * n as u128 + b as u128;

        // a * n + b < m なら、残りの各項の商はすべて 0 になる。
        if y_max < m as u128 {
            break;
        }

        // 直線の下にある格子点を、列ごとではなく行ごとに数え直す。
        //
        // floor_sum(n, m, a, b)
        // = floor_sum(
        //     floor((a * n + b) / m),
        //     a,
        //     m,
        //     (a * n + b) mod m,
        //   )
        //
        // という変形を行う。
        n = (y_max / m as u128) as u64;
        b = (y_max % m as u128) as u64;

        std::mem::swap(&mut m, &mut a);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::floor_sum;

    /// 定義どおりに `O(n)` で床関数の和を計算します。
    fn floor_sum_naive(n: u64, m: u64, a: u64, b: u64) -> u128 {
        (0..n)
            .map(|i| (a as u128 * i as u128 + b as u128) / m as u128)
            .sum()
    }

    #[test]
    fn calculates_floor_sum() {
        assert_eq!(floor_sum(6, 5, 3, 2), 9);
        assert_eq!(floor_sum(0, 5, 8, 7), 0);
        assert_eq!(floor_sum(10, 7, 0, 15), 20);
    }

    #[test]
    fn matches_naive_calculation() {
        for n in 0..20 {
            for m in 1..20 {
                for a in 0..20 {
                    for b in 0..20 {
                        assert_eq!(
                            floor_sum(n, m, a, b),
                            floor_sum_naive(n, m, a, b),
                            "n = {n}, m = {m}, a = {a}, b = {b}",
                        );
                    }
                }
            }
        }
    }
}
