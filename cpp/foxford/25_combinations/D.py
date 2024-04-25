import sys

sys.setrecursionlimit(100010)

def gcd(a, b):
    return a if b == 0 else gcd(b, a % b)

n = int(input())
mx = 0
ans = [0, n]

for i in range(1, n):
    g = gcd(i, n - i) 
    if g > mx:
        mx = g
        ans = f'{i} {n-i}'

print(ans)