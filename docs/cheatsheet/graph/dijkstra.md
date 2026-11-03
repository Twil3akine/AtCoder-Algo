# 非負の重みを持つグラフの最短費用なら、Dijkstra

## こういうときに使う

- 辺ごとの移動時間や費用が異なるとき
- すべての辺の重みが非負で、最短距離を求めたいとき

## 使うアルゴリズム

**Dijkstra法**を用います。

## 解き方

優先度付きキューから距離が最も小さい頂点を取り出し、隣接頂点の距離を更新します。

## 注意・計算量

負の重みがあるとDijkstra法の前提が崩れます。負辺を含む場合はBellman–Fordなどを検討します。 計算量は **通常 O((V + E) log V)** です。

**対応するAPI**：[`dijkstra()`](atcoder::graph::shortest_path::dijkstra)、[`bellman_ford()`](atcoder::graph::shortest_path::bellman_ford)

**練習問題**：[AtCoder ABC340 D — Super Takahashi Bros.](https://atcoder.jp/contests/abc340/tasks/abc340_d)
