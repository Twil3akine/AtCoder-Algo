#![doc = include_str!("../index.md")]

#[doc = include_str!("../tree/index.md")]
#[doc(alias = "木")]
#[doc(alias = "木の問題")]
pub mod tree {
    #[doc = include_str!("../tree/diameter.md")]
    #[doc(alias = "最遠点")]
    #[doc(alias = "木の直径")]
    #[doc(alias = "最長パス")]
    pub mod diameter {}

    #[doc = include_str!("../tree/diameter_pairs.md")]
    #[doc(alias = "直径の頂点対")]
    #[doc(alias = "同じ色")]
    #[doc(alias = "直径と等しい距離")]
    pub mod diameter_pairs {}

    #[doc = include_str!("../tree/center_centroid.md")]
    #[doc(alias = "木の中心")]
    #[doc(alias = "木の重心")]
    #[doc(alias = "均等に分割")]
    pub mod center_centroid {}

    #[doc = include_str!("../tree/lca_jump.md")]
    #[doc(alias = "共通祖先")]
    #[doc(alias = "2頂点間距離")]
    #[doc(alias = "k番目の頂点")]
    pub mod lca_jump {}

    #[doc = include_str!("../tree/path_queries.md")]
    #[doc(alias = "パス上の合計")]
    #[doc(alias = "辺の更新")]
    #[doc(alias = "木上のクエリ")]
    pub mod path_queries {}

    #[doc = include_str!("../tree/rerooting.md")]
    #[doc(alias = "全頂点を根")]
    #[doc(alias = "距離総和")]
    #[doc(alias = "全方位木DP")]
    pub mod rerooting {}
}

#[doc = include_str!("../graph/index.md")]
#[doc(alias = "グラフ")]
#[doc(alias = "グラフの問題")]
pub mod graph {
    #[doc = include_str!("../graph/bfs.md")]
    #[doc(alias = "重みなし最短路")]
    #[doc(alias = "辺の本数")]
    #[doc(alias = "到達可能性")]
    pub mod bfs {}

    #[doc = include_str!("../graph/zero_one_bfs.md")]
    #[doc(alias = "重み0と1")]
    #[doc(alias = "無料移動")]
    #[doc(alias = "ワープ")]
    pub mod zero_one_bfs {}

    #[doc = include_str!("../graph/dijkstra.md")]
    #[doc(alias = "非負重み")]
    #[doc(alias = "最小費用")]
    #[doc(alias = "ダイクストラ")]
    pub mod dijkstra {}

    #[doc = include_str!("../graph/scc.md")]
    #[doc(alias = "相互到達")]
    #[doc(alias = "強連結成分")]
    #[doc(alias = "有向グラフの閉路")]
    pub mod scc {}

    #[doc = include_str!("../graph/low_link.md")]
    #[doc(alias = "橋")]
    #[doc(alias = "関節点")]
    #[doc(alias = "分断される辺")]
    pub mod low_link {}

    #[doc = include_str!("../graph/max_flow.md")]
    #[doc(alias = "最大流")]
    #[doc(alias = "最大マッチング")]
    #[doc(alias = "最小カット")]
    pub mod max_flow {}

    #[doc = include_str!("../graph/topological_sort.md")]
    #[doc(alias = "依存関係")]
    #[doc(alias = "作業順序")]
    #[doc(alias = "DAG")]
    pub mod topological_sort {}
}

#[doc = include_str!("../data_structure/index.md")]
#[doc(alias = "データ構造・区間クエリ")]
pub mod data_structure {
    #[doc = include_str!("../data_structure/union_find.md")]
    #[doc(alias = "連結成分")]
    #[doc(alias = "辺を追加")]
    #[doc(alias = "同じグループ")]
    pub mod union_find {}

    #[doc = include_str!("../data_structure/rollback_union_find.md")]
    #[doc(alias = "巻き戻し")]
    #[doc(alias = "併合を戻す")]
    #[doc(alias = "オフライン動的連結性")]
    pub mod rollback_union_find {}

    #[doc = include_str!("../data_structure/segment_tree.md")]
    #[doc(alias = "一点更新")]
    #[doc(alias = "区間最大値")]
    #[doc(alias = "区間集約")]
    pub mod segment_tree {}

    #[doc = include_str!("../data_structure/segment_search.md")]
    #[doc(alias = "max_right")]
    #[doc(alias = "min_left")]
    #[doc(alias = "条件を満たす境界")]
    pub mod segment_search {}

