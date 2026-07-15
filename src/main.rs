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

/// 法 `MOD` 上の整数を扱う構造体。
///
/// 値は常に `0 <= val < MOD` に正規化される。
/// `+`, `-`, `*`, `/` と各種代入演算子を mod 上の演算として使える。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ModInt<const MOD: i64> {
    /// 正規化済みの値。
    val: i64,
}

impl<const MOD: i64> ModInt<MOD> {
    /// 任意の整数 `val` を `mod MOD` に正規化して作る。
    fn new(mut val: i64) -> Self {
        val %= MOD;
        if val < 0 {
            val += MOD;
        }
        Self { val }
    }

    /// 内部の正規化済み値を返す。
    fn val(&self) -> i64 {
        self.val
    }

    /// 乗法逆元を返す。
    ///
    /// `MOD` が素数で、値が `MOD` と互いに素である前提で `pow(MOD - 2)` を使う。
    fn inv(&self) -> Self {
        self.pow(MOD - 2)
    }

    /// 繰り返し二乗法で `self^exp` を計算する。
    ///
    /// 計算量は `O(log exp)`。
    fn pow(&self, mut exp: i64) -> Self {
        let mut res = 1;
        let mut base = self.val;

        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * base) % MOD;
            }
            base = (base * base) % MOD;
            exp /= 2;
        }

        Self::new(res)
    }
}

impl<const MOD: i64> Add for ModInt<MOD> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.val + rhs.val())
    }
}

impl<const MOD: i64> Sub for ModInt<MOD> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.val - rhs.val())
    }
}

impl<const MOD: i64> Mul for ModInt<MOD> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.val * rhs.val())
    }
}

impl<const MOD: i64> Div for ModInt<MOD> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}

impl<const MOD: i64> AddAssign for ModInt<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const MOD: i64> SubAssign for ModInt<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const MOD: i64> MulAssign for ModInt<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const MOD: i64> DivAssign for ModInt<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

/// AtCoder でよく使う法 `998244353` の `ModInt`。
type Mod998 = ModInt<998_244_353>;

/// AtCoder でよく使う法 `1000000007` の `ModInt`。
type Mod107 = ModInt<1_000_000_007>;

// =============================================

/// 英字を `0..26` の添字に変換する拡張トレイト。
///
/// `a` / `A` を 0、`b` / `B` を 1 のように扱う。
/// `char` と `u8` に実装している。
trait AlphaExt {
    /// アルファベットを 0-indexed の添字に変換する。
    fn to_idx(self) -> usize;
}

impl AlphaExt for char {
    /// `char` を小文字化して `a` からの距離に変換する。
    fn to_idx(self) -> usize {
        (self.to_ascii_lowercase() as u8 - b'a') as usize
    }
}

impl AlphaExt for u8 {
    /// ASCII バイトを小文字化して `b'a'` からの距離に変換する。
    fn to_idx(self) -> usize {
        (self.to_ascii_lowercase() - b'a') as usize
    }
}

// =============================================

/// スライスを降順に並べるための拡張トレイト。
///
/// 標準の昇順 `sort` / `sort_unstable` に対して、比較順を反転した
/// 降順ソートを短く書けるようにする。
pub trait SortReverse {
    /// スライスを降順に安定ソートする。
    fn sort_reverse(&mut self);

    /// スライスを降順に非安定ソートする。
    fn sort_unstable_reverse(&mut self);
}

impl<T: Ord> SortReverse for [T] {
    /// `sort_by` で比較順を反転して降順に並べる。
    fn sort_reverse(&mut self) {
        self.sort_by(|a, b| b.cmp(a));
    }

    /// `sort_unstable_by` で比較順を反転して降順に並べる。
    fn sort_unstable_reverse(&mut self) {
        self.sort_unstable_by(|a, b| b.cmp(a));
    }
}

// =============================================

