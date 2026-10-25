//! 空白区切り入力とバッファ付き出力を提供します。
//!
//! [`crate::input!`] は標準入力をスレッドごとに保持するため、複数回呼び出しても読み取り位置を
//! 引き継ぎます。出力には、破棄時に flush する [`Writer`] を利用できます。

use std::cell::RefCell;
use std::fmt::Display;
use std::io::{self, BufRead, BufWriter, Write};

/// `BufRead` から空白区切りのトークンを読み取る scanner です。
///
/// 一度に1行ずつ読み込み、ASCII 空白文字で区切られたトークンを読みます。
/// 読み取り位置は scanner が保持するため、同じ scanner への呼び出し間で続きから読み取れます。
/// トークン読み取りの時間計算量はトークン長を `L` バイトとして償却 `O(L)` です。
/// 入力が尽きた場合、読み取りに失敗した場合、またはトークンを `T` に変換できない場合は panic します。
#[doc(hidden)]
pub struct Scanner<R: BufRead> {
    reader: R,
    buffer: String,
    cursor: usize,
}

impl<R: BufRead> Scanner<R> {
    /// 入力元 `reader` を指定して scanner を作ります。
    ///
    /// 読み取り元の現在位置から読み始めます。以後の入力取得は遅延して行います。
    ///
    /// # Examples
    ///
    /// ```
    /// use atcoder::io::Scanner;
    ///
    /// let mut scanner = Scanner::new(&b"12 hello"[..]);
    /// assert_eq!(scanner.token::<usize>(), 12);
    /// assert_eq!(scanner.token::<String>(), "hello");
    /// ```
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: String::new(),
            cursor: 0,
        }
    }

    /// 入力元 `reader` を指定して scanner を作ります。
    ///
    /// [`new`](Self::new) と同じです。
    pub fn with_reader(reader: R) -> Self {
        Self::new(reader)
    }

    /// 次の空白区切りトークンを `T` として返します。
    ///
    /// `T` は [`std::str::FromStr`] を実装する必要があります。トークンがなければ追加の入力を読み、
    /// 入力終端に達したとき、または変換に失敗したときは panic します。
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

    #[doc(hidden)]
    pub static WRITER: RefCell<BufWriter<io::Stdout>> =
        RefCell::new(BufWriter::new(io::stdout()));
}

/// `input!` の内部で入力形式を展開します。
///
/// 通常の型に加え、タプル、固定長入力、`chars`、`usize1`、`isize1` を扱います。
///
/// 各 `name: type` の形式で変数を宣言しながら値を読みます。`mut` を付けた名前は可変変数になります。
/// `chars` はトークンを Unicode scalar value の `Vec<char>` にし、`[T; n]` は `n` 個を `Vec<T>`
/// として読みます。`usize1` と `isize1` は入力整数から 1 を引くため、入力値は 1 以上である必要があります。
/// タプル型 `(T1, T2, ...)` は各要素を順に読み取ります。
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

/// グローバルなバッファへ改行なしで出力します。
///
/// 出力後は [`wflush`] を呼び出してください。呼び出しごとに標準出力へ直接書き込まず、
/// スレッドローカルなバッファへ蓄積します。
#[macro_export]
macro_rules! wprint {
    ($($argument:tt)*) => {
        $crate::io::WRITER.with(|writer| {
            std::io::Write::write_fmt(
                &mut *writer.borrow_mut(),
                format_args!($($argument)*),
            )
            .unwrap()
        })
    };
}

/// グローバルなバッファへ改行付きで出力します。
///
/// フォーマット構文は [`format_args!`] と同じです。実際の書き出しはバッファが満杯になるか、
/// [`wflush`] が呼ばれたときに行われます。
#[macro_export]
macro_rules! wprintln {
    () => {
        $crate::wprint!("\n")
    };
    ($($argument:tt)*) => {
        $crate::wprint!("{}\n", format_args!($($argument)*))
    };
}

/// [`wprint!`] と [`wprintln!`] のスレッドローカルな標準出力バッファを flush します。
///
/// 競技プログラムの終了前や、対話型問題で出力を相手へ届ける必要があるときに呼び出します。
/// 書き込みに失敗した場合は panic します。
pub fn wflush() {
    WRITER.with(|writer| writer.borrow_mut().flush().unwrap());
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
    ///
    /// 書き込みは内部バッファに蓄積されます。明示的に [`flush`](Self::flush) するか、値が破棄されると
    /// バッファが出力先へ書き出されます。
    pub fn with_writer(writer: W) -> Self {
        Self {
            writer: BufWriter::new(writer),
        }
    }

    /// `Display` 可能な `value` を改行なしで出力バッファへ追加します。
    ///
    /// 書き込みに失敗した場合は panic します。
    pub fn print(&mut self, value: impl Display) {
        write!(self.writer, "{value}").unwrap();
    }

    /// `Display` 可能な `value` と改行を出力バッファへ追加します。
    ///
    /// 書き込みに失敗した場合は panic します。
    pub fn println(&mut self, value: impl Display) {
        writeln!(self.writer, "{value}").unwrap();
    }

    /// `condition` が `true` なら `Yes`、`false` なら `No` を改行付きで出力します。
    pub fn print_yes_no(&mut self, condition: bool) {
        self.println(if condition { "Yes" } else { "No" });
    }

    /// `Yes` を改行付きで出力します。
    pub fn print_yes(&mut self) {
        self.print_yes_no(true);
    }

    /// `No` を改行付きで出力します。
    pub fn print_no(&mut self) {
        self.print_yes_no(false);
    }

    /// `values` の各要素を `separator` で連結し、末尾に改行を出力します。
    ///
    /// 空イテレータの場合も改行を1つ出力します。イテレータの要素数を `N`、値と区切り文字の
    /// 合計出力バイト数を `S` とすると、時間計算量は `O(N + S)` です。
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

    /// `values` の要素を区切りなしで連結し、末尾に改行を付けて出力します。
    pub fn join_nospace<T: Display>(&mut self, values: impl IntoIterator<Item = T>) {
        self.join(values, "");
    }

    /// `values` の要素を ASCII 空白1文字で区切り、末尾に改行を付けて出力します。
    pub fn join_whitespace<T: Display>(&mut self, values: impl IntoIterator<Item = T>) {
        self.join(values, " ");
    }

    /// `values` の要素を改行で区切って出力し、最後にも改行を付けます。
    pub fn join_line<T: Display>(&mut self, values: impl IntoIterator<Item = T>) {
        self.join(values, "\n");
    }

    /// バッファの内容を出力先へ書き出します。
    ///
    /// 書き込みに失敗した場合は panic します。
    pub fn flush(&mut self) {
        self.writer.flush().unwrap();
    }
}

impl Writer<io::StdoutLock<'static>> {
    /// 標準出力を出力先として作ります。
    ///
    /// 標準出力をロックしてラップします。出力を明示的に確定したい場合は [`flush`](Self::flush) を呼びます。
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
