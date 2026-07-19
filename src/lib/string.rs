//! 文字列と列に対するアルゴリズムです。

use std::ops::{Bound, RangeBounds};

/// ASCII 英字を `0..26` の添字へ変換します。
///
/// `char` と `u8` に実装されています。ASCII 英字以外を渡した結果は意味を持ちません。
pub trait AlphaExt {
    /// 大文字・小文字を区別せず、`a` を `0`、`z` を `25` として返します。
    fn alphabet_index(self) -> usize;

    /// 大文字・小文字を区別せず、`a` を `0`、`z` を `25` として返します。
    ///
    /// [`alphabet_index`](Self::alphabet_index) と同じです。
    fn to_idx(self) -> usize;
}

impl AlphaExt for char {
    fn alphabet_index(self) -> usize {
        (self.to_ascii_lowercase() as u8 - b'a') as usize
    }

    fn to_idx(self) -> usize {
        self.alphabet_index()
    }
}

impl AlphaExt for u8 {
    fn alphabet_index(self) -> usize {
        (self.to_ascii_lowercase() - b'a') as usize
    }

    fn to_idx(self) -> usize {
        self.alphabet_index()
    }
}

/// Manacher 法で奇数長・偶数長の回文半径をまとめて返します。
///
/// 戻り値の長さは `2 * sequence.len() + 1` です。偶数添字は要素間、奇数添字は元の要素を
/// 中心とします。`radii[i]` は区切りを挿入した列上で中心自身を含む半径であり、対応する
/// 元の回文長は `radii[i] - 1` です。計算量と追加メモリは `O(N)` です。
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
pub trait SequenceExt<T> {
    /// 指定した区間が回文かどうかを返します。
    ///
    /// 指定区間の先頭と末尾から順に要素を比較します。
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

    /// 指定した区間を反転します。
    ///
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

    /// 指定した区間を左に `k` 個巡回シフトします。
    ///
    /// `k` が区間長以上の場合は、区間長で割った余りだけシフトします。
    /// 空区間に対して呼び出した場合は何もしません。
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

    /// 指定した区間を右に `k` 個巡回シフトします。
    ///
    /// `k` が区間長以上の場合は、区間長で割った余りだけシフトします。
    /// 空区間に対して呼び出した場合は何もしません。
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

#[cfg(test)]
mod tests {
    use super::{manacher, AlphaExt};

    #[test]
    fn finds_even_palindrome() {
        assert_eq!(manacher(b"abba")[4] - 1, 4);
        assert_eq!(b'Z'.alphabet_index(), 25);
    }
}
