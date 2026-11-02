//! 経路圧縮を使わず、過去の併合状態へ戻せるUnion-Findです。

/// 併合を1回ずつ取り消す操作と、保存時点までの巻き戻しを提供します。
///
/// サイズによる併合を使用します。探索・併合は `O(log N)`、
/// 1操作の取り消しは `O(1)` です。履歴には併合に失敗した操作も含みます。
///
/// # Examples
///
/// ```
/// use atcoder::data_structure::rollback_union_find::RollbackUnionFind;
///
/// let mut uf = RollbackUnionFind::new(4);
/// uf.merge(0, 1);
/// let snap = uf.snapshot();
/// uf.merge(1, 2);
/// assert!(uf.same(0, 2));
/// uf.rollback(snap);
/// assert!(!uf.same(0, 2));
/// ```
#[derive(Debug, Clone)]
pub struct RollbackUnionFind {
    parent_or_size: Vec<isize>,
    history: Vec<Option<(usize, isize, usize, isize)>>,
    groups: usize,
}

impl RollbackUnionFind {
    /// `n` 個の要素を、それぞれ別の集合として作ります。
    pub fn new(n: usize) -> Self {
        Self {
            parent_or_size: vec![-1; n],
            history: Vec::new(),
            groups: n,
        }
    }

    /// 頂点 `x` の属する集合の代表元を返します。計算量は `O(log N)` です。
    pub fn find(&self, mut x: usize) -> usize {
        while self.parent_or_size[x] >= 0 {
            x = self.parent_or_size[x] as usize;
        }
        x
    }

    /// 2つの集合を併合し、新たに併合できた場合に `true` を返します。
    ///
    /// 同じ集合への併合も履歴に1回の操作として記録します。
    /// 計算量は `O(log N)` です。
    pub fn merge(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            self.history.push(None);
            return false;
        }
        // parent_or_size は代表元に負の集合サイズを保持します。
        if self.parent_or_size[a] > self.parent_or_size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.history.push(Some((a, self.parent_or_size[a], b, self.parent_or_size[b])));
        self.parent_or_size[a] += self.parent_or_size[b];
        self.parent_or_size[b] = a as isize;
        self.groups -= 1;
        true
    }

    /// 2頂点が同じ集合に属する場合に `true` を返します。
    pub fn same(&self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    /// 頂点 `v` の属する集合のサイズを返します。
    pub fn size(&self, v: usize) -> usize {
        -self.parent_or_size[self.find(v)] as usize
    }

    /// 現在の連結成分数を返します。
    pub fn group_count(&self) -> usize {
        self.groups
    }

    /// 現在までの操作数をスナップショット番号として返します。
    pub fn snapshot(&self) -> usize {
        self.history.len()
    }

    /// 直前の併合操作を1回取り消します。履歴があれば `true` です。
    pub fn undo(&mut self) -> bool {
        let Some(record) = self.history.pop() else {
            return false;
        };
        if let Some((a, size_a, b, size_b)) = record {
            self.parent_or_size[a] = size_a;
            self.parent_or_size[b] = size_b;
            self.groups += 1;
        }
        true
    }

    /// `snapshot()` で得た番号まで操作を取り消します。
    ///
    /// 現在の操作数より大きい番号を指定するとpanicします。
    /// 取り消す操作数を `K` とすると計算量は `O(K)` です。
    pub fn rollback(&mut self, snapshot: usize) {
        assert!(snapshot <= self.history.len(), "invalid snapshot");
        while self.history.len() > snapshot {
            self.undo();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RollbackUnionFind;

    #[test]
    fn snapshot_and_redundant_merges() {
        let mut uf = RollbackUnionFind::new(4);
        assert!(uf.merge(0, 1));
        let snap = uf.snapshot();
        assert!(!uf.merge(0, 1));
        assert!(uf.merge(1, 2));
        assert_eq!(uf.size(0), 3);
        uf.rollback(snap);
        assert_eq!(uf.group_count(), 3);
        assert!(uf.same(0, 1));
        assert!(!uf.same(0, 2));
        assert!(uf.undo());
        assert_eq!(uf.group_count(), 4);
        assert!(!uf.undo());
    }
}
