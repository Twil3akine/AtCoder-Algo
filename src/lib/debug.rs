//! ローカル実行向けのデバッグ出力です。

/// デバッグビルドでのみ標準エラーへ出力します。
///
/// release build では引数を評価せず、何も出力しません。
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
