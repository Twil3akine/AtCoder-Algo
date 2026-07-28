//! 整数と任意基数の桁列を相互変換します。

/// `value` を `base` 進数の桁列へ変換します。
///
/// 桁は上位から順に並び、各桁は `0..base` の値になります。
/// `value` が `0` の場合は `[0]` を返します。
///
/// # Panics
///
/// `base` が `2` 未満の場合は panic します。
///
/// # Examples
///
/// ```
/// use atcoder::math::radix::to_digits;
///
/// assert_eq!(to_digits(13, 2), vec![1, 1, 0, 1]);
/// assert_eq!(to_digits(255, 16), vec![15, 15]);
/// assert_eq!(to_digits(0, 10), vec![0]);
/// ```
pub fn to_digits(mut value: usize, base: usize) -> Vec<usize> {
    assert!(base >= 2, "base must be at least 2");

    if value == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();
    while value > 0 {
        digits.push(value % base);
        value /= base;
    }
    digits.reverse();
    digits
}

/// 正の `value` を0のない `base` 進数の桁列へ変換します。
///
/// 桁は上位から順に並びます。返される各桁は文字などへの添字として扱いやすいように
/// `0..base` で表され、数値としてはそれぞれ `1..=base` に対応します。
///
/// # Panics
///
/// `base` が `2` 未満、または `value` が `0` の場合は panic します。
///
/// # Examples
///
/// ```
/// use atcoder::math::radix::to_bijective_digits;
///
/// assert_eq!(to_bijective_digits(1, 26), vec![0]);
/// assert_eq!(to_bijective_digits(26, 26), vec![25]);
/// assert_eq!(to_bijective_digits(27, 26), vec![0, 0]);
/// ```
pub fn to_bijective_digits(mut value: usize, base: usize) -> Vec<usize> {
    assert!(base >= 2, "base must be at least 2");
    assert!(value >= 1, "value must be positive");

    let mut digits = Vec::new();
    while value > 0 {
        value -= 1;
        digits.push(value % base);
        value /= base;
    }
    digits.reverse();
    digits
}

/// `base` 進数の桁列を `usize` へ変換します。
///
/// 桁は上位から順に渡します。空の桁列、`base` 以上の桁、または `usize` の範囲を
/// 超える値に対しては `None` を返します。
///
/// # Panics
///
/// `base` が `2` 未満の場合は panic します。
///
/// # Examples
///
/// ```
/// use atcoder::math::radix::from_digits;
///
/// assert_eq!(from_digits(&[1, 1, 0, 1], 2), Some(13));
/// assert_eq!(from_digits(&[15, 15], 16), Some(255));
/// ```
pub fn from_digits(digits: &[usize], base: usize) -> Option<usize> {
    assert!(base >= 2, "base must be at least 2");

    if digits.is_empty() {
        return None;
    }

    digits.iter().try_fold(0_usize, |value, &digit| {
        if digit >= base {
            return None;
        }
        value.checked_mul(base)?.checked_add(digit)
    })
}

#[cfg(test)]
mod tests {
    use super::{from_digits, to_bijective_digits, to_digits};

    #[test]
    fn converts_values_to_digits() {
        assert_eq!(to_digits(0, 2), vec![0]);
        assert_eq!(to_digits(13, 2), vec![1, 1, 0, 1]);
        assert_eq!(to_digits(255, 16), vec![15, 15]);
        assert_eq!(to_digits(26, 26), vec![1, 0]);
    }

    #[test]
    fn converts_values_to_bijective_digits() {
        assert_eq!(to_bijective_digits(1, 26), vec![0]);
        assert_eq!(to_bijective_digits(26, 26), vec![25]);
        assert_eq!(to_bijective_digits(27, 26), vec![0, 0]);
        assert_eq!(to_bijective_digits(28, 26), vec![0, 1]);
        assert_eq!(
            to_bijective_digits(usize::MAX, usize::MAX),
            vec![usize::MAX - 1]
        );
    }

    #[test]
    fn converts_digits_to_values() {
        assert_eq!(from_digits(&[0], 2), Some(0));
        assert_eq!(from_digits(&[1, 1, 0, 1], 2), Some(13));
        assert_eq!(from_digits(&[15, 15], 16), Some(255));
        assert_eq!(from_digits(&[0, 0, 1], 10), Some(1));
    }

    #[test]
    fn round_trips_values() {
        for base in [2, 3, 10, 16, 26, usize::MAX] {
            for value in [0, 1, 25, 26, 255, usize::MAX] {
                assert_eq!(from_digits(&to_digits(value, base), base), Some(value));
            }
        }
    }

    #[test]
    fn rejects_empty_invalid_or_overflowing_digits() {
        assert_eq!(from_digits(&[], 10), None);
        assert_eq!(from_digits(&[1, 2], 2), None);
        assert_eq!(from_digits(&[usize::MAX], usize::MAX), None);
        assert_eq!(from_digits(&[1, 1], usize::MAX), None);
    }

    #[test]
    #[should_panic(expected = "base must be at least 2")]
    fn rejects_invalid_base_when_encoding() {
        to_digits(10, 1);
    }

    #[test]
    #[should_panic(expected = "value must be positive")]
    fn rejects_zero_when_bijective_encoding() {
        to_bijective_digits(0, 26);
    }

    #[test]
    #[should_panic(expected = "base must be at least 2")]
    fn rejects_invalid_base_when_decoding() {
        from_digits(&[1], 1);
    }
}