/// スライスを座標圧縮する拡張トレイト。
///
/// 元の値の大小関係を保ったまま、各値を `0..k` の連番に変換する。
trait Compress<T> {
    /// 座圧後の配列と、添字から元の値へ戻すための値一覧を返す。
    ///
    /// 戻り値 `(compressed, vals)` について、`compressed[i]` は `self[i]` の圧縮後の値、
    /// `vals[j]` は圧縮後の値 `j` に対応する元の値。
    fn compressed(&self) -> (Vec<usize>, Vec<T>);
}

impl<T: Ord + Clone> Compress<T> for [T] {
    /// ソート済み重複除去配列に対して二分探索し、各要素の圧縮後添字を求める。
    ///
    /// 計算量は `O(N log N)`。
    fn compressed(&self) -> (Vec<usize>, Vec<T>) {
        let mut vals = self.to_vec();
        vals.sort_unstable();
        vals.dedup();

        let compressed = self
            .iter()
            .map(|x| vals.binary_search(x).unwrap())
            .collect();

        (compressed, vals)
    }
}

// =============================================

/// イテレータに対してランレングス圧縮を行う拡張トレイト。
///
/// 連続する同一の要素をまとめ、要素とその連続回数の組に変換します。
pub trait RleExt: Iterator {
    /// イテレータを消費し、`(要素, 連続した数)` の配列を返します。
    ///
    /// 戻り値 `Vec<(Self::Item, usize)>` について、タプルの第1要素は元の値、
    /// 第2要素はその値が連続して出現した回数を表します。
    fn rle(self) -> Vec<(Self::Item, usize)>
    where
        Self: Sized,
        Self::Item: PartialEq;
}

impl<I: Iterator> RleExt for I {
    /// イテレータを順次走査し、直前の要素と比較して連続回数をカウントします。
    ///
    /// 計算量は `O(N)`（`N` はイテレータの要素数）です。
    fn rle(self) -> Vec<(Self::Item, usize)>
    where
        Self: Sized,
        Self::Item: PartialEq,
    {
        let mut iter = self;
        let mut result = Vec::new();

        let mut current = match iter.next() {
            Some(v) => v,
            None => return result,
        };

        let mut count = 1;

        for item in iter {
            if item == current {
                count += 1;
            } else {
                result.push((current, count));
                current = item;
                count = 1;
            }
        }

        result.push((current, count));

        result
    }
}

// =============================================

/// 素集合データ構造 Disjoint Set Union。
///
/// 集合の併合、同一集合判定、集合サイズ取得をほぼ定数時間で行う。
/// 経路圧縮とサイズによる併合を使う。
struct UnionFind {
    /// 各頂点の親または集合サイズを表す配列。
    ///
    /// 根では負の集合サイズ、非根では親の添字を保持する。
    parents: Vec<isize>,

    /// 現在の連結成分数。
    group_count: usize,
}

impl UnionFind {
    /// `0..n` の各要素が独立した集合である状態を作る。
    fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            group_count: n,
        }
    }

    /// `x` が属する集合の代表元を返す。
    ///
    /// 経路圧縮により、以後の探索が速くなる。
    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] < 0 {
            x
        } else {
            let p = self.parents[x] as usize;
            let root = self.find(p);
            self.parents[x] = root as isize;
            root
        }
    }

    /// `x` と `y` の集合を併合する。
    ///
    /// すでに同じ集合なら `false`、新しく併合したなら `true` を返す。
    fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.parents[root_x] > self.parents[root_y] {
            swap(&mut root_x, &mut root_y);
        }

        self.parents[root_x] += self.parents[root_y];
        self.parents[root_y] = root_x as isize;

        self.group_count -= 1;

        true
    }

    /// `x` と `y` が同じ集合に属しているかを返す。
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// `x` が属する集合のサイズを返す。
    fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        (-self.parents[root]) as usize
    }

    /// 現在の集合数を返す。
    fn group_count(&self) -> usize {
        self.group_count
    }
}

// =============================================

/// 汎用セグメント木。
///
/// `op` に結合演算、`e` に単位元を渡して使う。
/// 区間取得は半開区間 `[l, r)` で行う。
struct SegmentTree<T> {
    /// 2つの値をまとめる演算。
    op: fn(T, T) -> T,

