# 橋・関節点・1本の辺や1頂点を削除すると分断される

**問題の手掛かり**：道路（辺）や街（頂点）を1つ取り除いた際に、連結成分が増えるか調べる。

**着眼点**：DFS木において、子の部分木から親より上の祖先へ戻れるかを判定します。

**解法**：LowLinkを用いて橋（辺）と関節点（頂点）をまとめて列挙します。

**適用条件・注意**：無向グラフが対象です。多重辺では「親頂点」だけでなく辺IDを区別する必要があります。

**計算量**：O(V + E)。

**使うAPI**：[`low_link()`](crate::graph::low_link::low_link)、[`LowLinkResult`](crate::graph::low_link::LowLinkResult)。

**練習問題**：[AtCoder ABC075 C — Bridge](https://atcoder.jp/contests/abc075/tasks/abc075_c)。
