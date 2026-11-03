# 1本の辺や1頂点を消すと分断されるか調べるなら、LowLink

## こういうときに使う

- ある辺を取り除くと連結成分が増えるか調べたいとき
- ある頂点を取り除くと残るグラフが分断されるか調べたいとき

## 使うアルゴリズム

**LowLink（橋・関節点の検出）**を用います。

## 解き方

DFS木の訪問順と、祖先へ戻れる最も浅い位置を使って橋と関節点を判定します。

## 注意・計算量

無向グラフ向けです。多重辺があるときは辺IDを区別する必要があります。 計算量は **O(V + E)** です。

**対応するAPI**：[`low_link()`](atcoder::graph::low_link::low_link)、[`LowLinkResult`](atcoder::graph::low_link::LowLinkResult)

**練習問題**：[AtCoder ABC075 C — Bridge](https://atcoder.jp/contests/abc075/tasks/abc075_c)
