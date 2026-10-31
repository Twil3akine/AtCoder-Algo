//! 無向・有向グラフの走査（幅優先探索と深さ優先探索）です。

use std::collections::VecDeque;

/// 頂点 `start` から幅優先探索を行い、各頂点への最短距離（辺の本数）を返します。
///
/// グラフは隣接リスト `graph[v] = v から出る辺の行き先` で与えます。到達できない
/// 頂点の距離は `None` です。計算量は `O(V + E)` です。
///
/// # Examples
///
/// ```
/// use atcoder::graph::traversal::bfs;
///
/// let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
/// let dist = bfs(&graph, 0);
/// assert_eq!(dist, vec![Some(0), Some(1), Some(1), Some(2)]);
/// ```
pub fn bfs(graph: &[Vec<usize>], start: usize) -> Vec<Option<usize>> {
    let mut dist = vec![None; graph.len()];
    let mut queue = VecDeque::new();
    dist[start] = Some(0);
    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        for &to in &graph[v] {
            if dist[to].is_none() {
                dist[to] = Some(dist[v].unwrap() + 1);
                queue.push_back(to);
            }
        }
    }
    dist
}

/// 頂点 `start` から深さ優先探索を行い、頂点を訪れた順序を返します。
///
/// グラフは隣接リストで与えます。反復実装のため、深いグラフでもスタック
/// オーバーフローしません。計算量は `O(V + E)` です。
pub fn dfs_order(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];
    let mut order = Vec::new();
    let mut stack = vec![start];
    visited[start] = true;
    while let Some(v) = stack.pop() {
        order.push(v);
        for &to in &graph[v] {
            if !visited[to] {
                visited[to] = true;
                stack.push(to);
            }
        }
    }
    order
}

/// 無向グラフの連結成分を返します。各成分は頂点の列です。
///
/// グラフは隣接リストで与えます。計算量は `O(V + E)` です。
pub fn connected_components(graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut visited = vec![false; graph.len()];
    let mut components = Vec::new();
    for start in 0..graph.len() {
        if visited[start] {
            continue;
        }
        let mut component = Vec::new();
        let mut stack = vec![start];
        visited[start] = true;
        while let Some(v) = stack.pop() {
            component.push(v);
            for &to in &graph[v] {
                if !visited[to] {
                    visited[to] = true;
                    stack.push(to);
                }
            }
        }
        components.push(component);
    }
    components
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_gives_shortest_distances() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![], vec![]];
        let dist = bfs(&graph, 0);
        assert_eq!(dist, vec![Some(0), Some(1), Some(1), Some(2), None]);
    }

    #[test]
    fn dfs_visits_all_reachable() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let mut order = dfs_order(&graph, 0);
        order.sort_unstable();
        assert_eq!(order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn components_are_split() {
        let graph = vec![vec![1], vec![0, 2], vec![1], vec![]];
        let mut components = connected_components(&graph);
        for component in &mut components {
            component.sort_unstable();
        }
        components.sort();
        assert_eq!(components, vec![vec![0, 1, 2], vec![3]]);
    }
}
