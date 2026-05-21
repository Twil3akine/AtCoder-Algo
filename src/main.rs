#![allow(dead_code)]
#![allow(unused)]

use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::ops::{Deref, DerefMut};
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

// ローカル実行時(デバッグビルド)だけ eprintln! を実行
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!($($arg)*)
    };
}

// =============================================
// Scanner
// =============================================

struct Scanner<R: BufRead> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    fn with_reader(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace(),
        }
    }

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

macro_rules! input {
    // 終端
    ($(,)?) => {};

    // mut 変数 (複数対応: mut a, b: usize)
    (mut $($var:ident),+ : $t:tt $(, $($rest:tt)*)?) => {
        $( let mut $var = SC.with(|sc| read_value!(sc.borrow_mut(), $t)); )+
        $(input!($($rest)*);)?
    };

    // 通常変数 (複数対応: a, b: usize)
    ($($var:ident),+ : $t:tt $(, $($rest:tt)*)?) => {
        $( let $var = SC.with(|sc| read_value!(sc.borrow_mut(), $t)); )+
        $(input!($($rest)*);)?
    };
}

// =============================================
// wprint! / wprintln! マクロ
// =============================================

macro_rules! wprint {
    ($($arg:tt)*) => {
        WR.with(|wr| write!(wr.borrow_mut(), $($arg)*).unwrap())
    };
}

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

struct Writer<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> Writer<W> {
    fn print<S: std::fmt::Display>(&mut self, s: S) {
        write!(self.writer, "{}", s).unwrap();
    }

    fn println<S: std::fmt::Display>(&mut self, s: S) {
        writeln!(self.writer, "{}", s).unwrap();
    }

    fn print_yes_no(&mut self, cnd: bool) {
        self.println(if cnd { "Yes" } else { "No" });
    }

    fn print_yes(&mut self) {
        self.print_yes_no(true);
    }

    fn print_no(&mut self) {
        self.print_yes_no(false);
    }

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

    fn join_nospace<S: std::fmt::Display, I: IntoIterator<Item = S>>(&mut self, iter: I) {
        self.join(iter, "");
    }

    fn join_whitespace<S: std::fmt::Display, I: IntoIterator<Item = S>>(&mut self, iter: I) {
        self.join(iter, " ");
    }

    fn join_line<S: std::fmt::Display, I: IntoIterator<Item = S>>(&mut self, iter: I) {
        self.join(iter, "\n");
    }
}

impl Writer<std::io::StdoutLock<'static>> {
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

trait FastMath {
    fn fast_pow(self, n: Self) -> Self;
    fn mod_pow(self, n: Self, m: Self) -> Self;
    fn mod_inv(self, m: Self) -> Self;

    fn gcd(self, rhs: Self) -> Self;
    fn lcm(self, rhs: Self) -> Self;
}
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

pub type MaxHeap<T> = BinaryHeap<T>;

#[derive(Debug, Clone)]
pub struct MinHeap<T>(BinaryHeap<Reverse<T>>);
impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    /// 要素の追加
    pub fn push(&mut self, item: T) {
        self.0.push(Reverse(item));
    }

    /// 最小の要素を取り出す
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|Reverse(v)| v)
    }

    /// 最小の要素の参照を返す
    pub fn peek(&mut self) -> Option<&T> {
        self.0.peek().map(|Reverse(v)| v)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// =============================================

struct Xorshift {
    seed: u64,
}
impl Xorshift {
    fn new(seed: u64) -> Self {
        Xorshift {
            seed: if seed == 0 { 88172645463325252 } else { seed },
        }
    }

    fn next(&mut self) -> u64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        self.seed
    }

    // min 以上 max 以下の乱数を返す (usize用)
    fn next_range(&mut self, min: usize, max: usize) -> usize {
        min + (self.next() as usize % (max - min + 1))
    }

    // 0.0 以上 1.0 未満の乱数を返す
    fn next_f64(&mut self) -> f64 {
        self.next() as f64 / u64::MAX as f64
    }
}

// =============================================

struct Timer {
    start: Instant,
}
impl Timer {
    fn new() -> Self {
        Timer {
            start: Instant::now(),
        }
    }

    fn get_times(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

// =============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ModInt<const MOD: i64> {
    val: i64,
}
impl<const MOD: i64> ModInt<MOD> {
    fn new(mut val: i64) -> Self {
        val %= MOD;
        if val < 0 {
            val += MOD;
        }
        Self { val }
    }

    fn val(&self) -> i64 {
        self.val
    }

    fn inv(&self) -> Self {
        self.pow(MOD - 2)
    }

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

type Mod998 = ModInt<998_244_353>;
type Mod107 = ModInt<1_000_000_007>;

// =============================================

trait AlphaExt {
    fn to_idx(self) -> usize;
}
impl AlphaExt for char {
    fn to_idx(self) -> usize {
        (self.to_ascii_lowercase() as u8 - b'a') as usize
    }
}
impl AlphaExt for u8 {
    fn to_idx(self) -> usize {
        (self.to_ascii_lowercase() - b'a') as usize
    }
}

// =============================================

pub trait SortReverse {
    fn sort_reverse(&mut self);
    fn sort_unstable_reverse(&mut self);
}

impl<T: Ord> SortReverse for [T] {
    fn sort_reverse(&mut self) {
        self.sort_by(|a, b| b.cmp(a));
    }

