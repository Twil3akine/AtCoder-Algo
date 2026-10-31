//! 隣接リストの順序を保つ、汎用の全方位木DPです。

use crate::graph::tree::rooted_order;

/// 各頂点を根にした木DPの結果を、頂点番号順に返します。
///
/// `graph` は0始まりの無向木の隣接リストです。空の木には空配列を返します。
/// `merge(&a, &b)` は隣接頂点からの寄与を結合し、`identity` は左右両方の単位元です。
/// 結合則が必要ですが、可換である必要はありません。各頂点では `graph[v]` の順序で
/// 結合し、親方向の寄与もその隣接位置に入れます。
///
/// `add_vertex(&merged, v)` は寄与の集約値に頂点 `v` を加え、型 `U` のDP値を作ります。
/// `add_edge(&value, from, to)` は `from` 側のDP値を、辺を渡った先の `to` への寄与に
/// 変換します。頂点の重みや辺の重みはクロージャで参照できます。
///
/// 反復実装で、prefix・suffixの集約により各隣接頂点を定数回だけ処理します。
/// 各演算と `T::clone` が `O(1)` なら、計算量・メモリ量は `O(N)` です。
///
/// # Examples
///
/// 各頂点から全頂点への距離の和を、`(距離の和, 頂点数)` で計算します。
///
/// ```
/// use atcoder::graph::rerooting::rerooting;
///
/// let graph = vec![vec![1], vec![0, 2], vec![1]];
/// let dp = rerooting(
///     &graph,
///     (0usize, 0usize),
///     |a, b| (a.0 + b.0, a.1 + b.1),
///     |value, _v| (value.0, value.1 + 1),
///     |value, _from, _to| (value.0 + value.1, value.1),
/// );
/// assert_eq!(dp, vec![(3, 3), (2, 3), (3, 3)]);
/// ```
pub fn rerooting<T, U, Merge, AddVertex, AddEdge>(
    graph: &[Vec<usize>],
    identity: T,
    merge: Merge,
    add_vertex: AddVertex,
    add_edge: AddEdge,
) -> Vec<U>
where
    T: Clone,
    Merge: Fn(&T, &T) -> T,
    AddVertex: Fn(&T, usize) -> U,
    AddEdge: Fn(&U, usize, usize) -> T,
{
    let n = graph.len();
    if n == 0 {
        return Vec::new();
    }
    let (parent, _, order) = rooted_order(graph, 0);
    let mut down = vec![identity.clone(); n];
    for &v in order.iter().rev() {
        let mut merged = identity.clone();
        for &to in &graph[v] {
            if to != parent[v] {
                merged = merge(&merged, &down[to]);
            }
        }
        if v != 0 {
            down[v] = add_edge(&add_vertex(&merged, v), v, parent[v]);
        }
    }

    let mut up = vec![identity.clone(); n];
    let mut result: Vec<Option<U>> = (0..n).map(|_| None).collect();
    for &v in &order {
        let mut prefix = Vec::with_capacity(graph[v].len() + 1);
        prefix.push(identity.clone());
        for &to in &graph[v] {
            let value = if to == parent[v] { &up[v] } else { &down[to] };
            prefix.push(merge(prefix.last().unwrap(), value));
        }
        result[v] = Some(add_vertex(prefix.last().unwrap(), v));

        let mut suffix = identity.clone();
        for (i, &to) in graph[v].iter().enumerate().rev() {
            if to != parent[v] {
                let without_child = merge(&prefix[i], &suffix);
                up[to] = add_edge(&add_vertex(&without_child, v), v, to);
            }
            let value = if to == parent[v] { &up[v] } else { &down[to] };
            suffix = merge(value, &suffix);
        }
    }
    result.into_iter().map(Option::unwrap).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::traversal::bfs;

    fn distance_sums(graph: &[Vec<usize>]) -> Vec<(usize, usize)> {
        rerooting(
            graph,
            (0, 0),
            |a, b| (a.0 + b.0, a.1 + b.1),
            |value, _| (value.0, value.1 + 1),
            |value, _, _| (value.0 + value.1, value.1),
        )
    }

    #[test]
    fn empty_and_singleton() {
        assert!(distance_sums(&[]).is_empty());
        assert_eq!(distance_sums(&[vec![]]), vec![(0, 1)]);
    }

    #[test]
    fn distance_sums_match_bfs() {
        use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
        let mut rng = StdRng::seed_from_u64(456);
        for n in 2..60 {
            let mut graph = vec![vec![]; n];
            for v in 1..n {
                let parent = rng.gen_range(0..v);
                graph[parent].push(v);
                graph[v].push(parent);
            }
            for neighbors in &mut graph {
                neighbors.shuffle(&mut rng);
            }
            let expected: Vec<_> = (0..n)
                .map(|v| (bfs(&graph, v).iter().map(|d| d.unwrap()).sum(), n))
                .collect();
            assert_eq!(distance_sums(&graph), expected);
        }
    }

    #[test]
    fn noncommutative_merge_preserves_parent_position_and_edge_direction() {
        fn naive(graph: &[Vec<usize>], v: usize, parent: usize) -> String {
            let mut merged = String::new();
            for &to in &graph[v] {
                if to != parent {
                    merged.push_str(&format!("{}>{}:{};", to, v, naive(graph, to, v)));
                }
            }
            format!("{v}[{merged}]")
        }
        let graph = vec![
            vec![2, 1],
            vec![3, 0, 4],
            vec![5, 0],
            vec![1],
            vec![1],
            vec![2],
        ];
        // 集約値Tと頂点DP値Uを別の型にし、UにはCloneも要求しません。
        struct Value(String);
        let actual = rerooting(
            &graph,
            String::new(),
            |a, b| format!("{a}{b}"),
            |value, v| Value(format!("{v}[{value}]")),
            |value, from, to| format!("{from}>{to}:{};", value.0),
        );
        for (v, value) in actual.iter().enumerate() {
            assert_eq!(value.0, naive(&graph, v, graph.len()));
        }
    }

    #[test]
    fn weighted_distances_use_edge_and_vertex_data() {
        let graph = vec![vec![1], vec![0, 2], vec![1]];
        let weights = [2i64, 3, 5];
        let dp = rerooting(
            &graph,
            (0, 0),
            |a, b| (a.0 + b.0, a.1 + b.1),
            |value, v| (value.0, value.1 + weights[v]),
            |value, from, to| {
                let length = if from.min(to) == 0 { 4 } else { 7 };
                (value.0 + value.1 * length, value.1)
            },
        );
        assert_eq!(dp, vec![(67, 10), (43, 10), (43, 10)]);
    }

    #[test]
    fn long_chain_uses_no_recursion() {
        let n = 200_000;
        let mut graph = vec![vec![]; n];
        for v in 1..n {
            graph[v - 1].push(v);
            graph[v].push(v - 1);
        }
        let dp = distance_sums(&graph);
        for (v, &(sum, count)) in dp.iter().enumerate() {
            assert_eq!(sum, v * (v + 1) / 2 + (n - 1 - v) * (n - v) / 2);
            assert_eq!(count, n);
        }
    }

    #[test]
    fn high_degree_tree_uses_linear_number_of_merges() {
        use std::cell::Cell;
        let n = 200_000;
        let mut graph = vec![vec![]; n];
        for v in 1..n {
            graph[0].push(v);
            graph[v].push(0);
        }
        let calls = Cell::new(0);
        let dp = rerooting(
            &graph,
            0,
            |a, b| {
                calls.set(calls.get() + 1);
                a + b
            },
            |value, _| value + 1,
            |value, _, _| *value,
        );
        assert!(dp.iter().all(|&value| value == n));
        assert!(calls.get() <= 6 * n);
    }
}
