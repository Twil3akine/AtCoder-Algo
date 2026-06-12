#![allow(dead_code)]
#![allow(unused)]

use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::fmt::Debug;
use std::iter::StepBy;
use std::ops::{Deref, DerefMut, Index, RangeFrom};
use std::{
    cmp::{max, min},
    io::*,
    iter::zip,
    mem::swap,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    process::exit,
    time::Instant,
};

use itertools::Itertools;

// =============================================

/// ローカル実行時(デバッグビルド)だけ `eprintln!` を実行する。
///
/// 提出時のリリースビルドでは何も出力しないため、途中状態の確認に使える。
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!($($arg)*)
    };
}

// =============================================
// Scanner
// =============================================

/// 標準入力などの `BufRead` から空白区切りのトークンを高速に読み取る。
///
/// `token::<T>()` で任意の `FromStr` 実装型にパースする。
/// AtCoder 形式の入力では、通常は `input!` マクロ経由で利用する。
struct Scanner<R: BufRead> {
    /// 入力元。
    reader: R,
    /// 直近に読み込んだ1行分のバイト列。
    buf_str: Vec<u8>,
    /// `buf_str` に対応する空白区切りイテレータ。
    buf_iter: std::str::SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    /// 任意の `BufRead` を入力元として `Scanner` を作る。
    fn with_reader(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace(),
        }
    }

    /// 次の空白区切りトークンを読み、型 `T` に変換して返す。
    ///
    /// 現在の行に未読トークンがない場合は次の行を読み込む。
    /// パースに失敗した場合は panic する。
    fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed to parse token");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed to read line");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}

// =============================================
// グローバルな stdin / stdout
// =============================================

thread_local! {
    static SC: RefCell<Scanner<std::io::BufReader<std::io::Stdin>>> =
        RefCell::new(Scanner::with_reader(std::io::BufReader::new(stdin())));

    static WR: RefCell<BufWriter<std::io::Stdout>> =
        RefCell::new(BufWriter::new(stdout()));
}

// =============================================
// read_value! (input! の内部用)
// =============================================

/// `input!` の内部で使う値読み取りマクロ。
///
/// タプル、配列、`chars`、`usize1`、`isize1` などの競プロでよく使う
/// 入力形式をまとめて扱う。
macro_rules! read_value {
    // 1. タプル (例: (usize, i32, chars))
    ($sc:expr, ($($t:tt),*)) => {
        ( $(read_value!($sc, $t)),* )
    };

    // 2. 配列 (例: [usize; n], [[isize; w]; h], [(usize, usize); m])
    ($sc:expr, [$t:tt; $len:expr]) => {
        (0..$len).map(|_| read_value!($sc, $t)).collect::<Vec<_>>()
    };

    // 3. 特殊型: chars (文字列を Vec<char> に変換)
    ($sc:expr, chars) => {
        $sc.token::<String>().chars().collect::<Vec<char>>()
    };

    // 4. 特殊型: usize1 (1-indexed を 0-indexed の usize に変換)
    ($sc:expr, usize1) => {
        $sc.token::<usize>() - 1
    };

    // 5. 特殊型: isize1 (1-indexed を 0-indexed の isize に変換)
    ($sc:expr, isize1) => {
        $sc.token::<isize>() - 1
    };

    // 6. 通常の型 (usize, i64, String, f64 など)
    ($sc:expr, $t:ty) => {
        $sc.token::<$t>()
    };
}

// =============================================
// input! マクロ
// =============================================

/// グローバルな `Scanner` から変数を宣言しながら読み込む。
///
/// 例: `input!(n: usize, a: [i64; n], s: chars);`
/// 同じ型の変数は `a, b: usize` のようにまとめて書ける。
macro_rules! input {
    ($(,)?) => {};

    (mut $($var:ident),+ : $t:tt $(, $($rest:tt)*)?) => {
        $(
            let mut $var = SC.with(|sc| {
                let mut sc = sc.borrow_mut();
                read_value!(&mut *sc, $t)
            });
        )+
        $(input!($($rest)*);)?
    };

    ($($var:ident),+ : $t:tt $(, $($rest:tt)*)?) => {
        $(
            let $var = SC.with(|sc| {
                let mut sc = sc.borrow_mut();
                read_value!(&mut *sc, $t)
            });
        )+
        $(input!($($rest)*);)?
    };
}

// =============================================
// wprint! / wprintln! マクロ
// =============================================

/// グローバルな `BufWriter` に改行なしで出力する。
///
/// 標準の `print!` と同じ書式を受け取り、最後に `wflush()` でまとめて
/// フラッシュする用途を想定している。
macro_rules! wprint {
    ($($arg:tt)*) => {
        WR.with(|wr| write!(wr.borrow_mut(), $($arg)*).unwrap())
    };
}

