import functools as ft
import itertools as it
import math as mt
import sys

# =================================


def main():
    


# =================================


# 最大再帰回数の設定
sys.setrecursionlimit(10**7)
# 巨大な整数の文字列変換の設定
sys.set_int_max_str_digits(0)

try:
    import pypyjit

    pypyjit.set_param("max_unroll_recursion=-1")
except ImportError:
    pass

input = sys.stdin.readline
_write = sys.stdout.write


def print(*args, sep=" ", end="\n"):
    _write(sep.join(map(str, args)) + end)


if __name__ == "__main__":
    main()
