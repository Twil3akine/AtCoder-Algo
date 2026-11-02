//! 有向グラフの強連結成分分解（SCC）です。
//!
//! 辺をたどって互いに到達できる頂点を同じ成分へまとめます。
//! Kosaraju法を反復DFSで実装し、深いグラフでもスタックを消費しません。

/// 強連結成分分解の結果です。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SccResult {
    /// 各頂点が属する成分番号です。成分番号は0始まりです。
    pub ids: Vec<usize>,
    /// 各成分に属する頂点です。成分間の辺は番号の小さい成分から大きい成分へ向かいます。
    pub groups: Vec<Vec<usize>>,
}

/// 有向グラフを強連結成分へ分解します。
///
/// グラフは \`graph[v] = v から出る辺の行き先\` で与えます。
/// 重複辺と自己ループを含むグラフも扱えます。計算量・メモリ量は \`O(V + E)\` です。
///
/// # Examples
///
/// \`\`\`
/// use atcoder::graph::scc::scc;
///
/// let graph = vec![vec![1], vec![0, 2], vec![3], vec![2]];
/// let result = scc(&graph);
/// assert_eq!(result.groups, vec![vec![0, 1], vec![2, 3]]);
/// assert_eq!(result.ids, vec![0, 0, 1, 1]);
/// \`\`\`
pub fn scc(graph: &[Vec<usize>]) -> SccResult {
    let n = graph.len();
    let mut rev = vec![Vec::new(); n];
    for (v, edges) in graph.iter().enumerate() {
        for &to in edges {
            rev[to].push(v);
        }
    }

    let mut visited = vec![false; n];
    let mut order = Vec::with_capacity(n);

    for start in 0..n {
        if visited[start] {
            continue;
        }
        visited[start] = true;
        // (頂点, 次に調べる辺の添字)を保持して帰りがけ順を作ります。
        let mut stack = vec![(start, 0usize)];
        while let Some(&(v, next)) = stack.last() {
            if next == graph[v].len() {
                order.push(v);
                stack.pop();
                continue;
            }
            stack.last_mut().unwrap().1 += 1;
            let to = graph[v][next];
            if !visited[to] {
                visited[to] = true;
                stack.push((to, 0));
            }
        }
    }

    let mut ids = vec![usize::MAX; n];
    let mut groups = Vec::new();
    for &start in order.iter().rev() {
        if ids[start] != usize::MAX {
            continue;
        }
        let id = groups.len();
        let mut group = Vec::new();
        let mut stack = vec![start];
        ids[start] = id;
        while let Some(v) = stack.pop() {
            group.push(v);
            for &to in &rev[v] {
                if ids[to] == usize::MAX {
                    ids[to] = id;
                    stack.push(to);
                }
            }
        }
        group.sort_unstable();
        groups.push(group);
    }
    SccResult { ids, groups }
}

#[cfg(test)]
mod tests {
    use super::scc;

    #[test]
    fn cycles_and_one_way_edges() {
        let g = vec![vec![1], vec![0, 2], vec![3], vec![2], vec![]];
        let r = scc(&g);
        assert_eq!(r.ids[0], r.ids[1]);
        assert_eq!(r.ids[2], r.ids[3]);
        assert!(r.ids[1] < r.ids[2]);
        assert_ne!(r.ids[4], r.ids[0]);
        assert_eq!(r.groups.len(), 3);
    }

    #[test]
    fn empty_and_deep_path() {
        assert!(scc(&[]).groups.is_empty());
        let n = 200_000;
        let mut g = vec![Vec::new(); n];
        for i in 1..n {
            g[i - 1].push(i);
        }
        let r = scc(&g);
        assert_eq!(r.groups.len(), n);
        assert_eq!(r.ids[0], 0);
        assert_eq!(r.ids[n - 1], n - 1);
    }
}
