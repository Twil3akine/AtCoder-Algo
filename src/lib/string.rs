//! 文字列と列に対するアルゴリズムです。

use std::ops::{Bound, RangeBounds};

/// ASCII 英字を `0..26` の添字へ変換します。
///
/// `char` と `u8` に実装されています。大文字・小文字の ASCII 英字にだけ使用してください。
/// それ以外の値に対する変換結果は定義された用途を持ちません。
pub trait AlphaExt {
    /// ASCII 英字を大文字・小文字を区別しない添字へ変換します。
    ///
    /// `a` または `A` は `0`、`z` または `Z` は `25` です。呼び出し側は `self` が ASCII 英字であることを
    /// 保証してください。
    fn to_idx(self) -> usize;
}

impl AlphaExt for char {
    fn to_idx(self) -> usize {
        (self.to_ascii_lowercase() as u8 - b'a') as usize
    }
}

impl AlphaExt for u8 {
    fn to_idx(self) -> usize {
        (self.to_ascii_lowercase() - b'a') as usize
    }
}

/// `0..26` の添字を小文字の ASCII 英字へ変換します。
///
/// `usize` と `u8` に実装されています。`26` 以上の値を渡すと panic します。
///
/// # Examples
///
/// ```
/// use atcoder::string::AlphaIndexExt;
///
/// assert_eq!(0_usize.to_alpha(), 'a');
/// assert_eq!(25_u8.to_alpha(), 'z');
/// ```
pub trait AlphaIndexExt {
    /// 小文字 ASCII 英字に対応する添字を返します。
    ///
    /// `0` は `a`、`25` は `z` です。`self` が `26` 以上の場合は panic します。
    fn to_alpha(self) -> char;
}

impl AlphaIndexExt for usize {
    fn to_alpha(self) -> char {
        assert!(self < 26, "alphabet index out of range");
        char::from(b'a' + self as u8)
    }
}

impl AlphaIndexExt for u8 {
    fn to_alpha(self) -> char {
        assert!(self < 26, "alphabet index out of range");
        char::from(b'a' + self)
    }
}

/// Manacher 法で奇数長・偶数長の回文半径をまとめて返します。
///
/// 戻り値の長さは `2 * sequence.len() + 1` です。空の入力では `[1]` を返します。
/// 偶数添字は要素間、奇数添字は元の要素を
/// 中心とします。`radii[i]` は区切りを挿入した列上で中心自身を含む半径であり、対応する
/// 元の回文長は `radii[i] - 1` です。元の列の添字における回文の開始位置は、中心 `i` と半径 `r`
/// から `(i + 1 - r) / 2` で求められます。`N = sequence.len()` とすると、計算量と追加メモリは `O(N)` です。
///
/// # Examples
///
/// ```
/// use atcoder::string::manacher;
///
/// let radii = manacher(&['a', 'b', 'b', 'a']);
/// assert_eq!(radii[4] - 1, 4);
/// ```
pub fn manacher<T: Eq + Clone>(sequence: &[T]) -> Vec<usize> {
    let mut separated = Vec::with_capacity(2 * sequence.len() + 1);
    for value in sequence {
        separated.push(None);
        separated.push(Some(value.clone()));
    }
    separated.push(None);

    let mut radii = vec![0; separated.len()];
    let mut left = 0;
    let mut right = 0;
    for center in 0..separated.len() {
        let mut radius = 1;
        if center < right {
            let mirror = left + right - 1 - center;
            radius = radii[mirror].min(right - center);
        }
        while center >= radius
            && center + radius < separated.len()
            && separated[center - radius] == separated[center + radius]
        {
            radius += 1;
        }
        radii[center] = radius;
        if center + radius > right {
            left = center + 1 - radius;
            right = center + radius;
        }
    }
    radii
}

