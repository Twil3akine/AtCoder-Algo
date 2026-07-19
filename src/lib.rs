//! 競技プログラミング向けのアルゴリズム・データ構造ライブラリです。
//!
//! 実装は用途ごとのモジュールに分かれています。提出コードでは必要な項目だけを
//! `use atcoder::...` で読み込み、`cargo bundle` で単一ファイルに展開できます。

#[path = "lib/algorithm/mod.rs"]
pub mod algorithm;
#[path = "lib/data_structure/mod.rs"]
pub mod data_structure;
#[path = "lib/geometry.rs"]
pub mod geometry;
#[path = "lib/graph/mod.rs"]
pub mod graph;
#[path = "lib/grid.rs"]
pub mod grid;
#[path = "lib/io.rs"]
pub mod io;
#[path = "lib/math/mod.rs"]
pub mod math;
#[path = "lib/random.rs"]
pub mod random;
#[path = "lib/string.rs"]
pub mod string;
