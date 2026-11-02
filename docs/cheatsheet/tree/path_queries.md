# 木のパス上の合計・最大値と更新

**問題の手掛かり**：`u`から`v`へのパスにある頂点・辺の値を集約し、さらに途中で値が変わる。

**着眼点**：木のパスを、線形配列上の少数の半開区間へ分解します。区間はセグメント木で管理します。

**解法**：HLDで頂点番号を並べ替え、`path(u, v)`（頂点値）または`edge_path(u, v)`（辺値）で得た区間を集約します。

**適用条件・注意**：一般的な集約演算には結合則が必要です。非可換な演算では`PathSegment::reversed`による順序を守ります。単に距離だけ求めるならLCAで十分です。

**計算量**：HLD構築 O(N)、セグメント木でのパスクエリ O(log² N)。

**使うAPI**：[`Hld`](crate::graph::hld::Hld)、[`SegmentTree`](crate::data_structure::segment_tree::SegmentTree)。

**練習問題**：[AtCoder ABC294 G — Distance Queries on a Tree](https://atcoder.jp/contests/abc294/tasks/abc294_g)。
