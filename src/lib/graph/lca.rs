//! ダブリングで根付き木の最小共通祖先・距離・パス上の移動を求めます。

use crate::graph::tree::rooted_order;

/// 重みなし無向木の最小共通祖先（LCA）を管理します。
///
/// 頂点番号は0始まりです。構築は反復実装で、計算量・メモリ量は `O(N log N)`、
/// 祖先・距離・移動の問い合わせは `O(log N)` です。
///
/// # Examples
///
/// ```
/// use atcoder::graph::lca::Lca;
///
/// let graph = vec![vec![1], vec![0, 2, 3], vec![1], vec![1]];
/// let tree = Lca::new(&graph, 0);
/// assert_eq!(tree.lca(2, 3), 1);
/// assert_eq!(tree.distance(2, 3), 2);
/// assert_eq!(tree.kth_ancestor(2, 2), Some(0));
/// assert_eq!(tree.kth_ancestor(2, 3), None);
/// assert_eq!(tree.jump(2, 3, 1), Some(1));
/// ```
#[derive(Debug, Clone)]
pub struct Lca {
    ancestors: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl Lca {
    /// 隣接リストで与えた木を `root` を根として構築します。
    ///
    /// 入力は空でない無向木であることを前提とし、`root` が範囲外なら panic します。
    /// 計算量・メモリ量は `O(N log N)` です。
    pub fn new(graph: &[Vec<usize>], root: usize) -> Self {
        let (parent, depth, _) = rooted_order(graph, root);
        let levels = (usize::BITS - graph.len().leading_zeros()) as usize;
        let mut ancestors = Vec::with_capacity(levels);
        ancestors.push(parent);
        for level in 1..levels {
            let previous = &ancestors[level - 1];
            let next = (0..graph.len()).map(|v| previous[previous[v]]).collect();
            ancestors.push(next);
        }
        Self { ancestors, depth }
    }

    /// 根から `v` への距離を返します。計算量は `O(1)` です。
    /// `v` が範囲外なら panic します。
    pub fn depth(&self, v: usize) -> usize {
        self.depth[v]
    }

    /// `v` から親方向へ `k` 辺進んだ祖先を返します。`k = 0` なら `Some(v)` です。
    ///
    /// 根より先へ進む場合は `None`、`v` が範囲外なら panic します。
    /// 計算量は `O(log N)` です。
    pub fn kth_ancestor(&self, mut v: usize, mut k: usize) -> Option<usize> {
        if k > self.depth[v] {
            return None;
        }
        let mut level = 0;
        while k > 0 {
            if k & 1 == 1 {
                v = self.ancestors[level][v];
            }
            level += 1;
            k >>= 1;
        }
        Some(v)
    }

    /// `u` と `v` の最小共通祖先を返します。計算量は `O(log N)` です。
    /// いずれかの頂点番号が範囲外なら panic します。
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] < self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        u = self.kth_ancestor(u, self.depth[u] - self.depth[v]).unwrap();
        if u == v {
            return u;
        }
        for ancestors in self.ancestors.iter().rev() {
            if ancestors[u] != ancestors[v] {
                u = ancestors[u];
                v = ancestors[v];
            }
        }
        self.ancestors[0][u]
    }

    /// `u` と `v` の距離を辺の本数で返します。計算量は `O(log N)` です。
    /// いずれかの頂点番号が範囲外なら panic します。
    pub fn distance(&self, u: usize, v: usize) -> usize {
        let ancestor = self.lca(u, v);
        (self.depth[u] - self.depth[ancestor]) + (self.depth[v] - self.depth[ancestor])
    }

    /// `u` から `v` へのパスを `k` 辺進んだ頂点を返します。
    ///
    /// `k = 0` なら `Some(u)`、距離を超える場合は `None` です。
    /// 計算量は `O(log N)` です。いずれかの頂点番号が範囲外なら panic します。
    pub fn jump(&self, u: usize, v: usize, k: usize) -> Option<usize> {
        let ancestor = self.lca(u, v);
        let up = self.depth[u] - self.depth[ancestor];
        let distance = up + self.depth[v] - self.depth[ancestor];
        if k > distance {
            None
        } else if k <= up {
            self.kth_ancestor(u, k)
        } else {
            self.kth_ancestor(v, distance - k)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::tree::path;

    #[test]
    fn singleton_and_out_of_path_jumps() {
        let tree = Lca::new(&[vec![]], 0);
        assert_eq!(tree.depth(0), 0);
        assert_eq!(tree.lca(0, 0), 0);
        assert_eq!(tree.distance(0, 0), 0);
        assert_eq!(tree.kth_ancestor(0, 0), Some(0));
        assert_eq!(tree.kth_ancestor(0, usize::MAX), None);
        assert_eq!(tree.jump(0, 0, 0), Some(0));
        assert_eq!(tree.jump(0, 0, usize::MAX), None);
    }

    #[test]
    fn all_queries_match_naive_paths_for_every_root() {
        use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
        let mut rng = StdRng::seed_from_u64(42);
        for n in 2..25 {
            let mut graph = vec![vec![]; n];
            for v in 1..n {
                let parent = rng.gen_range(0..v);
                graph[parent].push(v);
                graph[v].push(parent);
            }
            for neighbors in &mut graph {
                neighbors.shuffle(&mut rng);
            }
            for root in 0..n {
                let tree = Lca::new(&graph, root);
                let root_paths: Vec<_> = (0..n).map(|v| path(&graph, root, v)).collect();
                for u in 0..n {
                    assert_eq!(tree.depth(u), root_paths[u].len() - 1);
                    for k in 0..=n {
                        let expected = root_paths[u].iter().rev().nth(k).copied();
                        assert_eq!(tree.kth_ancestor(u, k), expected);
                    }
                    for v in 0..n {
                        let expected = root_paths[u]
                            .iter()
                            .zip(&root_paths[v])
                            .take_while(|(a, b)| a == b)
                            .last()
                            .unwrap()
                            .0;
                        assert_eq!(tree.lca(u, v), *expected);
                        let vertices = path(&graph, u, v);
                        assert_eq!(tree.distance(u, v), vertices.len() - 1);
                        for k in 0..=vertices.len() {
                            assert_eq!(tree.jump(u, v, k), vertices.get(k).copied());
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn long_chain_uses_no_recursion() {
        let n = 200_000;
        let mut graph = vec![vec![]; n];
        for v in 1..n {
            graph[v - 1].push(v);
            graph[v].push(v - 1);
        }
        let tree = Lca::new(&graph, n - 1);
        assert_eq!(tree.lca(0, n / 2), n / 2);
        assert_eq!(tree.distance(0, n - 1), n - 1);
        assert_eq!(tree.kth_ancestor(0, n - 1), Some(n - 1));
        assert_eq!(tree.jump(0, n - 1, n / 2), Some(n / 2));
    }
}
