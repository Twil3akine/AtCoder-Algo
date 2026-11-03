# 有向グラフで互いに行き来できる頂点をまとめるなら、SCC

## こういうときに使う

- 有向辺をたどって、2頂点が相互に到達できるか調べたいとき
- 閉路を含む有向グラフをDAGに圧縮したいとき

## 使うアルゴリズム

**SCC（強連結成分分解）**を用います。

## 解き方

相互に到達可能な頂点を同じ成分にまとめます。成分を1頂点に縮約するとDAGになります。

## 注意・計算量

有向グラフ向けです。無向グラフの通常の連結成分分解とは異なります。 計算量は **O(V + E)** です。

**対応するAPI**：[`scc()`](atcoder::graph::scc::scc)、[`SccResult`](atcoder::graph::scc::SccResult)

**練習問題**：[CSES — Planets and Kingdoms](https://cses.fi/problemset/task/1683)
