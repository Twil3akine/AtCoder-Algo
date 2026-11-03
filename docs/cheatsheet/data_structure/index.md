# データ構造・区間クエリ — こういうときに何を使う？

問題文の条件から選べる、実装済みライブラリの逆引きです。

| こういうとき | 用いるアルゴリズム |
| --- | --- |
| 辺を追加しながら、2頂点がつながっているか判定したい | [Union-Find](crate::data_structure::union_find) |
| 探索で仮に辺を追加し、前の連結状態へ戻したい | [Rollback Union-Find](crate::data_structure::rollback_union_find) |
| 配列の1点を書き換えながら、区間の合計・最小値を求めたい | [セグメント木](crate::data_structure::segment_tree) |
| 累積和が初めてK以上になる位置などを高速に探したい | [セグメント木上の二分探索](crate::data_structure::segment_search) |
| 区間に一括加算・代入しながら、区間和や区間最小値も求めたい | [遅延評価セグメント木](crate::data_structure::lazy_segment_tree) |
| 区間に一括操作した後、指定した1点の値だけ知りたい | [双対セグメント木](crate::data_structure::dual_segment_tree) |
| 要素に加算しながら、先頭からの合計や区間和を求めたい | [Fenwick Tree（BIT）](crate::data_structure::fenwick_tree) |
| 数列の大小が逆転している添字対を数えたい | [座標圧縮＋Fenwick Tree](crate::data_structure::inversion_count) |
| 個数の累積和から、k番目の要素がある位置を探したい | [Fenwick Treeの累積頻度二分探索](crate::data_structure::prefix_kth) |
| 配列が更新されず、区間の最小値・最大値を大量に求めたい | [Sparse Table](crate::data_structure::sparse_table) |
| キューの先頭を削除し、末尾へ追加しながら全体を集約したい | [SWAG](crate::data_structure::swag) |
| 毎回、現在の候補の中から最小値・最大値を取り出したい | [優先度付きキュー（Heap）](crate::data_structure::priority_queue) |

[チートシートのトップ](crate)
