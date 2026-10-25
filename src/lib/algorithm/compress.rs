//! 座標圧縮です。

/// スライスを大小関係を保った連番へ変換する拡張トレイトです。
///
/// `Ord` を実装する型に利用できます。`[T]` に実装されているため、ベクタや配列からも
/// スライスとして呼び出せます。
pub trait Compress<T> {
    /// 元の値を、重複を除いた昇順列内の順位へ置き換えます。
    ///
    /// # Returns
    ///
    /// `compressed[i]` は元の `self[i]` に対応する 0-indexed の順位です。`values[j]` は順位
    /// `j` の元の値で、昇順かつ重複なしです。要素数を `N` とすると `O(N log N)` です。
    /// 空スライスの場合は、どちらも空のベクタを返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use atcoder::algorithm::compress::Compress;
    ///
    /// let (compressed, values) = [40, 10, 40, 20].compressed();
    /// assert_eq!(compressed, [2, 0, 2, 1]);
    /// assert_eq!(values, [10, 20, 40]);
    /// ```
    fn compressed(&self) -> (Vec<usize>, Vec<T>);
}

impl<T: Ord + Clone> Compress<T> for [T] {
    fn compressed(&self) -> (Vec<usize>, Vec<T>) {
        let mut values = self.to_vec();
        values.sort_unstable();
        values.dedup();
        let compressed = self
            .iter()
            .map(|value| values.binary_search(value).unwrap())
            .collect();
        (compressed, values)
    }
}

#[cfg(test)]
mod tests {
    use super::Compress;

    #[test]
    fn compresses_duplicates() {
        assert_eq!([3, 1, 3].compressed(), (vec![1, 0, 1], vec![1, 3]));
    }
}