    /// 単位元。
    e: T,

    /// 元の配列の長さ。
    len: usize,

    /// セグメント木内部で使う葉の数。
    ///
    /// `len` 以上の最小の2冪。
    size: usize,

    /// セグメント木の内部配列。
    ///
    /// 1-indexed の完全二分木として管理する。
    /// 根は `data[1]`、葉は `data[size + i]`。
    data: Vec<T>,
}

impl<T: Copy> SegmentTree<T> {
    /// 長さ `len` のセグメント木を作る。
    ///
    /// 初期値はすべて単位元 `e` になる。
    fn new(op: fn(T, T) -> T, len: usize, e: T) -> Self {
        let size = len.next_power_of_two();

        Self {
            op,
            e,
            len,
            size,
            data: vec![e; 2 * size],
        }
    }

    /// 配列 `ary` からセグメント木を構築する。
    ///
    /// 計算量は `O(N)`。
    fn from(op: fn(T, T) -> T, ary: Vec<T>, e: T) -> Self {
        let len = ary.len();
        let size = len.next_power_of_two();
        let mut data = vec![e; 2 * size];

        for i in 0..len {
            data[size + i] = ary[i];
        }

        for i in (1..size).rev() {
            data[i] = op(data[i << 1], data[i << 1 | 1]);
        }

        Self {
            op,
            e,
            len,
            size,
            data,
        }
    }

    /// `idx` 番目の値を `v` に更新する。
    ///
    /// `idx` は 0-indexed。
    /// 計算量は `O(log N)`。
    fn apply(&mut self, mut idx: usize, v: T) {
        idx += self.size;
        self.data[idx] = v;

        while idx > 1 {
            idx >>= 1;
            self.data[idx] = (self.op)(self.data[idx << 1], self.data[idx << 1 | 1]);
        }
    }

    /// `idx` 番目の値を返す。
    ///
    /// `idx` は 0-indexed。
    fn at(&self, idx: usize) -> T {
        self.data[self.size + idx]
    }

    /// 半開区間 `[l, r)` の集約値を返す。
    ///
    /// `l`, `r` は 0-indexed。
    /// 計算量は `O(log N)`。
    fn get(&self, mut l: usize, mut r: usize) -> T {
        l += self.size;
        r += self.size;

        let mut left = self.e;
        let mut right = self.e;

        while l < r {
            if l & 1 == 1 {
                left = (self.op)(left, self.data[l]);
                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;
                right = (self.op)(self.data[r], right);
            }

            l >>= 1;
            r >>= 1;
        }

        (self.op)(left, right)
    }

    /// `[l, r)` が条件 `pred` を満たす最大の `r` を探す。
    ///
    /// 左端 `l` を固定し、右方向に伸ばしていく。
    ///
    /// `pred(e)` は `true` である必要がある。
    fn max_right<F>(&self, mut l: usize, pred: F) -> usize
    where
        F: Fn(T) -> bool,
    {
        assert!(l <= self.len);
        assert!(pred(self.e));

        if l == self.len {
            return self.len;
        }

        l += self.size;
        let mut acc = self.e;

        loop {
            while l % 2 == 0 {
                l >>= 1;
            }

            let next = (self.op)(acc, self.data[l]);

            if !pred(next) {
                while l < self.size {
                    l <<= 1;
                    let next = (self.op)(acc, self.data[l]);

                    if pred(next) {
                        acc = next;
                        l += 1;
                    }
                }

                return (l - self.size).min(self.len);
            }

            acc = next;
            l += 1;

            if l.is_power_of_two() {
                break;
            }
        }

        self.len
    }