    fn sort_unstable_reverse(&mut self) {
        self.sort_unstable_by(|a, b| b.cmp(a));
    }
}

// =============================================

trait Compress<T> {
    // 座圧後の配列と元の値のタプル
    fn compressed(&self) -> (Vec<usize>, Vec<T>);
}
impl<T: Ord + Clone> Compress<T> for [T] {
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

struct UnionFind {
    parents: Vec<isize>,
    group_count: usize,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            group_count: n,
        }
    }

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

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        (-self.parents[root]) as usize
    }

    fn group_count(&self) -> usize {
        self.group_count
    }
}

// =============================================

/// 汎用セグメント木。
///
/// `op` に結合演算、`e` に単位元を渡して使う。
/// 区間取得は半開区間 `[l, r)` で行う。
///
/// 例:
/// - 和: `op = |x, y| x + y`, `e = 0`
/// - 最大値: `op = max`, `e = 0` など
/// - 最小値: `op = min`, `e = INF` など
struct SegmentTree<T> {
    /// 2つの値をまとめる演算。
    op: fn(T, T) -> T,

    /// 単位元。
    ///
    /// 区間外の値として使われる。
    e: T,

    /// 元の配列の長さ。
    len: usize,

    /// セグメント木内部で使う葉の数。
    ///
    /// `len` 以上の最小の2冪。
    size: usize,

    /// セグメント木の内部配列。
    ///
    /// 0-indexed の完全二分木として管理する。
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
            data: vec![e; 2 * size - 1],
        }
    }

    /// 配列 `ary` からセグメント木を構築する。
    ///
    /// 計算量は `O(N)`。
    fn from(f: fn(T, T) -> T, ary: Vec<T>, e: T) -> Self {
        let n: usize = ary.len().next_power_of_two();
        let mut array: Vec<T> = vec![e; 2 * n - 1];

        for i in 0..ary.len() {
            array[n - 1 + i] = ary[i];
        }

        for i in (0..n - 1).rev() {
            array[i] = f(array[i * 2 + 1], array[i * 2 + 2]);
        }

        Self {
            op: f,
            len: ary.len(),
            size: n,
            data: array,
            e,
        }
    }

    /// `idx` 番目の値を `v` に更新する。
    ///
    /// `idx` は 0-indexed。
    /// 計算量は `O(log N)`。
    fn apply(&mut self, v: T, mut idx: usize) {
        idx += self.size - 1;
        self.data[idx] = v;

        while idx > 0 {
            idx = (idx - 1) / 2;
            self.data[idx] = (self.op)(self.data[idx * 2 + 1], self.data[idx * 2 + 2]);
        }
    }

    /// 半開区間 `[l, r)` の集約値を返す。
    ///
    /// `l`, `r` は 0-indexed。
    /// 計算量は `O(log N)`。
    fn get(&self, l: usize, r: usize) -> T {
        self._get(l, r, 0, 0, self.size)
    }

    /// `get` の内部実装。
    ///
    /// 現在見ているノード `idx` が区間 `[nl, nr)` を表す。
    fn _get(&self, l: usize, r: usize, idx: usize, nl: usize, nr: usize) -> T {
        if nr <= l || r <= nl {
            return self.e;
        }

        if l <= nl && nr <= r {
            return self.data[idx];
        }

        let mid = (nl + nr) / 2;
        let left = self._get(l, r, idx * 2 + 1, nl, mid);
        let right = self._get(l, r, idx * 2 + 2, mid, nr);

        (self.op)(left, right)
    }

    /// `[l, r)` が条件 `pred` を満たす最小の `l` を探す。
    ///
    /// 右端 `r` を固定し、左方向に伸ばしていく。
    /// AtCoder Library の `min_left` と同じ意味。
    ///
    /// `pred(e)` は `true` である必要がある。
    fn min_left<F: Fn(T) -> bool>(&self, r: usize, pred: F) -> usize {
        if r == 0 {
            return 0;
        }

        let mut acc = self.e;

        match self._min_left(r, &pred, &mut acc, 0, 0, self.size) {
            Some(l) => l.min(self.len),
            None => 0,
        }
    }

    /// `min_left` の内部実装。
    ///
    /// 右側の子から先に探索し、条件が壊れる境界を探す。
    fn _min_left<F: Fn(T) -> bool>(
        &self,
        r: usize,
        pred: &F,
        acc: &mut T,
        idx: usize,
        nl: usize,
        nr: usize,
    ) -> Option<usize> {
        if r <= nl {
            return None;
        }

        if nr <= r {
            let next = (self.op)(self.data[idx], *acc);

            if pred(next) {
                *acc = next;
                return None;
            }

            if nr - nl == 1 {
                return Some(nr);
            }
        }

        let mid = (nl + nr) / 2;

        if let Some(ans) = self._min_left(r, pred, acc, idx * 2 + 2, mid, nr) {
            return Some(ans);
        }

        self._min_left(r, pred, acc, idx * 2 + 1, nl, mid)
    }

    /// `[l, r)` が条件 `pred` を満たす最大の `r` を探す。
    ///
    /// 左端 `l` を固定し、右方向に伸ばしていく。
    /// AtCoder Library の `max_right` と同じ意味。
    ///
    /// `pred(e)` は `true` である必要がある。
    fn max_right<F: Fn(T) -> bool>(&self, l: usize, pred: F) -> usize {
        if l == self.len {
            return self.len;
        }

        let mut acc = self.e;

        match self._max_right(l, &pred, &mut acc, 0, 0, self.size) {
            Some(r) => r.min(self.len),
            None => self.len,
        }
    }

    /// `max_right` の内部実装。
    ///
    /// 左側の子から先に探索し、条件が壊れる境界を探す。
    fn _max_right<F: Fn(T) -> bool>(
        &self,
        l: usize,
        pred: &F,
        acc: &mut T,
        idx: usize,
        nl: usize,
        nr: usize,
    ) -> Option<usize> {
        if nr <= l {
            return None;
        }

        if l <= nl {
            let next = (self.op)(*acc, self.data[idx]);

            if pred(next) {
                *acc = next;
                return None;
            }

            if nr - nl == 1 {
                return Some(nl);
            }
        }

        let mid = (nl + nr) / 2;

        if let Some(ans) = self._max_right(l, pred, acc, idx * 2 + 1, nl, mid) {
            return Some(ans);
        }

        self._max_right(l, pred, acc, idx * 2 + 2, mid, nr)
    }
}

