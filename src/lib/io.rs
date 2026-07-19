//! 空白区切り入力とバッファ付き出力を提供します。
//!
//! [`crate::input!`] は標準入力をスレッドごとに保持するため、複数回呼び出しても読み取り位置を
//! 引き継ぎます。出力には、破棄時に flush する [`Writer`] を利用できます。

use std::cell::RefCell;
use std::fmt::Display;
use std::io::{self, BufRead, BufWriter, Write};

/// `BufRead` から空白区切りのトークンを読み取ります。
///
/// トークンの読み取りは償却 `O(L)` です。`L` はトークンのバイト長です。
/// 入力が尽きた場合、または `T` への変換に失敗した場合は panic します。
#[doc(hidden)]
pub struct Scanner<R: BufRead> {
    reader: R,
    buffer: String,
    cursor: usize,
}

impl<R: BufRead> Scanner<R> {
    /// 入力元 `reader` を指定して scanner を作ります。
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: String::new(),
            cursor: 0,
        }
    }

    /// 次のトークンを `T` として返します。
    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            let bytes = self.buffer.as_bytes();
            while self.cursor < bytes.len() && bytes[self.cursor].is_ascii_whitespace() {
                self.cursor += 1;
            }
            let start = self.cursor;
            while self.cursor < bytes.len() && !bytes[self.cursor].is_ascii_whitespace() {
                self.cursor += 1;
            }
            if start < self.cursor {
                return self.buffer[start..self.cursor]
                    .parse()
                    .ok()
                    .expect("failed to parse token");
            }
            self.buffer.clear();
            self.cursor = 0;
            let read = self
                .reader
                .read_line(&mut self.buffer)
                .expect("failed to read input");
            assert!(read > 0, "input exhausted");
        }
    }
}

thread_local! {
    #[doc(hidden)]
    pub static SCANNER: RefCell<Scanner<io::BufReader<io::Stdin>>> =
        RefCell::new(Scanner::new(io::BufReader::new(io::stdin())));
}

/// `input!` の内部で入力形式を展開します。
///
/// 通常の型に加え、タプル、固定長入力、`chars`、`usize1`、`isize1` を扱います。
#[doc(hidden)]
#[macro_export]
macro_rules! read_value {
    ($scanner:expr, ($($ty:tt),*)) => {
        ($( $crate::read_value!($scanner, $ty) ),*)
    };
    ($scanner:expr, [$ty:tt; $len:expr]) => {
        (0..$len)
            .map(|_| $crate::read_value!($scanner, $ty))
            .collect::<Vec<_>>()
    };
    ($scanner:expr, chars) => {
        $scanner.token::<String>().chars().collect::<Vec<char>>()
    };
    ($scanner:expr, usize1) => {
        $scanner.token::<usize>() - 1
    };
    ($scanner:expr, isize1) => {
        $scanner.token::<isize>() - 1
    };
    ($scanner:expr, $ty:ty) => {
        $scanner.token::<$ty>()
    };
}

/// 標準入力から変数を宣言しながら値を読み取ります。
///
/// # Examples
///
/// ```no_run
/// atcoder::input! {
///     n: usize,
///     values: [i64; n],
///     mut position: usize1,
///     word: chars,
/// }
/// # let _ = (values, position, word);
/// ```
#[macro_export]
macro_rules! input {
    ($(,)?) => {};
    (mut $($var:ident),+ : $ty:tt $(, $($rest:tt)*)?) => {
        $(
            let mut $var = $crate::io::SCANNER.with(|scanner| {
                let mut scanner = scanner.borrow_mut();
                $crate::read_value!(&mut *scanner, $ty)
            });
        )+
        $($crate::input!($($rest)*);)?
    };
    ($($var:ident),+ : $ty:tt $(, $($rest:tt)*)?) => {
        $(
            let $var = $crate::io::SCANNER.with(|scanner| {
                let mut scanner = scanner.borrow_mut();
                $crate::read_value!(&mut *scanner, $ty)
            });
        )+
        $($crate::input!($($rest)*);)?
    };
}

/// `Write` 先へ効率よく出力するヘルパです。
///
/// 値は内部の [`BufWriter`] に蓄積され、[`flush`](Self::flush) または `Drop` 時に
/// 書き出されます。
pub struct Writer<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> Writer<W> {
    /// `writer` を出力先として作ります。
    pub fn with_writer(writer: W) -> Self {
        Self {
            writer: BufWriter::new(writer),
        }
    }

    /// 値を改行なしで出力します。
    pub fn print(&mut self, value: impl Display) {
        write!(self.writer, "{value}").unwrap();
    }

    /// 値を改行付きで出力します。
    pub fn println(&mut self, value: impl Display) {
        writeln!(self.writer, "{value}").unwrap();
    }

    /// `condition` に応じて `Yes` または `No` を出力します。
    pub fn print_yes_no(&mut self, condition: bool) {
        self.println(if condition { "Yes" } else { "No" });
    }

    /// `Yes` を出力します。
    pub fn print_yes(&mut self) {
        self.print_yes_no(true);
    }

    /// `No` を出力します。
    pub fn print_no(&mut self) {
        self.print_yes_no(false);
    }

    /// 要素を `separator` で連結し、末尾に改行を出力します。
    ///
    /// イテレータの要素数を `N`、出力文字数を `S` とすると `O(N + S)` です。
    pub fn join<T: Display>(&mut self, values: impl IntoIterator<Item = T>, separator: &str) {
        let mut values = values.into_iter();
        if let Some(first) = values.next() {
            self.print(first);
            for value in values {
                self.print(separator);
                self.print(value);
            }
        }
        self.println("");
    }

    /// 要素を区切りなしで1行に出力します。
    pub fn join_nospace<T: Display>(&mut self, values: impl IntoIterator<Item = T>) {
        self.join(values, "");
    }

    /// 要素を空白区切りで1行に出力します。
    pub fn join_whitespace<T: Display>(&mut self, values: impl IntoIterator<Item = T>) {
        self.join(values, " ");
    }

    /// 各要素を1行ずつ出力します。
    pub fn join_line<T: Display>(&mut self, values: impl IntoIterator<Item = T>) {
        self.join(values, "\n");
    }

    /// バッファの内容を出力先へ書き出します。
    pub fn flush(&mut self) {
        self.writer.flush().unwrap();
    }
}

impl Writer<io::StdoutLock<'static>> {
    /// 標準出力を出力先として作ります。
    pub fn new() -> Self {
        Self::with_writer(io::stdout().lock())
    }
}

impl Default for Writer<io::StdoutLock<'static>> {
    fn default() -> Self {
        Self::new()
    }
}

impl<W: Write> Drop for Writer<W> {
    fn drop(&mut self) {
        self.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::{Scanner, Writer};

    #[test]
    fn scans_supported_values() {
        let mut scanner = Scanner::new("3 hello\n".as_bytes());
        assert_eq!(scanner.token::<usize>(), 3);
        assert_eq!(scanner.token::<String>(), "hello");
    }

    #[test]
    fn writes_joined_values() {
        let mut output = Vec::new();
        {
            let mut writer = Writer::with_writer(&mut output);
            writer.join_whitespace([1, 2, 3]);
        }
        assert_eq!(output, b"1 2 3\n");
    }
}