/// グローバルな `BufWriter` に改行付きで出力する。
///
/// 標準の `println!` と同じ書式を受け取る。
macro_rules! wprintln {
    // 引数なし (改行のみ)
    () => {
        WR.with(|wr| writeln!(wr.borrow_mut()).unwrap())
    };
    ($($arg:tt)*) => {
        WR.with(|wr| writeln!(wr.borrow_mut(), $($arg)*).unwrap())
    };
}

/// BufWriter を明示的にフラッシュする。
/// wprintln! / wprint! を使う場合は main の末尾で必ず呼ぶこと。
fn wflush() {
    WR.with(|wr| wr.borrow_mut().flush().unwrap());
}

// =============================================
// Writer (join 系など既存のメソッドはそのまま)
// =============================================

/// `Write` 先を `BufWriter` で包んだ出力ヘルパ。
///
/// `println`、Yes/No 出力、配列の join 出力をまとめて扱う。
/// `Drop` 時に自動で flush される。
struct Writer<W: Write> {
    /// 実際のバッファ付き出力先。
    writer: BufWriter<W>,
}

impl<W: Write> Writer<W> {
    /// 改行なしで1つの値を出力する。
    fn print<S: std::fmt::Display>(&mut self, s: S) {
        write!(self.writer, "{}", s).unwrap();
    }

    /// 改行付きで1つの値を出力する。
    fn println<S: std::fmt::Display>(&mut self, s: S) {
        writeln!(self.writer, "{}", s).unwrap();
    }

    /// 条件が true なら `Yes`、false なら `No` を出力する。
    fn print_yes_no(&mut self, cnd: bool) {
        self.println(if cnd { "Yes" } else { "No" });
    }

    /// `Yes` を出力する。
    fn print_yes(&mut self) {
        self.print_yes_no(true);
    }

    /// `No` を出力する。
    fn print_no(&mut self) {
        self.print_yes_no(false);
    }

    /// イテレータの要素を区切り文字 `sep` で連結して1行に出力する。
    fn join<S: std::fmt::Display, I: IntoIterator<Item = S>>(&mut self, iter: I, sep: &str) {
        let mut it = iter.into_iter();
        if let Some(first) = it.next() {
            self.print(first);
            for v in it {
                self.print(sep);
                self.print(v);
            }
        }
        self.println("");
    }

    /// イテレータの要素を区切りなしで1行に出力する。
    fn join_nospace<S: std::fmt::Display, I: IntoIterator<Item = S>>(&mut self, iter: I) {
        self.join(iter, "");
    }

    /// イテレータの要素を空白区切りで1行に出力する。
    fn join_whitespace<S: std::fmt::Display, I: IntoIterator<Item = S>>(&mut self, iter: I) {
        self.join(iter, " ");
    }

    /// イテレータの要素を改行区切りで出力する。
    fn join_line<S: std::fmt::Display, I: IntoIterator<Item = S>>(&mut self, iter: I) {
        self.join(iter, "\n");
    }
}

impl Writer<std::io::StdoutLock<'static>> {
    /// 標準出力に書き込む `Writer` を作る。
    fn new() -> Self {
        Self {
            writer: BufWriter::new(stdout().lock()),
        }
    }
}

impl<W: Write> Drop for Writer<W> {
    fn drop(&mut self) {
        self.writer.flush().unwrap();
    }
}

// =============================================

/// 整数型向けの競プロ用数値ユーティリティ。
///
/// 繰り返し二乗法、mod 累乗、mod 逆元、最大公約数、最小公倍数を提供する。
/// `impl_fast_math!` によって主要な符号付き・符号なし整数型へ実装される。
trait FastMath {
    /// 繰り返し二乗法で `self.pow(n)` 相当を計算する。
    ///
    /// 計算量は `O(log n)`。
    fn fast_pow(self, n: Self) -> Self;
    /// `self^n mod m` を繰り返し二乗法で計算する。
    ///
    /// 計算量は `O(log n)`。
    fn mod_pow(self, n: Self, m: Self) -> Self;
    /// `mod m` における乗法逆元を返す。
    ///
    /// `m` が素数かつ `self` と互いに素である前提で、フェルマーの小定理により
    /// `self^(m - 2) mod m` を計算する。
    fn mod_inv(self, m: Self) -> Self;

    /// ユークリッドの互除法で最大公約数を返す。
    fn gcd(self, rhs: Self) -> Self;
    /// 最大公約数を使って最小公倍数を返す。
    ///
    /// どちらかが 0 の場合は 0 を返す。
    fn lcm(self, rhs: Self) -> Self;
}
/// 指定した整数型へ `FastMath` を実装する。
///
/// 競プロでよく使う `i64` / `usize` などに同じ実装をまとめて展開する。
macro_rules! impl_fast_math {
    ($($t:ty), *) => {
        $(
            impl FastMath for $t {
                fn fast_pow(mut self, mut n: Self) -> Self {
                    let mut res: $t = 1;
                    while n > 0 {
                        if n & 1 == 1 {
                            res *= self;
                        }
                        self *= self;
                        n >>= 1;
                    }

                    res
                }

                fn mod_pow(mut self, mut n: Self, m: Self) -> Self {
                    self %= m;
                    let mut res: $t = 1;
                    while n > 0 {
                        if n & 1 == 1 {
                            res = (res *self) % m;
                        }
                        self = (self * self) % m;
                        n >>= 1;
                    }
                    res
                }

                fn mod_inv(self, m: Self) -> Self {
                    self.mod_pow(m-2, m)
                }

                fn gcd(self, rhs: Self) -> Self {
                    let mut a = self;
                    let mut b = rhs;

                    while b != 0 {
                        let r = a % b;
                        a = b;
                        b = r;
                    }

                    a
                }

                fn lcm(self, rhs: Self) -> Self {
                    if self == 0 || rhs == 0 {
                        0
                    } else {
                        self / self.gcd(rhs) * rhs
                    }
                }
            }
        )*
    };
}

