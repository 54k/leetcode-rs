n, k = map(int, input().split())

dp = [0] * (n+1)
dp[0] = 1

for i in range(n+1):
    for j in range(max(i - k, 0), i):
        dp[i] += dp[j]

print(dp[n])