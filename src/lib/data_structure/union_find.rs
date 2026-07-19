//! 素集合データ構造（Disjoint Set Union）です。

/// `0..n` の要素を互いに素な集合へ分割して管理します。
///
/// 経路圧縮とサイズによる併合を使用します。各操作の償却計算量は `O(α(N))` です。
/// 添字は 0-indexed で、範囲外の添字を渡すと panic します。
///
/// # Examples
///
/// ```
/// use atcoder::data_structure::union_find::UnionFind;
///
/// let mut dsu = UnionFind::new(4);
/// assert!(dsu.merge(0, 1));
/// assert!(dsu.same(0, 1));
/// assert_eq!(dsu.size(0), 2);
/// assert_eq!(dsu.group_count(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct UnionFind {
    parents: Vec<isize>,
    group_count: usize,
}

impl UnionFind {
    /// `n` 個の単一要素集合を作ります。計算量は `O(N)` です。
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            group_count: n,
        }
    }

    /// `x` の属する集合の代表元を返します。
    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] < 0 {
            x
        } else {
            let root = self.find(self.parents[x] as usize);
            self.parents[x] = root as isize;
            root
        }
    }

    /// `x` と `y` の集合を併合します。
    ///
    /// 新しく併合した場合は `true`、既に同じ集合なら `false` を返します。
    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return false;
        }
        if self.parents[x] > self.parents[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parents[x] += self.parents[y];
        self.parents[y] = x as isize;
        self.group_count -= 1;
        true
    }

    /// `x` と `y` が同じ集合なら `true` を返します。
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// `x` の属する集合の要素数を返します。
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        (-self.parents[root]) as usize
    }

    /// 現在の集合数を返します。計算量は `O(1)` です。
    pub const fn group_count(&self) -> usize {
        self.group_count
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn merges_components() {
        let mut dsu = UnionFind::new(3);
        assert!(dsu.merge(0, 2));
        assert!(!dsu.merge(2, 0));
        assert_eq!(dsu.size(2), 2);
        assert_eq!(dsu.group_count(), 2);
    }
}
