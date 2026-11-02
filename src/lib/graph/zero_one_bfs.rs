//! 重みが0または1の有向グラフの最短路（0-1 BFS）です。

use std::collections::VecDeque;

/// 始点から各頂点への最短距離を求めます。
///
/// \`graph[v]\` は \`(行き先, 重み)\` の列です。重みは0または1とし、
/// それ以外の重みを見つけた場合はpanicします。到達不能な頂点には \`None\` を返します。
/// 頂点数を \`V\`、辺数を \`E\` とすると計算量は \`O(V + E)\` です。
///
/// # Examples
///
/// \`\`\`
/// use atcoder::graph::zero_one_bfs::zero_one_bfs;
///
/// let graph = vec![vec![(1, 1), (2, 0)], vec![(3, 0)], vec![(1, 0)], vec![]];
/// assert_eq!(zero_one_bfs(&graph, 0), vec![Some(0), Some(0), Some(0), Some(0)]);
/// \`\`\`
pub fn zero_one_bfs(graph: &[Vec<(usize, usize)>], start: usize) -> Vec<Option<usize>> {
    assert!(start < graph.len(), "start out of bounds");
    let mut dist = vec![usize::MAX; graph.len()];
    let mut deque = VecDeque::new();
    dist[start] = 0;
    deque.push_front((start, 0usize));

    while let Some((v, d)) = deque.pop_front() {
        if dist[v] != d {
            continue;
        }
        for &(to, weight) in &graph[v] {
            assert!(weight <= 1, "edge weight must be 0 or 1");
            let nd = d + weight;
            if nd < dist[to] {
                dist[to] = nd;
                if weight == 0 {
                    deque.push_front((to, nd));
                } else {
                    deque.push_back((to, nd));
                }
            }
        }
    }
    dist.into_iter()
        .map(|d| if d == usize::MAX { None } else { Some(d) })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::zero_one_bfs;

    #[test]
    fn uses_free_edges_and_marks_unreachable() {
        let g = vec![vec![(1, 1), (2, 0)], vec![(3, 1)], vec![(1, 0)], vec![], vec![]];
        assert_eq!(
            zero_one_bfs(&g, 0),
            vec![Some(0), Some(0), Some(0), Some(1), None]
        );
    }

    #[test]
    #[should_panic(expected = "edge weight must be 0 or 1")]
    fn rejects_other_weights() {
        zero_one_bfs(&[vec![(1, 2)], vec![]], 0);
    }
}
