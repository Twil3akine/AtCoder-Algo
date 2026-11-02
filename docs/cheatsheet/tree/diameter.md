# 最も遠い2頂点・木の直径

**問題の手掛かり**：木の中で最も遠い2頂点、最長の単純パス、直径の長さやパスを求める。

**着眼点**：木では、任意の頂点から最も遠い頂点を`a`とし、`a`から最も遠い頂点を`b`とすると、`a`と`b`の距離が直径になります。

**解法**：BFSを2回使います。直径の端点とパスを得たら、中央の頂点（または辺）も調べられます。

**適用条件・注意**：この2回探索の性質は木に対するものです。一般のグラフにはそのまま適用できません。

**計算量**：O(N)。

**使うAPI**：[`diameter()`](crate::graph::tree::diameter)、[`diameter_info()`](crate::graph::tree::diameter_info)、[`centers()`](crate::graph::tree::centers)。

**練習問題**：[CSES — Tree Diameter](https://cses.fi/problemset/task/1131)。
