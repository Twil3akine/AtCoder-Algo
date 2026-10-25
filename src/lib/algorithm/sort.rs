//! 降順ソートの拡張メソッドです。

/// `Ord` を実装する要素のスライスへ降順ソートを追加します。
pub trait SortReverse {
    /// スライス全体を安定な降順に並べ替えます。
    ///
    /// 比較で等しい要素同士の相対順序は保たれます。`N` は要素数で、計算量は `O(N log N)` です。
    fn sort_reverse(&mut self);

    /// スライス全体を安定性を保証せず降順に並べ替えます。
    ///
    /// 比較で等しい要素同士の相対順序は未規定です。`N` は要素数で、計算量は `O(N log N)` です。
    fn sort_unstable_reverse(&mut self);
}

impl<T: Ord> SortReverse for [T] {
    fn sort_reverse(&mut self) {
        self.sort_by(|left, right| right.cmp(left));
    }

    fn sort_unstable_reverse(&mut self) {
        self.sort_unstable_by(|left, right| right.cmp(left));
    }
}

#[cfg(test)]
mod tests {
    use super::SortReverse;

    #[test]
    fn sorts_descending() {
        let mut values = [2, 1, 3];
        values.sort_unstable_reverse();
        assert_eq!(values, [3, 2, 1]);
    }
}
