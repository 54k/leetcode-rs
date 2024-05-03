x = int(input())

dp = [float('inf')] * (x+1)
dp[1] = 0

for i in range(2, x+1):
    if i % 2 == 0:
        dp[i] = min(dp[i//2]+1, dp[i])
    if i % 3 == 0:
        dp[i] = min(dp[i//3]+1, dp[i])
    dp[i] = min(dp[i-1]+1, dp[i])

print(dp[x])

path = []
i = x
while i != 1:
    path.append(i)
    next = i-1
    if i % 2 == 0:
        if dp[i//2] < dp[next]:
            next = i // 2
    if i % 3 == 0:
        if dp[i//3] < dp[next]:
            next = i // 3
    i = next

path.append(1)
for e in reversed(path):
    print(e, end = " ")




