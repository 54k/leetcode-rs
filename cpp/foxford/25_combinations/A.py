import sys

sys.setrecursionlimit(2**20)

MOD = 2**32

def f(n):
    global memo
    if n in memo:
        return memo[n]
    if n <= 2:
        return 1
    elif n % 2 == 1:
        memo[n] = f(6 * n // 7) % MOD + f(2*n // 3) % MOD
    else:
        memo[n] = f(n-1) % MOD + f(n-3) %MOD
    memo[n] %= MOD
    return memo[n]

n = int(input())
memo = {}
print(f(n) % MOD)


