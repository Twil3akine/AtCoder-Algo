# グラフ — こういうときに何を使う？

まず**有向・無向の違い、辺の重み、求めたい値**を確認してください。

| こういうとき | 用いるアルゴリズム |
| --- | --- |
| 全ての辺の移動コストが等しい | [BFS](crate::graph::bfs) |
| 辺のコストが0または1 | [0-1 BFS](crate::graph::zero_one_bfs) |
| 辺の重みが非負で最短費用を求める | [Dijkstra](crate::graph::dijkstra) |
| 有向グラフで相互に到達可能な頂点をまとめる | [SCC](crate::graph::scc) |
| 辺や頂点を1つ消すと分断される | [LowLink](crate::graph::low_link) |
| 容量を守って最大量を流す・最大割当を求める | [最大流（Dinic）](crate::graph::max_flow) |
| 依存関係を満たす実行順序を決める | [トポロジカルソート](crate::graph::topological_sort) |

[木の逆引き](crate::tree) / [チートシートのトップ](crate)
