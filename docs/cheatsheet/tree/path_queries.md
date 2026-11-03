# 木のパス上の合計・最大値を更新しながら求めるなら、HLD

## こういうときに使う

- 任意の2頂点間のパスにある値を集約したいとき
- 途中で頂点や辺の値を変更するクエリがあるとき

## 使うアルゴリズム

**HLD（Heavy-Light Decomposition）とセグメント木**を用います。

## 解き方

HLDで木のパスを O(log N) 個の半開区間に分け、各区間をセグメント木で集約します。辺の値は子側頂点に持たせる方法があります。

## 注意・計算量

集約演算には結合則が必要です。非可換な演算なら `PathSegment::reversed` に従って順序を守ります。距離だけならLCAの方が簡単です。 計算量は **HLD構築 O(N)、パスクエリ O(log² N)（セグメント木利用）** です。

**対応するAPI**：[`Hld`](atcoder::graph::hld::Hld)、[`SegmentTree`](atcoder::data_structure::segment_tree::SegmentTree)

**練習問題**：[AtCoder ABC294 G — Distance Queries on a Tree](https://atcoder.jp/contests/abc294/tasks/abc294_g)
