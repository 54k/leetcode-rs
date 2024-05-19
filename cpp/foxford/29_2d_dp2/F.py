s1, s2 = input(), input()
m, n = len(s1), len(s2)
INF = float('inf')
dp = [[INF] * (n+1) for _ in range(m+1)]

for i in range(m+1):
    dp[i][0] = i
    for j in range(n+1):
        dp[0][j] = j
        if i == 0 or j == 0:
            continue
        if s1[i-1] == s2[j-1]:
            dp[i][j] = dp[i-1][j-1]
        else:
            a, b, c = dp[i-1][j], dp[i][j-1], dp[i-1][j-1]
            dp[i][j] = min(a, b, c) + 1

print(dp[m][n])
