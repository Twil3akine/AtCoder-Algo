//! 反復実装によるHeavy-Light分解（HLD）です。
//!
//! 木の頂点を並べ替え、部分木を1区間、パスを `O(log N)` 個の区間に分けます。
//! パスの向きと区間の順序を保持するため、非可換な演算にも利用できます。

use std::ops::Range;

use crate::graph::tree::rooted_order;

/// HLD上の半開区間と走査方向です。
///
/// `reversed = false` なら添字の昇順、`true` なら降順に走査します。
/// 非可換な区間集約では、セグメント木などに順方向・逆方向の集約値を持たせ、
/// この向きに対応する値を選んでください。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathSegment {
    /// 並べ替え後の頂点配列における半開区間です。
    pub range: Range<usize>,
    /// この区間を添字の降順に走査する場合に `true` です。
    pub reversed: bool,
}

/// 重みなし無向木をHeavy-Light分解します。
///
/// 頂点番号・並べ替え後の添字はともに0始まりです。構築は反復実装で、
/// 計算量・メモリ量は `O(N)` です。入力の隣接リストは変更しません。
///
/// # Examples
///
/// ```
/// use atcoder::graph::hld::Hld;
///
/// let graph = vec![vec![1], vec![0, 2, 3], vec![1], vec![1]];
/// let tree = Hld::new(&graph, 0);
/// let mut vertices = Vec::new();
/// for segment in tree.path(2, 3) {
///     let mut part: Vec<_> = segment.range.map(|i| tree.vertex(i)).collect();
///     if segment.reversed {
///         part.reverse();
///     }
///     vertices.extend(part);
/// }
/// assert_eq!(vertices, vec![2, 1, 3]);
/// assert_eq!(tree.subtree_range(1).len(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct Hld {
    parent: Vec<usize>,
    depth: Vec<usize>,
    size: Vec<usize>,
    head: Vec<usize>,
    index: Vec<usize>,
    vertices: Vec<usize>,
}

impl Hld {
    /// 隣接リストで与えた木を `root` を根として分解します。
    ///
    /// 入力は空でない無向木であることを前提とし、`root` が範囲外なら panic します。
    /// 計算量・メモリ量は `O(N)` です。
    pub fn new(graph: &[Vec<usize>], root: usize) -> Self {
        let n = graph.len();
        let (parent, depth, order) = rooted_order(graph, root);
        let mut size = vec![1; n];
        let mut heavy = vec![None; n];
        for &v in order.iter().rev() {
            let mut largest = 0;
            for &to in &graph[v] {
                if to == parent[v] {
                    continue;
                }
                size[v] += size[to];
                if size[to] > largest {
                    largest = size[to];
                    heavy[v] = Some(to);
                }
            }
        }

        let mut head = vec![root; n];
        let mut index = vec![0; n];
        let mut vertices = Vec::with_capacity(n);
        let mut stack = vec![(root, root)];
        while let Some((v, chain_head)) = stack.pop() {
            head[v] = chain_head;
            index[v] = vertices.len();
            vertices.push(v);
            for &to in graph[v].iter().rev() {
                if to != parent[v] && Some(to) != heavy[v] {
                    stack.push((to, to));
                }
            }
            // heavy edgeを先に訪れ、各部分木も連続した区間にします。
            if let Some(to) = heavy[v] {
                stack.push((to, chain_head));
            }
        }
        Self {
            parent,
            depth,
            size,
            head,
            index,
            vertices,
        }
    }

    /// 頂点 `v` の並べ替え後の添字を返します。計算量は `O(1)` です。
    /// `v` が範囲外なら panic します。
    pub fn index(&self, v: usize) -> usize {
        self.index[v]
    }

    /// 並べ替え後の添字 `index` に対応する頂点を返します。計算量は `O(1)` です。
    /// `index` が範囲外なら panic します。
    pub fn vertex(&self, index: usize) -> usize {
        self.vertices[index]
    }

    /// `v` 自身を含む部分木の半開区間を返します。計算量は `O(1)` です。
    /// `v` が範囲外なら panic します。
    pub fn subtree_range(&self, v: usize) -> Range<usize> {
        self.index[v]..self.index[v] + self.size[v]
    }

    /// `u` から `v` への頂点パスを、両端を含む区間列で返します。
    ///
    /// 区間列を先頭から、各区間を指定された向きに走査すると、`u` から `v` への
    /// 頂点列になります。非可換な演算でも、この順序で集約してください。
    /// 同一頂点には長さ1の区間を返します。計算量・返す区間数は `O(log N)` です。
    /// いずれかの頂点番号が範囲外なら panic します。
    pub fn path(&self, u: usize, v: usize) -> Vec<PathSegment> {
        self.path_segments(u, v, false)
    }

    /// `u` から `v` への辺パスを区間列で返します。
    ///
    /// 各辺の値を、根から見て子側の頂点の添字に置いて利用します。
    /// 最小共通祖先の添字を除き、区間列と走査方向は [`path`](Self::path) と同じです。
    /// 同一頂点には空配列を返します。計算量・返す区間数は `O(log N)` です。
    /// いずれかの頂点番号が範囲外なら panic します。
    pub fn edge_path(&self, u: usize, v: usize) -> Vec<PathSegment> {
        self.path_segments(u, v, true)
    }

