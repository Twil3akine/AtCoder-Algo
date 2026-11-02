# 木の問題から解法を探す

対象は重みなしの無向木を基本とします。重み付きの木や一般のグラフは、各パターンの適用条件を確認してください。

| 問題文に現れる特徴 | 最初に考えること | 解法 |
| --- | --- | --- |
| 最も遠い2頂点、最長の単純パス | BFSを2回 | [直径・最遠点](crate::cheatsheet::tree::diameter) |
| 直径と同じ距離の頂点対、色が同じ両端 | 直径の中心から分岐を分ける | [直径の頂点対の数え上げ](crate::cheatsheet::tree::diameter_pairs) |
| 木を均等に分割、全頂点への最大距離を最小化 | 重心と中心を区別する | [中心・重心](crate::cheatsheet::tree::center_centroid) |
| 2頂点間の距離、k個上・k個進む | LCA・ダブリング | [距離・共通祖先](crate::cheatsheet::tree::lca_jump) |
| パス上の合計・最大値、値の更新 | パスを区間へ変換 | [HLD](crate::cheatsheet::tree::path_queries) |
| 全ての頂点を根にしてDPを計算 | 親・子からの寄与を再利用 | [全方位木DP](crate::cheatsheet::tree::rerooting) |

数え上げでは、単にアルゴリズムを選ぶだけでなく、**どの組を重複なく数えるか**も検討してください。
