# n, k = map(int, input().split())
# F = [[[0] * 2 for _ in range(k)]  for _ in range(n + 1)]

# F[1][1][0] = 1
# F[1][1][1] = 1

# for i in range(2, n+1):
#     for j in range(1, k):
#         if j == 1:
#             F[i][j][0] = sum(x[1] for x in F[i-1])
#             F[i][j][1] = sum(x[0] for x in F[i-1])
#         else:
#             F[i][j][0] = F[i-1][j-1][0]
#             F[i][j][1] = F[i-1][j-1][1]

# print(sum([sum(x) for x in F[n]]))

n, k = map(int, input().split())
if n > 0 and k == 2:
    print(2)
    exit(0)

dp = [[0, 0] for i in range(n + 1)]

if n < k:
    print(2 ** n)
    exit(0)

dp[0][1] = 0
dp[0][0] = 0

for i in range(1, k + 1):
    dp[i][1] = 2 ** (i - 1)
    dp[i][0] = 2 ** (i - 1)

for i in range(k, n + 1):
    dp[i][0] = sum([dp[i - j][1] for j in range(1, k)])
    dp[i][1] = sum([dp[i - j][0] for j in range(1, k)])
print(dp[n][0] + dp[n][1])


