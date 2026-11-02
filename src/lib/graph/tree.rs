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

/// 木の直径と中心をまとめて求めた結果です。
///
/// `path` は直径の両端を含む頂点列で、`length` はその辺数です。
/// `centers` は直径パスの中点にあたる1頂点または2頂点です。
/// 空の木では `length == 0` で、`path` と `centers` は空配列です。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiameterInfo {
    /// 直径の長さ（辺数）です。
    pub length: usize,
    /// 直径の両端を含む頂点列です。
    pub path: Vec<usize>,
    /// 木の中心（1頂点または2頂点）です。
    pub centers: Vec<usize>,
}

/// 木の直径の長さ・パス・中心をまとめて返します。
///
/// 直径を一度求め、中心をそのパスから取り出します。直径が複数あれば、
/// そのうち1本のパスを返します。計算量・メモリ量は `O(N)` です。
///
/// # Examples
///
/// ```
/// use atcoder::graph::tree::diameter_info;
///
/// let graph = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
/// let info = diameter_info(&graph);
/// assert_eq!(info.length, 3);
/// assert_eq!(info.path.len(), 4);
/// let mut centers = info.centers;
/// centers.sort_unstable();
/// assert_eq!(centers, vec![1, 2]);
/// ```
pub fn diameter_info(graph: &[Vec<usize>]) -> DiameterInfo {
    let (length, path) = diameter(graph);
    let centers = if path.is_empty() {
        Vec::new()
    } else {
        path[length / 2..=length.div_ceil(2)].to_vec()
    };
    DiameterInfo {
        length,
        path,
        centers,
    }
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
    diameter_info(graph).centers
}

/// 木の重心を昇順で返します。
///
/// 重心は、頂点を取り除いてできる各連結成分の頂点数が `N / 2` 以下になる頂点です。
/// 1個または2個存在し、直径の中点である木の中心とは異なります。
/// 空の木には空配列を返します。計算量・メモリ量は `O(N)` です。
///
/// # Examples
///
/// ```
/// use atcoder::graph::tree::centroids;
///
/// let graph = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
/// assert_eq!(centroids(&graph), vec![1, 2]);
/// ```
pub fn centroids(graph: &[Vec<usize>]) -> Vec<usize> {
    let n = graph.len();
    if n == 0 {
        return Vec::new();
    }
    let (parent, _, order) = rooted_order(graph, 0);
    let mut size = vec![1usize; n];
    for &v in order.iter().rev() {
        if v != 0 {
            size[parent[v]] += size[v];
        }
    }

    (0..n)
        .filter(|&v| {
            // 親方向の連結成分と、子の各部分木をすべて確認します。
            let largest = graph[v]
                .iter()
                .filter(|&&to| to != parent[v])
                .map(|&to| size[to])
                .chain(std::iter::once(n - size[v]))
                .max()
                .unwrap_or(0);
            largest <= n / 2
        })
        .collect()
}

#[cfg(test)]
mod centroid_tests {
    use super::{centers, centroids};

    #[test]
    fn empty_path_and_star() {
        assert!(centroids(&[]).is_empty());
        assert_eq!(centroids(&[vec![]]), vec![0]);
        let chain = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        assert_eq!(centroids(&chain), vec![1, 2]);
        let star = vec![vec![1, 2, 3, 4], vec![0], vec![0], vec![0], vec![0]];
        assert_eq!(centroids(&star), vec![0]);
    }

    #[test]
    fn centroid_is_not_always_center() {
        let mut graph = vec![vec![]; 11];
        for v in 1..=5 {
            graph[v - 1].push(v);
            graph[v].push(v - 1);
        }
        for v in 6..=10 {
            graph[0].push(v);
            graph[v].push(0);
        }
        assert_eq!(centroids(&graph), vec![0]);
        assert_eq!(centers(&graph), vec![2]);
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
    fn diameter_info_handles_empty_even_and_odd_paths() {
        let empty = diameter_info(&[]);
        assert_eq!(empty.length, 0);
        assert!(empty.path.is_empty());
        assert!(empty.centers.is_empty());

        let singleton = diameter_info(&[vec![]]);
        assert_eq!(singleton.length, 0);
        assert_eq!(singleton.path, vec![0]);
        assert_eq!(singleton.centers, vec![0]);

        for n in [4, 5] {
            let mut graph = vec![vec![]; n];
            for v in 1..n {
                graph[v - 1].push(v);
                graph[v].push(v - 1);
            }
            let info = diameter_info(&graph);
            assert_eq!(info.length, n - 1);
            assert_eq!((info.length, info.path.clone()), diameter(&graph));
            let mut middle = info.centers;
            middle.sort_unstable();
            let mut expected = centers(&graph);
            expected.sort_unstable();
            assert_eq!(middle, expected);
        }
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