/// 区間和用セグメント木。
///
/// `op = +` を使う。
struct SumSegmentTree<T>(SegmentTree<T>);

impl<T: Copy + Add<Output = T>> SumSegmentTree<T> {
    /// 長さ `len` の区間和セグメント木を作る。
    fn new(len: usize, e: T) -> Self {
        Self(SegmentTree::new(|x, y| x + y, len, e))
    }

    /// 配列 `a` から区間和セグメント木を作る。
    fn from(a: Vec<T>, e: T) -> Self {
        Self(SegmentTree::from(|x, y| x + y, a, e))
    }
}
impl<T> Deref for SumSegmentTree<T> {
    type Target = SegmentTree<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for SumSegmentTree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// 区間最大値用セグメント木。
///
/// `op = max` を使う。
struct MaxSegmentTree<T>(SegmentTree<T>);

impl<T: Copy + Ord> MaxSegmentTree<T> {
    /// 長さ `len` の区間最大値セグメント木を作る。
    fn new(len: usize, e: T) -> Self {
        Self(SegmentTree::new(max, len, e))
    }

    /// 配列 `a` から区間最大値セグメント木を作る。
    fn from(a: Vec<T>, e: T) -> Self {
        Self(SegmentTree::from(max, a, e))
    }
}
impl<T> Deref for MaxSegmentTree<T> {
    type Target = SegmentTree<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for MaxSegmentTree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// 区間最小値用セグメント木。
///
/// `op = min` を使う。
struct MinSegmentTree<T>(SegmentTree<T>);

impl<T: Copy + Ord> MinSegmentTree<T> {
    /// 長さ `len` の区間最小値セグメント木を作る。
    fn new(len: usize, e: T) -> Self {
        Self(SegmentTree::new(min, len, e))
    }

    /// 配列 `a` から区間最小値セグメント木を作る。
    fn from(a: Vec<T>, e: T) -> Self {
        Self(SegmentTree::from(min, a, e))
    }
}
impl<T> Deref for MinSegmentTree<T> {
    type Target = SegmentTree<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for MinSegmentTree<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// =============================================

fn is_valid_range(h: usize, w: usize, coord: (usize, usize)) -> bool {
    (0..h).contains(&coord.0) && (0..w).contains(&coord.1)
}

// =============================================

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
        n, q: usize,
        a: [usize; n],
    );

    let mut sgt: MaxSegmentTree<usize> = MaxSegmentTree::from(a, 0);

    for _ in 0..q {
        input!(
            ti: usize,
        );

        match ti {
            1 => {
                input!(
                    xi: usize1,
                    vi: usize
                );

                sgt.apply(vi, xi);
            }
            2 => {
                input!(
                    li: usize1,
                    ri: usize,
                );

                wr.println(sgt.get(li, ri));
            }
            3 => {
                input!(
                    xi: usize1,
                    vi: usize,
                );

                wr.println(
                    if vi == 0 {
                        xi
                    } else {
                        sgt.max_right(xi, |mx| mx < vi)
                    } + 1,
                );
            }
            _ => unreachable!(),
        }
    }
}
