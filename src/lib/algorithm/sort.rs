//! 降順ソートの拡張メソッドです。

/// スライスへ降順ソートを追加します。
pub trait SortReverse {
    /// 安定性を保って降順に並べます。計算量は `O(N log N)` です。
    fn sort_reverse(&mut self);

    /// 安定性を保たず降順に並べます。計算量は `O(N log N)` です。
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
