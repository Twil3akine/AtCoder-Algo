# 累積和が初めてK以上になる位置などを高速に探したいなら、セグメント木上の二分探索

## こういうときに使う

累積和が初めてK以上になる位置などを高速に探したい場合に、**セグメント木上の二分探索** を用います。

## 考え方と使い方

区間集約の結果について単調に変わる条件なら、集約しながら境界を探せます。 左から伸ばす場合は `max_right`、右から伸ばす場合は `min_left` を使います。例えば要素が非負なら、区間和と閾値の比較が単調になります。

## 注意・計算量

判定関数は単位元に対して真で、区間を広げたときに真から偽へ一度だけ変わる必要があります。

**計算量**：各問い合わせ O(log N)。

**対応API**：[`SegmentTree::max_right`](atcoder::data_structure::segment_tree::SegmentTree::max_right)、[`SegmentTree::min_left`](atcoder::data_structure::segment_tree::SegmentTree::min_left)。
