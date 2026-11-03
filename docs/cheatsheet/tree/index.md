# 木 — こういうときに何を使う？

対象は主に重みなしの木です。問題文の条件に近い行から解法を探してください。

| こういうとき | 用いるアルゴリズム |
| --- | --- |
| 木で最も遠い2頂点、最長パス | [木の直径](crate::tree::diameter) |
| 距離が直径になる同じ色の2頂点を数える | [木の中心から分岐別に数え上げる](crate::tree::diameter_pairs) |
| 最大距離を小さくする／木を均等に分割する | [中心／重心](crate::tree::center_centroid) |
| 2頂点間の距離、k番目の頂点を何度も求める | [LCA・ダブリング](crate::tree::lca_jump) |
| パス上の合計・最大値を更新しながら求める | [HLD・セグメント木](crate::tree::path_queries) |
| すべての頂点を根にしたDPが必要 | [全方位木DP](crate::tree::rerooting) |

[グラフの逆引き](crate::graph) / [チートシートのトップ](crate)
