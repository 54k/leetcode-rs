n = int(input())
h = list(map(int, input().split()))

dp = [0] * (n+1)

for i in range(5, n+1):
    dp[i] = max(dp[i-1], dp[i-5] + 10 * (h[i-1] + h[i-2] + h[i-3] + h[i-4] + h[i-5]))

print(max(dp))
