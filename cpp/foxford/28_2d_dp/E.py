n, m = map(int, input().split())
w = list(map(int, input().split()))
c = list(map(int, input().split()))

dp = [[0] * (m+1) for _ in range(n+1)]

for i in range(1, n+1):
    for j in range(0, m+1):
        dp[i][j] = dp[i-1][j]
        if j >= w[i-1]:
            dp[i][j] = max(dp[i][j], dp[i-1][j-w[i-1]] + c[i-1])

k = m
for i in range(n, 0, -1):
    if dp[i][k] != dp[i-1][k]:
        print(i)
        k -= w[i-1]