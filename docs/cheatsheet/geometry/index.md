# 幾何 — こういうときに何を使う？

問題文の特徴をもとに、既存ライブラリの使いどころを探します。

| こういうとき | 用いるアルゴリズム |
| --- | --- |
| 3点の位置関係や、点が直線の左右どちらにあるか判定したい | [外積・ccw](crate::geometry::ccw) |
| 2つの線分が交差するか、端点で接触するかを判定したい | [線分交差判定](crate::geometry::segment_intersection) |
| 点集合を全て含む、最も外側の凸多角形を求めたい | [凸包（Andrew法）](crate::geometry::convex_hull) |
| 多角形の面積を小数の誤差なく求めたい | [靴紐公式・符号付き面積の2倍](crate::geometry::polygon_area) |
| 原点を基準に点を角度順へ並べたい | [偏角ソート](crate::geometry::polar_sort) |
| 2点間の距離を平方根を取らずに比較したい | [距離の2乗](crate::geometry::distance_squared) |

[チートシートのトップ](crate)
