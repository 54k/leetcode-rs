m = int(input())
a = list(map(int, input().split()))

n = int(input())
b = list(map(int, input().split()))

dp = [[0] * n for _ in range(m)]
prev = [-1] * m

for i in range(m):
    for j in range(n):
        if a[i] == b[j]:
            dp[i][j] = 1
            for k in range(i):
                for l in range(j):
                    if a[k] == b[l] and a[k] < a[i] and dp[i][j] < dp[k][l] + 1:
                        dp[i][j] = dp[k][l] + 1
                        prev[i] = k


b_i, b_j = 0, 0
for i in range(m):
    for j in range(n):
        if dp[i][j] > dp[b_i][b_j]:
            b_i = i
            b_j = j
pos = b_i

answer = []
while pos != -1:
    answer.append(a[pos])
    pos = prev[pos]

print(len(answer))
print(" ".join(map(str, answer[::-1])))