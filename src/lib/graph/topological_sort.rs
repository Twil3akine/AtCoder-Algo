//! 有向非循環グラフ（DAG）のトポロジカルソートです。

use std::collections::VecDeque;

/// トポロジカル順序を返します。グラフに閉路がある場合は `None` です。
///
/// グラフは隣接リスト `graph[v] = v から出る辺の行き先` で与えます。
/// Kahn 法（入次数の小さい頂点から取り出す方法）で計算し、計算量は
/// `O(V + E)` です。
///
/// # Examples
///
/// ```
/// use atcoder::graph::topological_sort::topological_sort;
///
/// let graph = vec![vec![1], vec![2], vec![]];
/// assert_eq!(topological_sort(&graph), Some(vec![0, 1, 2]));
/// ```
pub fn topological_sort(graph: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = graph.len();
    let mut indegree = vec![0usize; n];
    for v in 0..n {
        for &to in &graph[v] {
            indegree[to] += 1;
        }
    }
    let mut queue: VecDeque<usize> = (0..n).filter(|&v| indegree[v] == 0).collect();
    let mut order = Vec::with_capacity(n);
    while let Some(v) = queue.pop_front() {
        order.push(v);
        for &to in &graph[v] {
            indegree[to] -= 1;
            if indegree[to] == 0 {
                queue.push_back(to);
            }
        }
    }
    if order.len() == n {
        Some(order)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_a_dag() {
        let graph = vec![vec![2], vec![2], vec![]];
        let order = topological_sort(&graph).unwrap();
        assert_eq!(order.len(), 3);
        assert_eq!(order[2], 2);
    }

    #[test]
    fn rejects_a_cycle() {
        let graph = vec![vec![1], vec![0]];
        assert_eq!(topological_sort(&graph), None);
    }
}
