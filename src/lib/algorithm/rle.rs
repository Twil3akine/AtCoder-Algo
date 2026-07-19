//! ランレングス圧縮です。

/// 連続する同一要素をまとめるイテレータ拡張です。
pub trait RleExt: Iterator {
    /// イテレータを消費し、`(要素, 連続回数)` の列を返します。
    ///
    /// 要素数を `N` とすると時間・追加メモリともに `O(N)` です。
    ///
    /// # Examples
    ///
    /// ```
    /// use atcoder::algorithm::rle::RleExt;
    ///
    /// assert_eq!("aaabbc".chars().rle(), vec![('a', 3), ('b', 2), ('c', 1)]);
    /// ```
    fn rle(self) -> Vec<(Self::Item, usize)>
    where
        Self: Sized,
        Self::Item: PartialEq;
}

impl<I: Iterator> RleExt for I {
    fn rle(self) -> Vec<(Self::Item, usize)>
    where
        Self: Sized,
        Self::Item: PartialEq,
    {
        let mut iter = self;
        let Some(mut current) = iter.next() else {
            return Vec::new();
        };
        let mut count = 1;
        let mut result = Vec::new();
        for item in iter {
            if item == current {
                count += 1;
            } else {
                result.push((current, count));
                current = item;
                count = 1;
            }
        }
        result.push((current, count));
        result
    }
}
