//! 更新のない配列の区間最小値・最大値を高速に取得するSparse Tableです。

/// べき等な結合演算の静的区間クエリを処理します。
///
/// `op` は結合則とべき等性（`op(x, x) == x`）を満たす必要があります。
/// 例えば `min`、`max`、`gcd` に利用できますが、一般の加算には利用できません。
/// 構築は `O(N log N)`、空間計算量は `O(N log N)`、取得は `O(1)` です。
///
/// # Examples
///
/// ```
/// use atcoder::data_structure::sparse_table::SparseTable;
///
/// let st = SparseTable::new(vec![8, 3, 7, 2, 9], |a: i32, b| a.min(b));
/// assert_eq!(st.get(1, 4), 2);
/// ```
pub struct SparseTable<T, F> {
    table: Vec<Vec<T>>,
    logs: Vec<usize>,
    op: F,
}

impl<T: Copy, F: Fn(T, T) -> T> SparseTable<T, F> {
    /// 配列と区間演算を指定して構築します。計算量は `O(N log N)` です。
    pub fn new(values: Vec<T>, op: F) -> Self {
        let n = values.len();
        let mut logs = vec![0usize; n + 1];
        for i in 2..=n {
            logs[i] = logs[i / 2] + 1;
        }

        let mut table = Vec::new();
        if !values.is_empty() {
            table.push(values);
            for k in 1..=logs[n] {
                let half = 1usize << (k - 1);
                let span = 1usize << k;
                let mut row = Vec::with_capacity(n - span + 1);
                for i in 0..=n - span {
                    row.push(op(table[k - 1][i], table[k - 1][i + half]));
                }
                table.push(row);
            }
        }
        Self { table, logs, op }
    }

    /// 元の配列の長さを返します。計算量は `O(1)` です。
    pub fn len(&self) -> usize {
        self.logs.len() - 1
    }

    /// 配列が空の場合に `true` を返します。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 空でない半開区間 `[left, right)` の集約結果を返します。
    ///
    /// 区間が空、または範囲外ならpanicします。計算量は `O(1)` です。
    pub fn get(&self, left: usize, right: usize) -> T {
        assert!(left < right && right <= self.len(), "invalid range");
        let k = self.logs[right - left];
        let span = 1usize << k;
        (self.op)(self.table[k][left], self.table[k][right - span])
    }
}

#[cfg(test)]
mod tests {
    use super::SparseTable;

    #[test]
    fn range_min_max_and_empty() {
        let min = SparseTable::new(vec![8, 3, 7, 2, 9], |a: i32, b| a.min(b));
        assert_eq!(min.get(1, 4), 2);
        assert_eq!(min.get(0, 1), 8);
        let max = SparseTable::new(vec![8, 3, 7, 2, 9], |a: i32, b| a.max(b));
        assert_eq!(max.get(0, 5), 9);
        let empty = SparseTable::<i32, _>::new(vec![], |a: i32, b| a.min(b));
        assert!(empty.is_empty());
    }
}
