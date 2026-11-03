# 移動コストが0または1だけなら、0-1 BFS

## こういうときに使う

- 無料の移動とコスト1の移動が混在するとき
- ワープなど特別な移動の回数を最小化したいとき

## 使うアルゴリズム

**0-1 BFS**を用います。

## 解き方

重み0の辺による移動を両端キューの先頭へ、重み1の辺による移動を末尾へ追加します。

## 注意・計算量

辺のコストは0または1に限定されます。コスト2以上を含むならDijkstraを使います。 計算量は **O(V + E)** です。

**対応するAPI**：[`zero_one_bfs()`](atcoder::graph::zero_one_bfs::zero_one_bfs)

**練習問題**：[AtCoder ABC176 D — Wizard in Maze](https://atcoder.jp/contests/abc176/tasks/abc176_d)
