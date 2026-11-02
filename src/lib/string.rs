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

/// KMP法で使用する接頭辞関数を返します。
///
/// `pi[i]` は `s[..=i]` の真の接頭辞と接尾辞が一致する最大長です。
/// 計算量は O(N) です。
///
/// # Examples
///
/// ```
/// use atcoder::string::prefix_function;
/// assert_eq!(prefix_function(b"ababa"), vec![0, 0, 1, 2, 3]);
/// ```
pub fn prefix_function<T: Eq>(s: &[T]) -> Vec<usize> {
    let mut pi = vec![0; s.len()];
    for i in 1..s.len() {
        let mut j = pi[i - 1];
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}

/// KMP法でパターンが現れる開始位置をすべて返します。
///
/// 重なった一致も列挙します。空パターンは `0..=text.len()` に
/// 一致すると定義します。計算量は O(N + M) です。
///
/// # Examples
///
/// ```
/// use atcoder::string::kmp_search;
/// assert_eq!(kmp_search(b"abababa", b"aba"), vec![0, 2, 4]);
/// ```
pub fn kmp_search<T: Eq>(text: &[T], pattern: &[T]) -> Vec<usize> {
    if pattern.is_empty() {
        return (0..=text.len()).collect();
    }
    let pi = prefix_function(pattern);
    let mut positions = Vec::new();
    let mut j = 0;
    for (i, value) in text.iter().enumerate() {
        while j > 0 && *value != pattern[j] {
            j = pi[j - 1];
        }
        if *value == pattern[j] {
            j += 1;
        }
        if j == pattern.len() {
            positions.push(i + 1 - j);
            j = pi[j - 1];
        }
    }
    positions
}

#[cfg(test)]
mod kmp_tests {
    use super::{kmp_search, prefix_function};

    #[test]
    fn overlapping_and_empty_patterns() {
        assert_eq!(prefix_function(b"ababa"), vec![0, 0, 1, 2, 3]);
        assert_eq!(kmp_search(b"abababa", b"aba"), vec![0, 2, 4]);
        assert_eq!(kmp_search(b"abc", b""), vec![0, 1, 2, 3]);
        assert!(kmp_search(b"abc", b"abcd").is_empty());
    }
}

/// Z-algorithm で、各位置から始まる部分文字列と文字列全体との
/// 最長共通接頭辞の長さを返します。
///
/// 結果の `z[i]` は `s[i..]` と `s` の最長共通接頭辞の長さで、`z[0]` は
/// `s.len()` です。計算量は `O(N)` です。`s` の中でパターン `t` が現れる
/// 位置を探すには、`t + sentinel + s` の形に連結して使います。
pub fn z_algorithm<T: Eq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0usize; n];
    if n == 0 {
        return z;
    }
    z[0] = n;
    let (mut l, mut r) = (0usize, 0usize);
    for i in 1..n {
        if i < r {
            z[i] = z[i - l].min(r - i);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

/// ローリングハッシュです。部分文字列のハッシュを `O(1)` で取得できます。
///
/// 法は 2^61 - 1 で、基数は決定的な固定値にしているため、同じ入力からは
/// 常に同じハッシュが得られます。バイト列を対象にします。
pub struct RollingHash {
    base: u64,
    power: Vec<u64>,
    prefix: Vec<u64>,
}

const MOD61: u64 = (1u64 << 61) - 1;

fn mul61(a: u64, b: u64) -> u64 {
    let au = a >> 31;
    let ad = a & ((1u64 << 31) - 1);
    let bu = b >> 31;
    let bd = b & ((1u64 << 31) - 1);
    let mid = au * bd + ad * bu;
    let mut x = au * bu * 2 + (mid >> 30) + ((mid & ((1u64 << 30) - 1)) << 31) + ad * bd;
    x = (x >> 61) + (x & MOD61);
    if x >= MOD61 {
        x - MOD61
    } else {
        x
    }
}

fn mod61(x: u64) -> u64 {
    let x = (x >> 61) + (x & MOD61);
    if x >= MOD61 {
        x - MOD61
    } else {
        x
    }
}

impl RollingHash {
    /// バイト列 `s` のローリングハッシュを構築します。計算量は `O(N)` です。
    pub fn new(s: &[u8]) -> Self {
        // 基数は決定的な値にして、同じ入力には同じハッシュを使う。
        let base = 9113823u64 % MOD61;
        let mut power = vec![1u64; s.len() + 1];
        for i in 0..s.len() {
            power[i + 1] = mul61(power[i], base);
        }
        let mut prefix = vec![0u64; s.len() + 1];
        for (i, &c) in s.iter().enumerate() {
            prefix[i + 1] = mod61(mul61(prefix[i], base) + c as u64 + 1);
        }
        Self {
            base,
            power,
            prefix,
        }
    }

    /// 半開区間 `[left, right)` のハッシュを返します。計算量は `O(1)` です。
    pub fn get(&self, left: usize, right: usize) -> u64 {
        let value =
            self.prefix[right] as i128 - mul61(self.prefix[left], self.power[right - left]) as i128;
        let m = MOD61 as i128;
        ((value % m) + m) as u64 % MOD61
    }

    /// 基数を返します（デバッグ用）。
    pub fn base(&self) -> u64 {
        self.base
    }
}

#[cfg(test)]
mod z_algorithm_and_rolling_hash_tests {
    use super::{z_algorithm, RollingHash};

    #[test]
    fn z_finds_longest_prefix_matches() {
        let z = z_algorithm(b"aaaaa");
        assert_eq!(z, vec![5, 4, 3, 2, 1]);
        let z = z_algorithm(b"abcababc");
        assert_eq!(z[5], 3);
        assert_eq!(z[3], 2);
    }

    #[test]
    fn rolling_hash_matches_equal_substrings() {
        let hash = RollingHash::new(b"abcababcab");
        assert_eq!(hash.get(0, 5), hash.get(5, 10));
        assert_eq!(hash.get(0, 4), hash.get(5, 9));
        assert_ne!(hash.get(0, 4), hash.get(1, 5));
        let single = RollingHash::new(b"x");
        assert_eq!(single.get(0, 1), single.get(0, 1));
    }
}

/// 列の各接頭辞について、先頭と末尾が一致する最長の真の接頭辞長を求めます。
///
/// KMP法で使う接頭辞関数です。`pi[i]` は `sequence[..=i]` の最長の
/// 真の接頭辞（列全体より短い接頭辞）であり、同時に接尾辞でもある長さです。
/// 計算量・追加メモリは `O(N)` です。
///
/// # Examples
///
/// ```
/// use atcoder::string::prefix_function;
///
/// assert_eq!(prefix_function(b"ababa"), vec![0, 0, 1, 2, 3]);
/// ```
pub fn prefix_function<T: Eq>(sequence: &[T]) -> Vec<usize> {
    let n = sequence.len();
    let mut pi = vec![0; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && sequence[i] != sequence[j] {
            j = pi[j - 1];
        }
        if sequence[i] == sequence[j] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}

/// 列 `text` の中に現れる `pattern` の開始添字を昇順で返します。
///
/// KMP法によって検索し、計算量は `O(N + M)`、追加メモリは `O(M)` です。
/// 空パターンは全ての境界に一致するものとし、`0..=text.len()` を返します。
///
/// # Examples
///
/// ```
/// use atcoder::string::kmp_search;
///
/// assert_eq!(kmp_search(b"abababa", b"aba"), vec![0, 2, 4]);
/// ```
pub fn kmp_search<T: Eq>(text: &[T], pattern: &[T]) -> Vec<usize> {
    if pattern.is_empty() {
        return (0..=text.len()).collect();
    }
    let pi = prefix_function(pattern);
    let mut matches = Vec::new();
    let mut matched = 0;
    for (i, element) in text.iter().enumerate() {
        while matched > 0 && element != &pattern[matched] {
            matched = pi[matched - 1];
        }
        if element == &pattern[matched] {
            matched += 1;
        }
        if matched == pattern.len() {
            matches.push(i + 1 - matched);
            matched = pi[matched - 1];
        }
    }
    matches
}

#[cfg(test)]
mod kmp_tests {
    use super::{kmp_search, prefix_function};

    #[test]
    fn prefix_and_overlapping_matches() {
        assert_eq!(prefix_function(b"ababa"), vec![0, 0, 1, 2, 3]);
        assert_eq!(kmp_search(b"abababa", b"aba"), vec![0, 2, 4]);
        assert!(kmp_search(b"xyz", b"a").is_empty());
    }

    #[test]
    fn empty_pattern_matches_boundaries() {
        assert_eq!(kmp_search(b"ab", b""), vec![0, 1, 2]);
        assert_eq!(kmp_search(b"", b""), vec![0]);
        assert!(prefix_function(b"").is_empty());
    }
}