    /// `[l, r)` が条件 `pred` を満たす最小の `l` を探す。
    ///
    /// 右端 `r` を固定し、左方向に伸ばしていく。
    ///
    /// `pred(e)` は `true` である必要がある。
    fn min_left<F>(&self, mut r: usize, pred: F) -> usize
    where
        F: Fn(T) -> bool,
    {
        assert!(r <= self.len);
        assert!(pred(self.e));

        if r == 0 {
            return 0;
        }

        r += self.size;
        let mut acc = self.e;

        loop {
            r -= 1;

            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }

            let next = (self.op)(self.data[r], acc);

            if !pred(next) {
                while r < self.size {
                    r = r << 1 | 1;
                    let next = (self.op)(self.data[r], acc);

                    if pred(next) {
                        acc = next;
                        r -= 1;
                    }
                }

                return (r + 1 - self.size).min(self.len);
            }

            acc = next;

            if r.is_power_of_two() {
                break;
            }
        }

        0
    }
}

impl<T> Index<usize> for SegmentTree<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.size + index]
    }
}

impl<T: Copy + Debug> SegmentTree<T> {
    /// 元配列部分 `[0, len)` だけを表示する。
    fn debug_values(&self) {
        debug!("{:?}", &self.data[self.size..self.size + self.len]);
    }

    /// 元配列部分 `[l, r)` だけを表示する。
    fn debug_range(&self, l: usize, r: usize) {
        debug!("{:?}", &self.data[self.size + l..self.size + r]);
    }

    /// 内部木全体を表示する。
    fn debug_tree(&self) {
        debug!("{:?}", self.data);
    }
}

// =============================================

/// 汎用双対セグメント木。
///
/// `op` に操作の合成、`e` に単位元を渡して使う。
/// 区間更新は半開区間 `[l, r)` で行う。
///
/// `op(a, b)` は「操作 a の後に操作 b を行う」という意味。
struct DualSegmentTree<T> {
    /// 2つの操作を合成する演算。
    op: fn(T, T) -> T,

    /// 単位元。
    e: T,

    /// 元の配列の長さ。
    len: usize,

    /// セグメント木内部で使う葉の数。
    size: usize,

    /// 木の高さ。
    log: usize,

    /// 遅延値を持つ配列。
    ///
    /// 1-indexed の完全二分木として管理する。
    data: Vec<T>,
}

impl<T: Copy> DualSegmentTree<T> {
    /// 長さ `len` の双対セグメント木を作る。
    ///
    /// 初期値はすべて単位元 `e` になる。
    fn new(op: fn(T, T) -> T, len: usize, e: T) -> Self {
        let size = len.next_power_of_two().max(1);
        let log = size.trailing_zeros() as usize;

        Self {
            op,
            e,
            len,
            size,
            log,
            data: vec![e; 2 * size],
        }
    }

    /// 配列 `ary` から双対セグメント木を構築する。
    fn from(op: fn(T, T) -> T, ary: Vec<T>, e: T) -> Self {
        let len = ary.len();
        let size = len.next_power_of_two().max(1);
        let log = size.trailing_zeros() as usize;
        let mut data = vec![e; 2 * size];

        for i in 0..len {
            data[size + i] = ary[i];
        }

        Self {
            op,
            e,
            len,
            size,
            log,
            data,
        }
    }

    /// ノード `idx` に操作 `v` を合成する。
    fn all_apply(&mut self, idx: usize, v: T) {
        self.data[idx] = (self.op)(self.data[idx], v);
    }

    /// ノード `idx` の遅延値を子に降ろす。
    fn push(&mut self, idx: usize) {
        let v = self.data[idx];

        self.all_apply(idx << 1, v);
        self.all_apply(idx << 1 | 1, v);

        self.data[idx] = self.e;
    }

    /// `idx` に向かう経路上の遅延値を下に降ろす。
    fn push_path(&mut self, idx: usize) {
        let idx = idx + self.size;

        for h in (1..=self.log).rev() {
            self.push(idx >> h);
        }
    }

