//! 無向グラフの橋と関節点を求めるLowLinkです。
//!
//! 多重辺と自己ループを区別するため、辺には入力順の番号を割り当てます。
//! 再帰を使わずにDFSの帰りがけ処理を行います。

/// LowLinkで検出した橋・関節点です。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LowLinkResult {
    /// 橋となる辺の入力添字（0始まり、昇順）です。
    pub bridges: Vec<usize>,
    /// 関節点の頂点番号（0始まり、昇順）です。
    pub articulation_points: Vec<usize>,
}

/// 無向グラフの橋と関節点を求めます。
///
/// `n` は頂点数、`edges` は両端頂点の組です。非連結グラフ・多重辺・自己ループも扱います。
/// 橋は削除すると連結成分の数が増える辺、関節点は削除すると残ったグラフの
/// 連結成分の数が増える頂点です。計算量・メモリ量は `O(V + E)` です。
///
/// # Examples
///
/// ```
/// use atcoder::graph::low_link::low_link;
///
/// let r = low_link(4, &[(0, 1), (1, 2), (2, 0), (2, 3)]);
/// assert_eq!(r.bridges, vec![3]);
/// assert_eq!(r.articulation_points, vec![2]);
/// ```
pub fn low_link(n: usize, edges: &[(usize, usize)]) -> LowLinkResult {
    let mut graph = vec![Vec::<(usize, usize)>::new(); n];
    for (id, &(u, v)) in edges.iter().enumerate() {
        graph[u].push((v, id));
        graph[v].push((u, id));
    }

    let mut ord = vec![usize::MAX; n];
    let mut low = vec![0usize; n];
    let mut parent = vec![usize::MAX; n];
    let mut parent_edge = vec![usize::MAX; n];
    let mut children = vec![0usize; n];
    let mut articulation = vec![false; n];
    let mut bridges = Vec::new();
    let mut timer = 0usize;

    for root in 0..n {
        if ord[root] != usize::MAX {
            continue;
        }
        ord[root] = timer;
        low[root] = timer;
        timer += 1;
        let mut stack = vec![(root, 0usize)];

        while let Some(&(v, next)) = stack.last() {
            if next < graph[v].len() {
                stack.last_mut().unwrap().1 += 1;
                let (to, edge_id) = graph[v][next];
                if parent_edge[v] == edge_id {
                    continue;
                }
                if ord[to] == usize::MAX {
                    parent[to] = v;
                    parent_edge[to] = edge_id;
                    children[v] += 1;
                    ord[to] = timer;
                    low[to] = timer;
                    timer += 1;
                    stack.push((to, 0));
                } else {
                    low[v] = low[v].min(ord[to]);
                }
            } else {
                stack.pop();
                if parent[v] == usize::MAX {
                    if children[v] >= 2 {
                        articulation[v] = true;
                    }
                } else {
                    let p = parent[v];
                    low[p] = low[p].min(low[v]);
                    if low[v] > ord[p] {
                        bridges.push(parent_edge[v]);
                    }
                    if parent[p] != usize::MAX && low[v] >= ord[p] {
                        articulation[p] = true;
                    }
                }
            }
        }
    }
    bridges.sort_unstable();
    LowLinkResult {
        bridges,
        articulation_points: (0..n).filter(|&v| articulation[v]).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::low_link;

    #[test]
    fn cycle_with_tail_and_parallel_edges() {
        let r = low_link(5, &[(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (3, 4)]);
        assert_eq!(r.bridges, vec![3]);
        assert_eq!(r.articulation_points, vec![2, 3]);
    }

    #[test]
    fn disconnected_roots_and_self_loop() {
        let r = low_link(5, &[(0, 1), (0, 2), (3, 3)]);
        assert_eq!(r.bridges, vec![0, 1]);
        assert_eq!(r.articulation_points, vec![0]);
    }

    #[test]
    fn deep_chain_uses_no_recursion() {
        let n = 200_000;
        let r = low_link(n, &(1..n).map(|v| (v - 1, v)).collect::<Vec<_>>());
        assert_eq!(r.bridges.len(), n - 1);
        assert_eq!(r.articulation_points.len(), n - 2);
    }
}
