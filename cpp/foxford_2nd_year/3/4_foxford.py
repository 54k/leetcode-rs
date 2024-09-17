n, k, m = map(int, input().split())
ans = 0
while n // k > 0 and k >= m:
    ans = ans + (n // k) * (k // m)
    n = n % k + (k % m) * (n // k)
print(ans)