    fn path_segments(&self, mut u: usize, mut v: usize, edge: bool) -> Vec<PathSegment> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        while self.head[u] != self.head[v] {
            if self.depth[self.head[u]] >= self.depth[self.head[v]] {
                left.push(PathSegment {
                    range: self.index[self.head[u]]..self.index[u] + 1,
                    reversed: true,
                });
                u = self.parent[self.head[u]];
            } else {
                right.push(PathSegment {
                    range: self.index[self.head[v]]..self.index[v] + 1,
                    reversed: false,
                });
                v = self.parent[self.head[v]];
            }
        }
        let (range, reversed) = if self.depth[u] >= self.depth[v] {
            (self.index[v] + usize::from(edge)..self.index[u] + 1, true)
        } else {
            (self.index[u] + usize::from(edge)..self.index[v] + 1, false)
        };
        if !range.is_empty() {
            left.push(PathSegment { range, reversed });
        }
        left.extend(right.into_iter().rev());
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structure::segment_tree::SegmentTree;
    use crate::graph::tree::path;

    fn expand(tree: &Hld, segments: Vec<PathSegment>) -> Vec<usize> {
        let mut vertices = Vec::new();
        for segment in segments {
            let mut part: Vec<_> = segment.range.map(|i| tree.vertex(i)).collect();
            if segment.reversed {
                part.reverse();
            }
            vertices.extend(part);
        }
        vertices
    }

    #[test]
    fn singleton() {
        let tree = Hld::new(&[vec![]], 0);
        assert_eq!(tree.index(0), 0);
        assert_eq!(tree.vertex(0), 0);
        assert_eq!(tree.subtree_range(0), 0..1);
        assert_eq!(expand(&tree, tree.path(0, 0)), vec![0]);
        assert!(tree.edge_path(0, 0).is_empty());
    }

    #[test]
    fn paths_edges_and_subtrees_match_naive_traversal() {
        use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
        let mut rng = StdRng::seed_from_u64(123);
        for n in 2..30 {
            let mut graph = vec![vec![]; n];
            for v in 1..n {
                let parent = rng.gen_range(0..v);
                graph[parent].push(v);
                graph[v].push(parent);
            }
            for neighbors in &mut graph {
                neighbors.shuffle(&mut rng);
            }
            for root in [0, n - 1] {
                let tree = Hld::new(&graph, root);
                for u in 0..n {
                    assert_eq!(tree.vertex(tree.index(u)), u);
                    let mut actual: Vec<_> =
                        tree.subtree_range(u).map(|i| tree.vertex(i)).collect();
                    actual.sort_unstable();
                    let expected: Vec<_> = (0..n)
                        .filter(|&v| path(&graph, root, v).contains(&u))
                        .collect();
                    assert_eq!(actual, expected);
                    for v in 0..n {
                        let vertices = path(&graph, u, v);
                        assert_eq!(expand(&tree, tree.path(u, v)), vertices);
                        let edges: Vec<_> = vertices
                            .windows(2)
                            .map(|e| {
                                if tree.depth[e[0]] > tree.depth[e[1]] {
                                    e[0]
                                } else {
                                    e[1]
                                }
                            })
                            .collect();
                        assert_eq!(expand(&tree, tree.edge_path(u, v)), edges);
                    }
                }
            }
        }
    }

    #[test]
    fn noncommutative_segment_tree_products_follow_path_direction() {
        // 一次関数の合成を、順方向・逆方向の組としてセグメント木に持たせます。
        fn compose(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
            (a.0 * b.0 % 101, (a.0 * b.1 + a.1) % 101)
        }
        let graph = vec![
            vec![1, 2],
            vec![0, 3, 4],
            vec![0, 5],
            vec![1],
            vec![1],
            vec![2],
        ];
        let tree = Hld::new(&graph, 0);
        let values: Vec<_> = (0..graph.len())
            .map(|v| (v as u64 + 2, v as u64 + 1))
            .collect();
        let ordered: Vec<_> = (0..graph.len())
            .map(|i| {
                let value = values[tree.vertex(i)];
                (value, value)
            })
            .collect();
        let segment_tree = SegmentTree::from_vec(
            ordered,
            |a, b| (compose(a.0, b.0), compose(b.1, a.1)),
            ((1, 0), (1, 0)),
        );
        for u in 0..graph.len() {
            for v in 0..graph.len() {
                let mut actual = (1, 0);
                for segment in tree.path(u, v) {
                    let (forward, backward) =
                        segment_tree.get(segment.range.start, segment.range.end);
                    actual = compose(actual, if segment.reversed { backward } else { forward });
                }
                let expected = path(&graph, u, v)
                    .iter()
                    .fold((1, 0), |acc, &v| compose(acc, values[v]));
                assert_eq!(actual, expected);
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
        let tree = Hld::new(&graph, 0);
        assert_eq!(tree.subtree_range(1), 1..n);
        assert_eq!(
            expand(&tree, tree.path(n - 1, 0)),
            (0..n).rev().collect::<Vec<_>>()
        );
        assert_eq!(
            expand(&tree, tree.edge_path(0, n - 1)),
            (1..n).collect::<Vec<_>>()
        );
    }
}
