//! ローカル実行向けのデバッグ出力です。

/// デバッグビルドでのみ標準エラーへ出力します。
///
/// [`eprintln!`] と同じフォーマット構文を受け取ります。debug assertion が有効なビルドでは
/// 標準エラーへ改行付きで出力します。release build では呼び出し全体がコンパイル時に除かれ、
/// 引数の式も評価されません。
///
/// # Examples
///
/// ```
/// atcoder::debug!("value = {}", 42);
/// ```
#[macro_export]
macro_rules! debug {
    ($($argument:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!($($argument)*)
    };
}