    /// 半開区間 `[l, r)` に操作 `v` を適用する。
    ///
    /// `l`, `r` は 0-indexed。
    /// 計算量は `O(log N)`。
    fn apply(&mut self, mut l: usize, mut r: usize, v: T) {
        assert!(l <= r);
        assert!(r <= self.len);

        if l == r {
            return;
        }

        self.push_path(l);
        self.push_path(r - 1);

        l += self.size;
        r += self.size;

        while l < r {
            if l & 1 == 1 {
                self.all_apply(l, v);
                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;
                self.all_apply(r, v);
            }

            l >>= 1;
            r >>= 1;
        }
    }

    /// `idx` 番目の値を返す。
    ///
    /// `idx` は 0-indexed。
    /// 計算量は `O(log N)`。
    fn at(&self, mut idx: usize) -> T {
        assert!(idx < self.len);

        idx += self.size;

        let mut acc = self.e;

        while idx > 0 {
            acc = (self.op)(acc, self.data[idx]);
            idx >>= 1;
        }

        acc
    }
}

// =============================================

/// 2次元双対セグメント木。
///
/// 矩形 `[u, d) x [l, r)` に更新をかけ、点 `(i, j)` の値を取得する。
///
/// 加算・max・min など、合成順を気にしなくてよい演算向け。
struct DualSegmentTree2D<T> {
    op: fn(T, T) -> T,
    e: T,
    h: usize,
    w: usize,
    size_h: usize,
    size_w: usize,
    data: Vec<T>,
}

impl<T: Copy> DualSegmentTree2D<T> {
    /// `h x w` の2次元双対セグメント木を作る。
    fn new(h: usize, w: usize, op: fn(T, T) -> T, e: T) -> Self {
        let size_h = h.next_power_of_two();
        let size_w = w.next_power_of_two();

        Self {
            op,
            e,
            h,
            w,
            size_h,
            size_w,
            data: vec![e; 4 * size_h * size_w],
        }
    }

    /// 2次元配列上の添字を1次元に潰す。
    fn id(&self, x: usize, y: usize) -> usize {
        x * (2 * self.size_w) + y
    }

    /// 1次元区間 `[l, r)` をセグ木上のノード集合に分解する。
    fn range_nodes(mut l: usize, mut r: usize, size: usize) -> Vec<usize> {
        l += size;
        r += size;

        let mut nodes = Vec::new();

        while l < r {
            if l & 1 == 1 {
                nodes.push(l);
                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;
                nodes.push(r);
            }

            l >>= 1;
            r >>= 1;
        }

        nodes
    }

    /// 矩形 `[u, d) x [l, r)` に `v` を適用する。
    ///
    /// 計算量: `O(log H log W)`
    fn apply(&mut self, u: usize, d: usize, l: usize, r: usize, v: T) {
        assert!(u <= d && d <= self.h);
        assert!(l <= r && r <= self.w);

        if u == d || l == r {
            return;
        }

        let xs = Self::range_nodes(u, d, self.size_h);
        let ys = Self::range_nodes(l, r, self.size_w);

        for x in xs {
            for &y in &ys {
                let idx = self.id(x, y);
                self.data[idx] = (self.op)(self.data[idx], v);
            }
        }
    }

    /// 点 `(i, j)` の値を返す。
    ///
    /// 計算量: `O(log H log W)`
    fn at(&self, i: usize, j: usize) -> T {
        assert!(i < self.h);
        assert!(j < self.w);

        let mut xs = Vec::new();
        let mut x = i + self.size_h;
        while x > 0 {
            xs.push(x);
            x >>= 1;
        }

        let mut ys = Vec::new();
        let mut y = j + self.size_w;
        while y > 0 {
            ys.push(y);
            y >>= 1;
        }

        let mut res = self.e;

        for &x in &xs {
            for &y in &ys {
                res = (self.op)(res, self.data[self.id(x, y)]);
            }
        }

        res
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

/// 2次元グリッド上の座標が範囲内かを判定する。
///
/// `coord = (row, col)` が `0 <= row < h` かつ `0 <= col < w` なら `true`。
fn is_valid_range(h: usize, w: usize, coord: (usize, usize)) -> bool {
    (0..h).contains(&coord.0) && (0..w).contains(&coord.1)
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

    input! {
        
    };
}

/*

*/