impl_fast_math!(i32, i64, isize, u32, u64, usize);

// =============================================

/// 最大値を優先して取り出すヒープ。
///
/// 標準ライブラリの `BinaryHeap` そのものの別名。
pub type MaxHeap<T> = BinaryHeap<T>;

/// 最小値を優先して取り出すヒープ。
///
/// 内部では `Reverse<T>` を使って `BinaryHeap` の順序を反転している。
/// `push` / `pop` / `peek` はすべて標準のヒープ操作と同じ計算量になる。
#[derive(Debug, Clone)]
pub struct MinHeap<T>(BinaryHeap<Reverse<T>>);
impl<T: Ord> MinHeap<T> {
    /// 空の最小ヒープを作る。
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    /// 要素を追加する。
    pub fn push(&mut self, item: T) {
        self.0.push(Reverse(item));
    }

    /// 最小の要素を取り出す。
    ///
    /// 空の場合は `None` を返す。
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|Reverse(v)| v)
    }

    /// 最小の要素の参照を返す。
    ///
    /// 空の場合は `None` を返す。
    pub fn peek(&self) -> Option<&T> {
        self.0.peek().map(|Reverse(v)| v)
    }

    /// 現在ヒープに入っている要素数を返す。
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// ヒープが空なら `true` を返す。
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// =============================================

/// 軽量な Xorshift 乱数生成器。
///
/// 競プロのランダムテストやヒューリスティックで使うための簡易 PRNG。
/// 暗号用途には使わない。
struct Xorshift {
    /// 現在の内部状態。
    seed: u64,
}
impl Xorshift {
    /// 初期シードを指定して乱数生成器を作る。
    ///
    /// `seed == 0` の場合は固定の非ゼロ値に置き換える。
    fn new(seed: u64) -> Self {
        Xorshift {
            seed: if seed == 0 { 88172645463325252 } else { seed },
        }
    }

    /// 次の `u64` 乱数を返し、内部状態を更新する。
    fn next(&mut self) -> u64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        self.seed
    }

    /// `min` 以上 `max` 以下の `usize` 乱数を返す。
    ///
    /// `min <= max` である必要がある。
    fn next_range(&mut self, min: usize, max: usize) -> usize {
        min + (self.next() as usize % (max - min + 1))
    }

    /// `0.0` 以上 `1.0` 未満の `f64` 乱数を返す。
    fn next_f64(&mut self) -> f64 {
        self.next() as f64 / u64::MAX as f64
    }
}

// =============================================

/// 各位置を中心とする回文半径を求める。
///
/// 元配列 `array` の各要素の間に区切り要素を挟んだ列を内部的に作り、
/// 奇数長回文と偶数長回文をまとめて 1 つの配列で扱う。
fn manacher<T: Eq + Clone>(array: &[T]) -> Vec<usize> {
    let n: usize = array.len();

    // b = [None, Some(a[0]), None, Some(a[1]), ..., Some(a[n-1]), None]
    let mut b: Vec<Option<T>> = Vec::with_capacity(2 * n + 1);
    for x in array {
        b.push(None);
        b.push(Some(x.clone()));
    }
    b.push(None);

    let m: usize = b.len();
    let mut rad: Vec<usize> = vec![0; m];

    // 現在見ている最右回文区間 [l, r)
    let mut l: usize = 0;
    let mut r: usize = 0;

    for i in 0..m {
        let mut k: usize = 1;

        if i < r {
            let j: usize = l + r - 1 - i; // i の鏡像
            k = rad[j].min(r - i);
        }

        while i >= k && i + k < m && b[i - k] == b[i + k] {
            k += 1;
        }

        rad[i] = k;

        if i + k > r {
            l = i + 1 - k;
            r = i + k;
        }
    }

    rad
}

// =============================================

/// 8近傍の移動方向。
///
/// 順に右、上、左、下、右上、左上、左下、右下を表す。
const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 0),
    (-1, 1),
    (-1, -1),
    (1, -1),
    (1, 1),
]; // 右, 上, 左, 下, 右上、左上、左下、右下

// =============================================

fn main() {
    let mut wr = Writer::new();

    input!(
        
    );
}