/// 列の任意区間に対する操作を提供します。
///
/// `[T]` に実装されているため、`Vec<T>` や配列のスライスに対して利用できます。
///
/// 範囲引数は [`RangeBounds`] に従い、`..`, `l..`, `..r`, `l..r`, `l..=r` などを指定できます。
/// 範囲は半開区間 `[start, end)` として扱われます。添字は0始まりで、変換後の範囲が
/// `0 <= start <= end <= self.len()` を満たさない場合は panic します。
pub trait SequenceExt<T> {
    /// `range` で指定した区間が回文かどうかを返します。
    ///
    /// 範囲内の要素を先頭と末尾から順に比較します。空区間と1要素の区間は回文です。
    /// 計算量は区間長を `L` として `O(L)`、追加メモリは `O(1)` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use atcoder::string::SequenceExt;
    ///
    /// let a = vec![1, 2, 3, 2, 1];
    ///
    /// assert!(a.is_palindrome(..));
    /// assert!(a.is_palindrome(1..4));
    /// assert!(a.is_palindrome(2..3));
    /// assert!(!a.is_palindrome(0..4));
    /// ```
    fn is_palindrome<R: RangeBounds<usize>>(&self, range: R) -> bool
    where
        T: Eq;

    /// `range` で指定した区間の要素順を反転します。
    ///
    /// 範囲外の要素は変更しません。空区間では何もしません。
    /// 計算量は区間長を `L` として `O(L)`、追加メモリは `O(1)` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use atcoder::string::SequenceExt;
    ///
    /// let mut a = vec![1, 2, 3, 4, 5];
    /// a.reverse_range(1..4);
    ///
    /// assert_eq!(a, vec![1, 4, 3, 2, 5]);
    /// ```
    fn reverse_range<R: RangeBounds<usize>>(&mut self, range: R);

    /// `range` で指定した区間を左に `k` 個巡回シフトします。
    ///
    /// `k` が区間長以上の場合は、区間長で割った余りだけシフトします。たとえば `[1, 2, 3]` を `k = 1`
    /// でシフトすると `[2, 3, 1]` になります。空区間に対して呼び出した場合は何もしません。
    ///
    /// 計算量は区間長を `L` として `O(L)`、追加メモリは `O(1)` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use atcoder::string::SequenceExt;
    ///
    /// let mut a = vec![1, 2, 3, 4, 5];
    /// a.rotate_left_range(1..5, 2);
    ///
    /// assert_eq!(a, vec![1, 4, 5, 2, 3]);
    /// ```
    fn rotate_left_range<R: RangeBounds<usize>>(&mut self, range: R, k: usize);

    /// `range` で指定した区間を右に `k` 個巡回シフトします。
    ///
    /// `k` が区間長以上の場合は、区間長で割った余りだけシフトします。たとえば `[1, 2, 3]` を `k = 1`
    /// でシフトすると `[3, 1, 2]` になります。空区間に対して呼び出した場合は何もしません。
    ///
    /// 計算量は区間長を `L` として `O(L)`、追加メモリは `O(1)` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use atcoder::string::SequenceExt;
    ///
    /// let mut a = vec![1, 2, 3, 4, 5];
    /// a.rotate_right_range(1..5, 2);
    ///
    /// assert_eq!(a, vec![1, 4, 5, 2, 3]);
    /// ```
    fn rotate_right_range<R: RangeBounds<usize>>(&mut self, range: R, k: usize);
}

impl<T> SequenceExt<T> for [T] {
    fn is_palindrome<R: RangeBounds<usize>>(&self, range: R) -> bool
    where
        T: Eq,
    {
        let (l, r) = range_bounds(range, self.len());
        let slice = &self[l..r];

        slice.iter().eq(slice.iter().rev())
    }

    fn reverse_range<R: RangeBounds<usize>>(&mut self, range: R) {
        let (l, r) = range_bounds(range, self.len());
        self[l..r].reverse();
    }

    fn rotate_left_range<R: RangeBounds<usize>>(&mut self, range: R, k: usize) {
        let (l, r) = range_bounds(range, self.len());
        let len = r - l;

        if len > 0 {
            self[l..r].rotate_left(k % len);
        }
    }

