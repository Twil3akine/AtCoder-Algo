//! 区間更新・一点取得を行う双対セグメント木です。

/// 半開区間への作用と一点取得を扱う双対セグメント木です。
///
/// 添字は 0-indexed、区間は `[left, right)` です。`compose(old, new)` は古い作用の後に
/// 新しい作用を適用した合成を返し、結合則と単位元 `identity` を満たす必要があります。
/// 更新と取得はいずれも `O(log N)` です。
///
/// # Examples
///
/// ```
/// use atcoder::data_structure::dual_segment_tree::DualSegmentTree;
///
/// let mut tree = DualSegmentTree::new(4, |a, b| a + b, 0);
/// tree.apply(1, 4, 3);
/// tree.apply(2, 3, 2);
/// assert_eq!(tree.get(2), 5);
/// ```
#[derive(Clone)]
pub struct DualSegmentTree<T, F> {
    compose: F,
    identity: T,
    len: usize,
    size: usize,
    log: usize,
    data: Vec<T>,
}

impl<T: Copy, F: Fn(T, T) -> T> DualSegmentTree<T, F> {
    /// 長さ `len`、全要素が `identity` の木を作ります。計算量は `O(N)` です。
    pub fn new(len: usize, compose: F, identity: T) -> Self {
        Self::from_vec(vec![identity; len], compose, identity)
    }

    /// 各点の初期値を指定して木を構築します。計算量は `O(N)` です。
    pub fn from_vec(values: impl Into<Vec<T>>, compose: F, identity: T) -> Self {
        let values = values.into();
        let len = values.len();
        let size = len.next_power_of_two().max(1);
        let log = size.trailing_zeros() as usize;
        let mut data = vec![identity; size * 2];
        data[size..size + len].copy_from_slice(&values);
        Self {
            compose,
            identity,
            len,
            size,
            log,
            data,
        }
    }

    fn apply_node(&mut self, index: usize, value: T) {
        self.data[index] = (self.compose)(self.data[index], value);
    }

    fn push(&mut self, index: usize) {
        let value = self.data[index];
        self.apply_node(index << 1, value);
        self.apply_node(index << 1 | 1, value);
        self.data[index] = self.identity;
    }

    fn push_path(&mut self, index: usize) {
        let index = index + self.size;
        for height in (1..=self.log).rev() {
            self.push(index >> height);
        }
    }

    /// 半開区間 `[left, right)` に `value` を適用します。
    ///
    /// 範囲が `0..=len` を外れる、または `left > right` の場合は panic します。
    pub fn apply(&mut self, left: usize, right: usize, value: T) {
        assert!(left <= right && right <= self.len, "range out of bounds");
        if left == right {
            return;
        }
        self.push_path(left);
        self.push_path(right - 1);
        let mut left = left + self.size;
        let mut right = right + self.size;
        while left < right {
            if left & 1 == 1 {
                self.apply_node(left, value);
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                self.apply_node(right, value);
            }
            left >>= 1;
            right >>= 1;
        }
    }

    /// `index` 番目の値を返します。範囲外なら panic します。
    pub fn get(&self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        let mut index = index + self.size;
        let mut path = Vec::with_capacity(self.log + 1);
        while index > 0 {
            path.push(index);
            index >>= 1;
        }
        path.into_iter().fold(self.identity, |value, index| {
            (self.compose)(value, self.data[index])
        })
    }
}

/// 矩形更新・一点取得を行う2次元双対セグメント木です。
///
/// 行区間 `[upper, lower)` と列区間 `[left, right)` はともに 0-indexed の半開区間です。
/// `compose` は可換である必要があります。更新・取得は `O(log H log W)`、メモリは
/// `O(HW)` です。
#[derive(Clone)]
pub struct DualSegmentTree2D<T, F> {
    compose: F,
    identity: T,
    height: usize,
    width: usize,
    row_size: usize,
    column_size: usize,
    data: Vec<T>,
}

impl<T: Copy, F: Fn(T, T) -> T> DualSegmentTree2D<T, F> {
    /// `height × width` の木を作ります。
    pub fn new(height: usize, width: usize, compose: F, identity: T) -> Self {
        let row_size = height.next_power_of_two().max(1);
        let column_size = width.next_power_of_two().max(1);
        Self {
            compose,
            identity,
            height,
            width,
            row_size,
            column_size,
            data: vec![identity; 4 * row_size * column_size],
        }
    }

    fn id(&self, row: usize, column: usize) -> usize {
        row * (2 * self.column_size) + column
    }

    fn range_nodes(mut left: usize, mut right: usize, size: usize) -> Vec<usize> {
        left += size;
        right += size;
        let mut nodes = Vec::new();
        while left < right {
            if left & 1 == 1 {
                nodes.push(left);
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                nodes.push(right);
            }
            left >>= 1;
            right >>= 1;
        }
        nodes
    }

    /// 矩形 `[upper, lower) × [left, right)` に `value` を適用します。
    pub fn apply(&mut self, upper: usize, lower: usize, left: usize, right: usize, value: T) {
        assert!(
            upper <= lower && lower <= self.height,
            "row range out of bounds"
        );
        assert!(
            left <= right && right <= self.width,
            "column range out of bounds"
        );
        for row in Self::range_nodes(upper, lower, self.row_size) {
            for column in Self::range_nodes(left, right, self.column_size) {
                let index = self.id(row, column);
                self.data[index] = (self.compose)(self.data[index], value);
            }
        }
    }

    /// 点 `(row, column)` の値を返します。範囲外なら panic します。
    pub fn get(&self, row: usize, column: usize) -> T {
        assert!(
            row < self.height && column < self.width,
            "index out of bounds"
        );
        let mut rows = Vec::new();
        let mut row = row + self.row_size;
        while row > 0 {
            rows.push(row);
            row >>= 1;
        }
        let mut columns = Vec::new();
        let mut column = column + self.column_size;
        while column > 0 {
            columns.push(column);
            column >>= 1;
        }
        let mut result = self.identity;
        for row in rows {
            for &column in &columns {
                result = (self.compose)(result, self.data[self.id(row, column)]);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::{DualSegmentTree, DualSegmentTree2D};

    #[test]
    fn applies_ranges() {
        let mut tree = DualSegmentTree::new(4, |a, b| a + b, 0);
        tree.apply(0, 3, 2);
        tree.apply(1, 4, 5);
        assert_eq!([tree.get(0), tree.get(1), tree.get(3)], [2, 7, 5]);
    }

    #[test]
    fn preserves_non_commutative_order() {
        let compose = |old: (i64, i64), new: (i64, i64)| (new.0 * old.0, new.0 * old.1 + new.1);
        let mut tree = DualSegmentTree::new(2, compose, (1, 0));
        tree.apply(0, 1, (2, 0));
        tree.apply(0, 2, (1, 3));
        assert_eq!(tree.get(0), (2, 3));
    }

    #[test]
    fn applies_rectangles() {
        let mut tree = DualSegmentTree2D::new(2, 3, |a, b| a + b, 0);
        tree.apply(0, 2, 1, 3, 4);
        assert_eq!(tree.get(1, 2), 4);
        assert_eq!(tree.get(0, 0), 0);
    }
}
