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