    fn rotate_right_range<R: RangeBounds<usize>>(&mut self, range: R, k: usize) {
        let (l, r) = range_bounds(range, self.len());
        let len = r - l;

        if len > 0 {
            self[l..r].rotate_right(k % len);
        }
    }
}

/// `RangeBounds<usize>` を半開区間 `[l, r)` に変換します。
///
/// `..`, `l..`, `..r`, `l..r`, `l..=r` などを受け付けます。
/// 変換後の範囲が `0 <= l <= r <= len` を満たさない場合は panic します。
fn range_bounds<R: RangeBounds<usize>>(range: R, len: usize) -> (usize, usize) {
    let l = match range.start_bound() {
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x.checked_add(1).expect("range start bound overflow"),
        Bound::Unbounded => 0,
    };

    let r = match range.end_bound() {
        Bound::Included(&x) => x.checked_add(1).expect("range end bound overflow"),
        Bound::Excluded(&x) => x,
        Bound::Unbounded => len,
    };

    assert!(l <= r && r <= len, "range out of bounds");

    (l, r)
}

/// 10進数の各桁を表す列を `usize` に変換します。
///
/// `[usize]` と `[char]` に実装されているため、`Vec<usize>` や `Vec<char>` にも
/// そのまま利用できます。要素を左から順に10進数の桁として読み、先頭のゼロも通常の整数と同じ値にします。
/// 空の列、10進数の桁として不正な要素、または `usize` の範囲を超える値に対しては `None` を返します。
/// 計算量は要素数を `N` として `O(N)`、追加メモリは `O(1)` です。
///
/// # Examples
///
/// ```
/// use atcoder::string::DecimalDigitsExt;
///
/// assert_eq!(vec![1, 2, 3].parse_usize(), Some(123));
/// assert_eq!(vec!['4', '5', '6'].parse_usize(), Some(456));
/// ```
pub trait DecimalDigitsExt {
    /// 各要素を10進数の一桁として解釈し、`usize` へ変換します。
    ///
    /// `[usize]` の要素はそれぞれ `0..=9` である必要があります。`[char]` の要素は
    /// [`char::to_digit`] で基数10の数字として認識される必要があります。
    fn parse_usize(&self) -> Option<usize>;
}

impl DecimalDigitsExt for [usize] {
    fn parse_usize(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        self.iter().try_fold(0_usize, |value, &digit| {
            if digit >= 10 {
                return None;
            }
            value.checked_mul(10)?.checked_add(digit)
        })
    }
}

impl DecimalDigitsExt for [char] {
    fn parse_usize(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        self.iter().try_fold(0_usize, |value, &digit| {
            let digit = digit.to_digit(10)? as usize;
            value.checked_mul(10)?.checked_add(digit)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{manacher, AlphaExt, AlphaIndexExt, DecimalDigitsExt};

    #[test]
    fn finds_even_palindrome() {
        assert_eq!(manacher(b"abba")[4] - 1, 4);
        assert_eq!(b'Z'.to_idx(), 25);
    }

    #[test]
    fn converts_alphabet_indices_to_lowercase_letters() {
        assert_eq!(0_usize.to_alpha(), 'a');
        assert_eq!(25_usize.to_alpha(), 'z');
        assert_eq!(0_u8.to_alpha(), 'a');
        assert_eq!(25_u8.to_alpha(), 'z');
    }

    #[test]
    #[should_panic(expected = "alphabet index out of range")]
    fn rejects_invalid_alphabet_index() {
        26_usize.to_alpha();
    }

    #[test]
    fn parses_decimal_digits() {
        assert_eq!([0, 1, 2, 3].parse_usize(), Some(123));
        assert_eq!(['0', '4', '5', '6'].parse_usize(), Some(456));
    }

    #[test]
    fn rejects_invalid_or_overflowing_decimal_digits() {
        assert_eq!(Vec::<usize>::new().parse_usize(), None);
        assert_eq!([1, 10].parse_usize(), None);
        assert_eq!(['1', 'x'].parse_usize(), None);
        assert_eq!(vec![9; usize::BITS as usize].parse_usize(), None);
    }
}
