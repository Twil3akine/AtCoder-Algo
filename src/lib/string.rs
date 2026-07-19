//! 文字列と列に対するアルゴリズムです。

/// ASCII 英字を `0..26` の添字へ変換します。
///
/// `char` と `u8` に実装されています。ASCII 英字以外を渡した結果は意味を持ちません。
pub trait AlphaExt {
    /// 大文字・小文字を区別せず、`a` を `0`、`z` を `25` として返します。
    fn alphabet_index(self) -> usize;
}

impl AlphaExt for char {
    fn alphabet_index(self) -> usize {
        (self.to_ascii_lowercase() as u8 - b'a') as usize
    }
}

impl AlphaExt for u8 {
    fn alphabet_index(self) -> usize {
        (self.to_ascii_lowercase() - b'a') as usize
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

#[cfg(test)]
mod tests {
    use super::{manacher, AlphaExt};

    #[test]
    fn finds_even_palindrome() {
        assert_eq!(manacher(b"abba")[4] - 1, 4);
        assert_eq!(b'Z'.alphabet_index(), 25);
    }
}
