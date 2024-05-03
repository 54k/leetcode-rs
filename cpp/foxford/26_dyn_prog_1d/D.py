n = int(input())

dp = [0] * (31)
dp[1] = 1
dp[2] = 1
dp[3] = 2

for i in range(4, 31):
    dp[i] = dp[i-1] + dp[i-2] + dp[i-3]

print(dp[n])