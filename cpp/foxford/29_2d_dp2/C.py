n = int(input())
a = list(map(int, input().split()))

dp = [0] * n
p = [0] * n

ans = 0
ansp = 0


for i in range(n):
    dp[i] = 1
    p[i] = i
    for j in range(i):
        if a[j]  < a[i]:
            if dp[j] + 1 > dp[i]:
                p[i] = j
                dp[i] = dp[j] + 1
            if dp[i] > ans:
                ans = dp[i]
                ansp = i

path = []
pp = ansp
while pp != p[pp]:
    path.append(a[pp])
    pp = p[pp]

path.append(a[pp])

print(" ".join(list(map(str, reversed(path)))))
