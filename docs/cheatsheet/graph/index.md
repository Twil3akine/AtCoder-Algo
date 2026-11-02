# グラフの問題から解法を探す

まず、**有向か無向か／辺の重みの条件／求めたい値**を整理します。

| 問題文に現れる特徴 | 最初に考えること | 解法 |
| --- | --- | --- |
| 全ての辺の移動コストが等しい | BFS | [重みなし最短路](crate::cheatsheet::graph::bfs) |
| 辺のコストが0または1 | 両端キュー | [0-1 BFS](crate::cheatsheet::graph::zero_one_bfs) |
| 重みが非負の最短距離 | 優先度付きキュー | [Dijkstra](crate::cheatsheet::graph::dijkstra) |
| 有向辺をたどって互いに到達できる | 強連結成分へ縮約 | [SCC](crate::cheatsheet::graph::scc) |
| 除去すると分断される辺や頂点 | LowLink | [橋・関節点](crate::cheatsheet::graph::low_link) |
| 辺に容量があり最大何個流せるか | 残余グラフ | [最大流](crate::cheatsheet::graph::max_flow) |
| 前提作業の順番を決める | DAGならトポロジカル順 | [依存関係と順序](crate::cheatsheet::graph::topological_sort) |

状態が「頂点番号だけ」では表せない問題（鍵の所持、時間、特殊な移動回数など）は、状態を拡張した上で探索を組み立ててください。
