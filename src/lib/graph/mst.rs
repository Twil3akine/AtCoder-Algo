//! 最小全域木（クラスカル法）です。

use crate::data_structure::union_find::UnionFind;

/// クラスカル法で最小全域木を求め、`(総重み, 採用した辺の列)` を返します。
///
/// 辺は `(重み, 始点, 終点)` の列で与えます。グラフが連結でない場合は、
/// 連結成分ごとの最小全域森を返します（採用した辺の本数で判定できます）。
/// 計算量は `O(E log E)` です。
///
/// # Examples
///
/// ```
/// use atcoder::graph::mst::kruskal;
///
/// let edges = vec![(1, 0, 1), (2, 1, 2), (3, 0, 2)];
/// let (total, chosen) = kruskal(3, &edges);
/// assert_eq!(total, 3);
/// assert_eq!(chosen.len(), 2);
/// ```
pub fn kruskal(n: usize, edges: &[(i64, usize, usize)]) -> (i64, Vec<(i64, usize, usize)>) {
    let mut sorted = edges.to_vec();
    sorted.sort_unstable_by_key(|&(w, _, _)| w);
    let mut uf = UnionFind::new(n);
    let mut total = 0;
    let mut chosen = Vec::new();
    for &(w, u, v) in &sorted {
        if uf.merge(u, v) {
            total += w;
            chosen.push((w, u, v));
        }
    }
    (total, chosen)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_lightest_edges() {
        let edges = vec![(5, 0, 1), (1, 1, 2), (4, 0, 2), (2, 2, 3), (9, 0, 3)];
        let (total, chosen) = kruskal(4, &edges);
        assert_eq!(total, 7);
        assert_eq!(chosen.len(), 3);
    }

    #[test]
    fn forest_for_disconnected_graph() {
        let edges = vec![(1, 0, 1), (1, 2, 3)];
        let (total, chosen) = kruskal(4, &edges);
        assert_eq!(total, 2);
        assert_eq!(chosen.len(), 2);
    }
}
