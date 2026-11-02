//! 最大流を求めるDinic法です。
//!
//! BFSでレベルグラフを構築し、反復DFSで増加路を探索します。
//! 残余グラフを維持するため、\`max_flow\` は同じインスタンスに繰り返し呼び出せます。

use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cap: i64,
}

/// 容量付き有向グラフの最大流を求めます。
///
/// 頂点は0始まり、容量は非負の \`i64\` です。総流量が \`i64\` に収まることを
/// 呼び出し側で保証してください。一般のグラフに対する計算量は \`O(V^2 E)\` です。
///
/// # Examples
///
/// \`\`\`
/// use atcoder::graph::max_flow::Dinic;
///
/// let mut flow = Dinic::new(4);
/// flow.add_edge(0, 1, 2);
/// flow.add_edge(0, 2, 1);
/// flow.add_edge(1, 3, 1);
/// flow.add_edge(1, 2, 1);
/// flow.add_edge(2, 3, 2);
/// assert_eq!(flow.max_flow(0, 3), 3);
/// \`\`\`
#[derive(Debug, Clone)]
pub struct Dinic {
    graph: Vec<Vec<usize>>,
    edges: Vec<Edge>,
}

impl Dinic {
    /// \`n\` 頂点のネットワークを作ります。計算量は \`O(N)\` です。
    pub fn new(n: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n],
            edges: Vec::new(),
        }
    }

    /// 有向辺 \`from -> to\` に容量 \`capacity\` を設定します。
    ///
    /// 戻り値は追加した辺の番号（0始まり）です。平行辺と自己ループも扱えます。
    /// 計算量は償却 \`O(1)\` です。
    pub fn add_edge(&mut self, from: usize, to: usize, capacity: i64) -> usize {
        assert!(capacity >= 0, "capacity must be non-negative");
        assert!(from < self.graph.len() && to < self.graph.len());
        let id = self.edges.len();
        self.graph[from].push(id);
        self.graph[to].push(id + 1);
        self.edges.push(Edge { to, cap: capacity });
        self.edges.push(Edge { to: from, cap: 0 });
        id / 2
    }

    /// 現在の残余グラフで流せる最大流量を返し、流した分だけ残余容量を更新します。
    ///
    /// \`source == sink\` なら0を返します。計算量は \`O(V^2 E)\` です。
    /// 再帰を使用しないため長いパスでもスタックオーバーフローしません。
    pub fn max_flow(&mut self, source: usize, sink: usize) -> i64 {
        let n = self.graph.len();
        assert!(source < n && sink < n);
        if source == sink {
            return 0;
        }
        let mut answer = 0i64;
        loop {
            let level = self.levels(source);
            if level[sink] == usize::MAX {
                return answer;
            }
            let mut next = vec![0usize; n];
            loop {
                let flow = self.augment(source, sink, &level, &mut next);
                if flow == 0 {
                    break;
                }
                answer = answer.checked_add(flow).expect("maximum flow overflow");
            }
        }
    }

    /// 最大流を流した後、残余容量が正の辺で \`source\` から到達可能な頂点を返します。
    ///
    /// 最小カットの始点側を得る場合に使用します。計算量は \`O(V + E)\` です。
    pub fn min_cut(&self, source: usize) -> Vec<bool> {
        assert!(source < self.graph.len());
        let mut seen = vec![false; self.graph.len()];
        let mut queue = VecDeque::new();
        seen[source] = true;
        queue.push_back(source);
        while let Some(v) = queue.pop_front() {
            for &id in &self.graph[v] {
                let e = &self.edges[id];
                if e.cap > 0 && !seen[e.to] {
                    seen[e.to] = true;
                    queue.push_back(e.to);
                }
            }
        }
        seen
    }

    /// 追加した \`edge_index\` 番目の辺に現在流れている流量を返します。
    ///
    /// 番号は \`add_edge\` の戻り値で、計算量は \`O(1)\` です。
    pub fn edge_flow(&self, edge_index: usize) -> i64 {
        self.edges[2 * edge_index + 1].cap
    }

    fn levels(&self, source: usize) -> Vec<usize> {
        let mut level = vec![usize::MAX; self.graph.len()];
        let mut queue = VecDeque::new();
        level[source] = 0;
        queue.push_back(source);
        while let Some(v) = queue.pop_front() {
            for &id in &self.graph[v] {
                let e = &self.edges[id];
                if e.cap > 0 && level[e.to] == usize::MAX {
                    level[e.to] = level[v] + 1;
                    queue.push_back(e.to);
                }
            }
        }
        level
    }

    // レベルグラフの増加路を1本見つけて、その経路にまとめて流します。
    fn augment(&mut self, source: usize, sink: usize, level: &[usize], next: &mut [usize]) -> i64 {
        let mut path = Vec::new();
        let mut v = source;
        loop {
            if v == sink {
                let pushed = path
                    .iter()
                    .map(|&id: &usize| self.edges[id].cap)
                    .min()
                    .unwrap();
                for id in path {
                    self.edges[id].cap -= pushed;
                    self.edges[id ^ 1].cap += pushed;
                }
                return pushed;
            }
            while next[v] < self.graph[v].len() {
                let id = self.graph[v][next[v]];
                let edge = &self.edges[id];
                if edge.cap > 0 && level[edge.to] == level[v] + 1 {
                    break;
                }
                next[v] += 1;
            }
            if next[v] == self.graph[v].len() {
                if let Some(id) = path.pop() {
                    v = self.edges[id ^ 1].to;
                    next[v] += 1;
                } else {
                    return 0;
                }
            } else {
                let id = self.graph[v][next[v]];
                path.push(id);
                v = self.edges[id].to;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Dinic;

    #[test]
    fn max_flow_cut_and_edge_flows() {
        let mut d = Dinic::new(4);
        let first = d.add_edge(0, 1, 2);
        d.add_edge(0, 2, 1);
        d.add_edge(1, 3, 1);
        d.add_edge(1, 2, 1);
        d.add_edge(2, 3, 2);
        assert_eq!(d.max_flow(0, 3), 3);
        assert_eq!(d.edge_flow(first), 2);
        assert_eq!(d.max_flow(0, 3), 0);
        assert_eq!(d.min_cut(0), vec![true, false, false, false]);
    }

    #[test]
    fn deep_path_and_parallel_edges() {
        let n = 200_000;
        let mut d = Dinic::new(n);
        for v in 1..n {
            d.add_edge(v - 1, v, 7);
        }
        d.add_edge(0, 1, 2);
        assert_eq!(d.max_flow(0, n - 1), 7);
    }

    #[test]
    fn self_loop_has_no_effect() {
        let mut d = Dinic::new(2);
        d.add_edge(0, 0, 100);
        d.add_edge(0, 1, 3);
        assert_eq!(d.max_flow(0, 1), 3);
    }
}
