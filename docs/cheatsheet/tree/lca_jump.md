# 木の2頂点間の距離やk番目の頂点を何度も求めるなら、LCA

## こういうときに使う

- 2頂点の最小共通祖先や距離を何度も求めたいとき
- 頂点の `k` 個上の祖先や、`u` から `v` へ `k` 辺進んだ頂点が必要なとき

## 使うアルゴリズム

**LCA（最小共通祖先）とダブリング**を用います。

## 解き方

根からの深さと2の累乗個上の祖先を前計算します。距離は `depth[u] + depth[v] - 2 * depth[lca(u, v)]` で求め、経路上の移動には `jump(u, v, k)` を使います。

## 注意・計算量

木の構造が変わらない場合に適しています。`k` がパス長を超えると `None` です。 計算量は **前計算 O(N log N)、各問い合わせ O(log N)** です。

**対応するAPI**：[`Lca`](atcoder::graph::lca::Lca)、[`Lca::distance()`](atcoder::graph::lca::Lca::distance)、[`Lca::jump()`](atcoder::graph::lca::Lca::jump)
