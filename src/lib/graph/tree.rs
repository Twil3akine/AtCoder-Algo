//! 重みなし無向木の直径・パス・中心を求めます。
//!
//! グラフは隣接リストで与え、頂点番号は0始まりです。入力が木であることを
//! 前提とします。走査はすべて反復実装です。

/// 木の直径を `(長さ, 端点から端点への頂点列)` で返します。
///
/// 長さは辺の本数です。直径が複数あれば、そのうちの1本を返します。
/// 空の木には `(0, vec![])`、1頂点の木には `(0, vec![0])` を返します。
/// 計算量・メモリ量は `O(N)` です。
///
/// # Examples
///
/// ```
/// use atcoder::graph::tree::{centers, diameter, path};
///
/// let graph = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
/// let (length, vertices) = diameter(&graph);
/// assert_eq!(length, 3);
/// assert_eq!(vertices.len(), 4);
/// assert_eq!(path(&graph, 0, 3), vec![0, 1, 2, 3]);
/// let mut middle = centers(&graph);
/// middle.sort_unstable();
/// assert_eq!(middle, vec![1, 2]);
/// ```
pub fn diameter(graph: &[Vec<usize>]) -> (usize, Vec<usize>) {
    if graph.is_empty() {
        return (0, Vec::new());
    }
    let (_, depth, _) = rooted_order(graph, 0);
    let start = (0..graph.len()).max_by_key(|&v| depth[v]).unwrap();
    let (parent, depth, _) = rooted_order(graph, start);
    let goal = (0..graph.len()).max_by_key(|&v| depth[v]).unwrap();
    let vertices = restore_path(&parent, start, goal);
    (depth[goal], vertices)
}

/// `start` から `goal` への単純パスを、両端を含む頂点列で返します。
///
/// 同じ頂点を指定した場合は `[start]` です。計算量・メモリ量は `O(N)` です。
/// 頂点番号が範囲外なら panic します。
pub fn path(graph: &[Vec<usize>], start: usize, goal: usize) -> Vec<usize> {
    assert!(goal < graph.len(), "vertex out of bounds");
    let (parent, _, _) = rooted_order(graph, start);
    restore_path(&parent, start, goal)
}

/// 各頂点への距離の最大値が最小になる頂点（木の中心）を返します。
///
/// 直径の中央にある1頂点または2頂点です。2頂点の場合の順序は未規定です。
/// 空の木には空配列を返します。計算量・メモリ量は `O(N)` です。
pub fn centers(graph: &[Vec<usize>]) -> Vec<usize> {
    let (length, vertices) = diameter(graph);
    if vertices.is_empty() {
        Vec::new()
    } else {
        vertices[length / 2..=length.div_ceil(2)].to_vec()
    }
}

// 根の親は根自身とし、親が子より先に現れる順序を作ります。
pub(crate) fn rooted_order(
    graph: &[Vec<usize>],
    root: usize,
) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let n = graph.len();
    assert!(root < n, "vertex out of bounds");
    let mut parent = vec![usize::MAX; n];
    let mut depth = vec![0; n];
    let mut order = Vec::with_capacity(n);
    let mut stack = vec![root];
    parent[root] = root;
    while let Some(v) = stack.pop() {
        order.push(v);
        for &to in &graph[v] {
            if parent[to] == usize::MAX {
                parent[to] = v;
                depth[to] = depth[v] + 1;
                stack.push(to);
            }
        }
    }
    assert_eq!(order.len(), n, "tree must be connected");
    (parent, depth, order)
}

fn restore_path(parent: &[usize], start: usize, mut goal: usize) -> Vec<usize> {
    let mut vertices = vec![goal];
    while goal != start {
        goal = parent[goal];
        vertices.push(goal);
    }
    vertices.reverse();
    vertices
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::traversal::bfs;

    #[test]
    fn empty_singleton_and_two_vertices() {
        assert_eq!(diameter(&[]), (0, vec![]));
        assert!(centers(&[]).is_empty());
        assert_eq!(diameter(&[vec![]]), (0, vec![0]));
        assert_eq!(centers(&[vec![]]), vec![0]);
        assert_eq!(path(&[vec![]], 0, 0), vec![0]);
        let graph = vec![vec![1], vec![0]];
        let mut middle = centers(&graph);
        middle.sort_unstable();
        assert_eq!(middle, vec![0, 1]);
    }

    #[test]
    fn diameter_path_and_centers_match_all_pairs_distances() {
        let graphs = [
            vec![vec![1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3]],
            vec![vec![1, 2, 3, 4], vec![0], vec![0], vec![0], vec![0]],
            vec![
                vec![1],
                vec![0, 2, 3],
                vec![1],
                vec![1, 4, 5],
                vec![3],
                vec![3],
            ],
        ];
        for graph in graphs {
            let distances: Vec<_> = (0..graph.len()).map(|v| bfs(&graph, v)).collect();
            let eccentricities: Vec<_> = distances
                .iter()
                .map(|row| row.iter().map(|d| d.unwrap()).max().unwrap())
                .collect();
            let (length, vertices) = diameter(&graph);
            assert_eq!(length, *eccentricities.iter().max().unwrap());
            assert_eq!(vertices.len(), length + 1);
            assert!(vertices.windows(2).all(|e| graph[e[0]].contains(&e[1])));
            let radius = *eccentricities.iter().min().unwrap();
            let mut middle = centers(&graph);
            middle.sort_unstable();
            assert_eq!(
                middle,
                (0..graph.len())
                    .filter(|&v| eccentricities[v] == radius)
                    .collect::<Vec<_>>()
            );
            for (u, row) in distances.iter().enumerate() {
                for (v, &distance) in row.iter().enumerate() {
                    let vertices = path(&graph, u, v);
                    assert_eq!(vertices.first(), Some(&u));
                    assert_eq!(vertices.last(), Some(&v));
                    assert_eq!(vertices.len(), distance.unwrap() + 1);
                    assert!(vertices.windows(2).all(|e| graph[e[0]].contains(&e[1])));
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
        assert_eq!(diameter(&graph).0, n - 1);
        assert_eq!(path(&graph, 0, n - 1), (0..n).collect::<Vec<_>>());
        let mut middle = centers(&graph);
        middle.sort_unstable();
        assert_eq!(middle, vec![n / 2 - 1, n / 2]);
    }
}
