//! 重み付きグラフの最短路（ダイクストラ法、ワーシャル・フロイド法、ベルマン・フォード法）です。

use crate::data_structure::heap::MinHeap;

/// 到達不能を表す十分大きな値です。最短路の加算で溢れないよう `i64::MAX / 4` にしています。
pub const INF: i64 = i64::MAX / 4;

/// ダイクストラ法で、頂点 `start` から各頂点への最短距離を返します。
///
/// グラフは隣接リスト `graph[v] = (行き先, 重み) の列` で与えます。重みは非負で
/// ある必要があります。到達できない頂点の距離は `INF` です。計算量は
/// `O(E log V)` です。
///
/// # Examples
///
/// ```
/// use atcoder::graph::shortest_path::{dijkstra, INF};
///
/// let graph = vec![
///     vec![(1, 2), (2, 5)],
///     vec![(2, 1)],
///     vec![],
/// ];
/// let dist = dijkstra(&graph, 0);
/// assert_eq!(dist, vec![0, 2, 3]);
/// ```
pub fn dijkstra(graph: &[Vec<(usize, i64)>], start: usize) -> Vec<i64> {
    let mut dist = vec![INF; graph.len()];
    let mut heap = MinHeap::new();
    dist[start] = 0;
    heap.push((0, start));
    while let Some((d, v)) = heap.pop() {
        if d > dist[v] {
            continue;
        }
        for &(to, w) in &graph[v] {
            if dist[to] > d + w {
                dist[to] = d + w;
                heap.push((dist[to], to));
            }
        }
    }
    dist
}

/// ワーシャル・フロイド法で、全頂点対の最短距離を求めます。
///
/// `dist[i][j]` を辺 i→j の重み（辺が無ければ `INF`、対角は 0）で初期化して
/// 渡すと、その場で最短距離に書き換えます。負の重みがあっても、負閉路が
/// 無ければ正しく動きます。計算量は `O(V^3)` です。
pub fn floyd_warshall(dist: &mut [Vec<i64>]) {
    let n = dist.len();
    for k in 0..n {
        for i in 0..n {
            if dist[i][k] == INF {
                continue;
            }
            for j in 0..n {
                if dist[k][j] == INF {
                    continue;
                }
                let via = dist[i][k] + dist[k][j];
                if via < dist[i][j] {
                    dist[i][j] = via;
                }
            }
        }
    }
}

/// ベルマン・フォード法で、頂点 `start` から各頂点への最短距離を返します。
///
/// 辺は `(始点, 終点, 重み)` の列で与えます。負の重みがあっても動きます。
/// `start` から到達可能な負閉路がある場合は `None` を返します。到達できない
/// 頂点の距離は `INF` です。計算量は `O(V E)` です。
pub fn bellman_ford(n: usize, edges: &[(usize, usize, i64)], start: usize) -> Option<Vec<i64>> {
    let mut dist = vec![INF; n];
    dist[start] = 0;
    for i in 0..n {
        let mut updated = false;
        for &(from, to, w) in edges {
            if dist[from] != INF && dist[to] > dist[from] + w {
                dist[to] = dist[from] + w;
                updated = true;
                if i == n - 1 {
                    return None;
                }
            }
        }
        if !updated {
            break;
        }
    }
    Some(dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijkstra_finds_shortest() {
        let graph = vec![
            vec![(1, 4), (2, 1)],
            vec![(3, 1)],
            vec![(1, 2), (3, 5)],
            vec![],
        ];
        assert_eq!(dijkstra(&graph, 0), vec![0, 3, 1, 4]);
    }

    #[test]
    fn floyd_warshall_fills_all_pairs() {
        let mut dist = vec![vec![0, 3, INF], vec![INF, 0, 2], vec![INF, INF, 0]];
        floyd_warshall(&mut dist);
        assert_eq!(dist[0][2], 5);
        assert_eq!(dist[2][0], INF);
    }

    #[test]
    fn bellman_ford_handles_negative_weights() {
        let edges = vec![(0, 1, 2), (1, 2, -5), (0, 2, 1)];
        assert_eq!(bellman_ford(3, &edges, 0), Some(vec![0, 2, -3]));
    }

    #[test]
    fn bellman_ford_detects_negative_cycle() {
        let edges = vec![(0, 1, 1), (1, 0, -3)];
        assert_eq!(bellman_ford(2, &edges, 0), None);
    }
}
