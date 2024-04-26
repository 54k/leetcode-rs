import sys

sys.setrecursionlimit(100010)

def gcd(a, b):
    return a if b == 0 else gcd(b, a % b)

n = int(input())
mx = 0
ans = [0, n]

for i in range(2, n // 2 + 1):
    g = gcd(i, n) 
    if g > mx:
        mx = g
        ans = f'{i} {n-i}'

print(ans)