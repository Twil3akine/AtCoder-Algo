//! 点更新・区間集約を行うセグメント木です。

use std::ops::Index;

/// モノイド `(T, op, identity)` 上のセグメント木です。
///
/// 添字は 0-indexed、区間は半開区間 `[left, right)` です。`op` は結合則を満たし、
/// `identity` はその単位元である必要があります。構築は `O(N)`、点更新・区間取得・
/// 二分探索は `O(log N)` です。
///
/// # Examples
///
/// ```
/// use atcoder::data_structure::segment_tree::SegmentTree;
///
/// let mut tree = SegmentTree::from_vec([1, 2, 3, 4], |a, b| a + b, 0);
/// assert_eq!(tree.fold(1, 4), 9);
/// tree.set(2, 10);
/// assert_eq!(tree.fold(0, 3), 13);
/// ```
#[derive(Clone)]
pub struct SegmentTree<T, F> {
    op: F,
    identity: T,
    len: usize,
    size: usize,
    data: Vec<T>,
}

impl<T: Copy, F: Fn(T, T) -> T> SegmentTree<T, F> {
    /// 長さ `len`、全要素が `identity` の木を作ります。計算量は `O(N)` です。
    pub fn new(len: usize, op: F, identity: T) -> Self {
        let size = len.next_power_of_two().max(1);
        Self {
            op,
            identity,
            len,
            size,
            data: vec![identity; size * 2],
        }
    }

    /// 配列から木を構築します。計算量は `O(N)` です。
    pub fn from_vec(values: impl Into<Vec<T>>, op: F, identity: T) -> Self {
        let values = values.into();
        let len = values.len();
        let size = len.next_power_of_two().max(1);
        let mut data = vec![identity; size * 2];
        data[size..size + len].copy_from_slice(&values);
        for index in (1..size).rev() {
            data[index] = op(data[index << 1], data[index << 1 | 1]);
        }
        Self {
            op,
            identity,
            len,
            size,
            data,
        }
    }

    /// 要素数を返します。
    pub const fn len(&self) -> usize {
        self.len
    }

    /// 要素がなければ `true` を返します。
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// `index` 番目を `value` に更新します。
    ///
    /// `index >= len` の場合は panic します。
    pub fn set(&mut self, index: usize, value: T) {
        assert!(index < self.len, "index out of bounds");
        let mut index = index + self.size;
        self.data[index] = value;
        while index > 1 {
            index >>= 1;
            self.data[index] = (self.op)(self.data[index << 1], self.data[index << 1 | 1]);
        }
    }

    /// `index` 番目の値を返します。範囲外なら panic します。
    pub fn get(&self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        self.data[self.size + index]
    }

    /// 半開区間 `[left, right)` の集約値を返します。
    ///
    /// `left > right` または `right > len` の場合は panic します。空区間には単位元を返します。
    pub fn fold(&self, left: usize, right: usize) -> T {
        assert!(left <= right && right <= self.len, "range out of bounds");
        let mut left = left + self.size;
        let mut right = right + self.size;
        let mut left_value = self.identity;
        let mut right_value = self.identity;
        while left < right {
            if left & 1 == 1 {
                left_value = (self.op)(left_value, self.data[left]);
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                right_value = (self.op)(self.data[right], right_value);
            }
            left >>= 1;
            right >>= 1;
        }
        (self.op)(left_value, right_value)
    }

    /// `predicate(fold(left, right))` が真である最大の `right` を返します。
    ///
    /// `predicate(identity)` は真で、集約区間を伸ばしたとき単調である必要があります。
    pub fn max_right<P>(&self, mut left: usize, predicate: P) -> usize
    where
        P: Fn(T) -> bool,
    {
        assert!(left <= self.len, "index out of bounds");
        assert!(predicate(self.identity), "predicate(identity) must be true");
        if left == self.len {
            return self.len;
        }
        left += self.size;
        let mut value = self.identity;
        loop {
            while left & 1 == 0 {
                left >>= 1;
            }
            let next = (self.op)(value, self.data[left]);
            if !predicate(next) {
                while left < self.size {
                    left <<= 1;
                    let next = (self.op)(value, self.data[left]);
                    if predicate(next) {
                        value = next;
                        left += 1;
                    }
                }
                return (left - self.size).min(self.len);
            }
            value = next;
            left += 1;
            if left.is_power_of_two() {
                break;
            }
        }
        self.len
    }

    /// `predicate(fold(left, right))` が真である最小の `left` を返します。
    ///
    /// `predicate(identity)` は真で、集約区間を左へ伸ばしたとき単調である必要があります。
    pub fn min_left<P>(&self, mut right: usize, predicate: P) -> usize
    where
        P: Fn(T) -> bool,
    {
        assert!(right <= self.len, "index out of bounds");
        assert!(predicate(self.identity), "predicate(identity) must be true");
        if right == 0 {
            return 0;
        }
        right += self.size;
        let mut value = self.identity;
        loop {
            right -= 1;
            while right > 1 && right % 2 == 1 {
                right >>= 1;
            }
            let next = (self.op)(self.data[right], value);
            if !predicate(next) {
                while right < self.size {
                    right = right << 1 | 1;
                    let next = (self.op)(self.data[right], value);
                    if predicate(next) {
                        value = next;
                        right -= 1;
                    }
                }
                return (right + 1 - self.size).min(self.len);
            }
            value = next;
            if right.is_power_of_two() {
                break;
            }
        }
        0
    }
}

impl<T, F> Index<usize> for SegmentTree<T, F> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "index out of bounds");
        &self.data[self.size + index]
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentTree;

    #[test]
    fn folds_and_searches() {
        let tree = SegmentTree::from_vec(vec![1, 2, 3, 4], |a, b| a + b, 0);
        assert_eq!(tree.fold(1, 3), 5);
        assert_eq!(tree.max_right(0, |sum| sum <= 5), 2);
        assert_eq!(tree.min_left(4, |sum| sum <= 7), 2);
    }
}