    #[doc = include_str!("../data_structure/lazy_segment_tree.md")]
    #[doc(alias = "区間加算")]
    #[doc(alias = "区間代入")]
    #[doc(alias = "区間更新区間取得")]
    pub mod lazy_segment_tree {}

    #[doc = include_str!("../data_structure/dual_segment_tree.md")]
    #[doc(alias = "区間更新一点取得")]
    #[doc(alias = "範囲加算一点取得")]
    #[doc(alias = "Dual Segment Tree")]
    pub mod dual_segment_tree {}

    #[doc = include_str!("../data_structure/fenwick_tree.md")]
    #[doc(alias = "BIT")]
    #[doc(alias = "前計算ではない累積和")]
    #[doc(alias = "一点加算区間和")]
    pub mod fenwick_tree {}

    #[doc = include_str!("../data_structure/inversion_count.md")]
    #[doc(alias = "転倒数")]
    #[doc(alias = "大小の逆転")]
    #[doc(alias = "逆順のペア")]
    pub mod inversion_count {}

    #[doc = include_str!("../data_structure/prefix_kth.md")]
    #[doc(alias = "k番目の要素")]
    #[doc(alias = "累積度数")]
    #[doc(alias = "Fenwick lower_bound")]
    pub mod prefix_kth {}

    #[doc = include_str!("../data_structure/sparse_table.md")]
    #[doc(alias = "静的RMQ")]
    #[doc(alias = "更新なし区間最小値")]
    #[doc(alias = "区間GCD")]
    pub mod sparse_table {}

    #[doc = include_str!("../data_structure/swag.md")]
    #[doc(alias = "スライド区間")]
    #[doc(alias = "両端キューではない")]
    #[doc(alias = "Sliding Window Aggregation")]
    pub mod swag {}

    #[doc = include_str!("../data_structure/priority_queue.md")]
    #[doc(alias = "次の最小候補")]
    #[doc(alias = "最小値を繰り返し取り出す")]
    #[doc(alias = "Priority Queue")]
    pub mod priority_queue {}
}

#[doc = include_str!("../array/index.md")]
#[doc(alias = "配列・並べ替え")]
pub mod array {
    #[doc = include_str!("../array/compress.md")]
    #[doc(alias = "座標圧縮")]
    #[doc(alias = "値が大きい")]
    #[doc(alias = "小さい順位に置き換える")]
    pub mod compress {}

    #[doc = include_str!("../array/rle.md")]
    #[doc(alias = "連続する同じ値")]
    #[doc(alias = "連続個数")]
    #[doc(alias = "ランレングス圧縮")]
    pub mod rle {}

    #[doc = include_str!("../array/descending_sort.md")]
    #[doc(alias = "降順")]
    #[doc(alias = "大きい順")]
    #[doc(alias = "逆順ソート")]
    pub mod descending_sort {}
}

#[doc = include_str!("../string/index.md")]
#[doc(alias = "文字列")]
pub mod string {
    #[doc = include_str!("../string/kmp_search.md")]
    #[doc(alias = "パターン検索")]
    #[doc(alias = "文字列出現位置")]
    #[doc(alias = "部分文字列を探す")]
    pub mod kmp_search {}

    #[doc = include_str!("../string/z_algorithm.md")]
    #[doc(alias = "最長共通接頭辞")]
    #[doc(alias = "Z配列")]
    #[doc(alias = "先頭との一致長")]
    pub mod z_algorithm {}

    #[doc = include_str!("../string/palindrome.md")]
    #[doc(alias = "回文")]
    #[doc(alias = "最長回文部分文字列")]
    #[doc(alias = "奇数長偶数長回文")]
    pub mod palindrome {}

    #[doc = include_str!("../string/rolling_hash.md")]
    #[doc(alias = "部分文字列一致")]
    #[doc(alias = "部分文字列比較")]
    #[doc(alias = "ハッシュで比較")]
    pub mod rolling_hash {}

    #[doc = include_str!("../string/alphabet_index.md")]
    #[doc(alias = "アルファベットを番号に")]
    #[doc(alias = "文字頻度")]
    #[doc(alias = "英字のindex")]
    pub mod alphabet_index {}

    #[doc = include_str!("../string/parse_digits.md")]
    #[doc(alias = "数字を数値に")]
    #[doc(alias = "文字列を整数に")]
    #[doc(alias = "10進数を読む")]
    pub mod parse_digits {}
}
