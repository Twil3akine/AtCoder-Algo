#![allow(dead_code)]
#![allow(unused)]

use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
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

struct SegmentTree<T> {
    f: fn(T, T) -> T,
    n: usize,
    array: Vec<T>,
    dv: T,
}
impl<T: Copy> SegmentTree<T> {
    fn new(f: fn(T, T) -> T, n: usize, dv: T) -> Self {
        Self {
            f,
            n,
            array: vec![dv; 2 * n - 1],
            dv,
        }
    }

    fn from(f: fn(T, T) -> T, mut ary: Vec<T>, dv: T) -> Self {
        let n: usize = ary.len().next_power_of_two();
        let mut array: Vec<T> = vec![dv; 2 * n - 1];

        for i in 0..ary.len() {
            array[n - 1 + i] = ary[i];
        }

        for i in (0..n - 1).rev() {
            array[i] = f(array[i * 2 + 1], array[i * 2 + 2]);
        }

        Self { f, n, array, dv }
    }

    fn apply(&mut self, v: T, mut idx: usize) {
        idx += self.n - 1;
        self.array[idx] = v;

        while idx > 0 {
            idx = (idx - 1) / 2;
            self.array[idx] = (self.f)(self.array[idx * 2 + 1], self.array[idx * 2 + 2]);
        }
    }

    fn get(&self, l: usize, r: usize) -> T {
        self._get(l, r, 0, 0, self.n)
    }

    fn _get(&self, l: usize, r: usize, idx: usize, nl: usize, nr: usize) -> T {
        if nr <= l || r <= nl {
            return self.dv;
        }

        if l <= nl && nr <= r {
            return self.array[idx];
        }

        let mid = (nl + nr) / 2;
        let left = self._get(l, r, idx * 2 + 1, nl, mid);
        let right = self._get(l, r, idx * 2 + 2, mid, nr);

        (self.f)(left, right)
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
        
    );
}
