//! フェニック木（Binary Indexed Tree）です。

use std::ops::{Add, Sub};

/// 一点加算と prefix 和を扱うフェニック木です。
///
/// 添字は 0-indexed です。加算と取得はいずれも `O(log N)` です。転倒数の
/// 数え上げや、頻度表に対する累積和の取得に使います。
///
/// # Examples
///
/// ```
/// use atcoder::data_structure::fenwick_tree::FenwickTree;
///
/// let mut tree = FenwickTree::<i64>::new(5);
/// tree.add(0, 3);
/// tree.add(2, 4);
/// assert_eq!(tree.sum(0, 3), 7);
/// assert_eq!(tree.sum(1, 2), 0);
/// ```
#[derive(Clone)]
pub struct FenwickTree<T> {
    len: usize,
    data: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Default + PartialOrd,
{
    /// 長さ `len`、全要素 0 の木を作ります。
    pub fn new(len: usize) -> Self {
        Self {
            len,
            data: vec![T::default(); len + 1],
        }
    }

    /// 各要素の初期値を指定して木を構築します。計算量は `O(N)` です。
    pub fn from_vec(values: Vec<T>) -> Self {
        let len = values.len();
        let mut data = vec![T::default(); len + 1];
        for (i, value) in values.into_iter().enumerate() {
            data[i + 1] = value;
        }
        for i in 1..=len {
            let j = i + (i & i.wrapping_neg());
            if j <= len {
                data[j] = data[j] + data[i];
            }
        }
        Self { len, data }
    }

    /// 添字 `index` の値に `value` を加えます。
    pub fn add(&mut self, index: usize, value: T) {
        let mut i = index + 1;
        while i <= self.len {
            self.data[i] = self.data[i] + value;
            i += i & i.wrapping_neg();
        }
    }

    /// 添字 `index` の値を返します。
    pub fn at(&self, index: usize) -> T {
        self.sum(index, index + 1)
    }

    /// 半開区間 `[left, right)` の合計を返します。
    pub fn sum(&self, left: usize, right: usize) -> T {
        self.prefix(right) - self.prefix(left)
    }

    fn prefix(&self, mut right: usize) -> T {
        let mut acc = T::default();
        while right > 0 {
            acc = acc + self.data[right];
            right -= right & right.wrapping_neg();
        }
        acc
    }

    /// prefix 和が `value` 以上になる最小の添字を返します。
    ///
    /// 全要素が非負である必要があります。該当する添字が無い場合は
    /// `len` を返します。
    pub fn lower_bound(&self, value: T) -> usize {
        let mut index = 0usize;
        let mut bit = 1usize;
        while bit << 1 <= self.len {
            bit <<= 1;
        }
        let mut acc = T::default();
        while bit > 0 {
            let next = index + bit;
            if next <= self.len && acc + self.data[next] < value {
                acc = acc + self.data[next];
                index = next;
            }
            bit >>= 1;
        }
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_and_sums() {
        let mut tree = FenwickTree::from_vec(vec![1i64, 2, 3, 4]);
        assert_eq!(tree.sum(0, 4), 10);
        tree.add(1, 10);
        assert_eq!(tree.at(1), 12);
        assert_eq!(tree.sum(1, 3), 15);
    }

    #[test]
    fn lower_bound_finds_first_reaching_index() {
        let tree = FenwickTree::from_vec(vec![1i64, 2, 3, 4]);
        // prefix: [1, 3, 6, 10]
        assert_eq!(tree.lower_bound(1), 0);
        assert_eq!(tree.lower_bound(3), 1);
        assert_eq!(tree.lower_bound(6), 2);
        assert_eq!(tree.lower_bound(11), 4);
    }
}
