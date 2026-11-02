# 非負重みの最短路・移動費用の最小化

**問題の手掛かり**：移動ごとに異なる費用があり、始点から各頂点への最小費用を求める。辺の重みが非負である。

**着眼点**：現在最も距離が短い未確定頂点を取り出して確定します。優先度付きキューが利用できます。

**解法**：Dijkstra法を使います。始点を固定して全頂点までの最短距離を取得します。

**適用条件・注意**：負辺がある場合はこの方法を選べません。Bellman–Fordなどを検討してください。到達不能の表現も確認します。

**計算量**：典型的に O((V + E) log V)。

**使うAPI**：[`dijkstra()`](crate::graph::shortest_path::dijkstra)、[`bellman_ford()`](crate::graph::shortest_path::bellman_ford)。

**練習問題**：[AtCoder ABC340 D — Super Takahashi Bros.](https://atcoder.jp/contests/abc340/tasks/abc340_d)。
