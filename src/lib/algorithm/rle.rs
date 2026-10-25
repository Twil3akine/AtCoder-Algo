//! ランレングス圧縮です。

/// 連続する同一要素をまとめるイテレータ拡張です。
///
/// 要素同士を [`PartialEq`] で比較できるイテレータに利用できます。値の並べ替えは行わないため、
/// 離れた位置にある同じ値は別々のグループになります。
pub trait RleExt: Iterator {
    /// イテレータを消費し、`(要素, 連続回数)` の列を返します。
    ///
    /// イテレータを先頭から末尾まで消費します。返り値は出現順で、各回数は 1 以上です。
    /// 空イテレータでは空のベクタを返します。要素数を `N`、連続グループ数を `G` とすると、
    /// 時間計算量は `O(N)`、結果に必要な追加メモリは `O(G)` です。
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
