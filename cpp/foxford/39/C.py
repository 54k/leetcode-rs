INF = 10 ** 10

n, m = map(int, input().split())

G = [[INF] * n for _ in range(n)]
for _ in range(m):
    s,e,l = map(int, input().split())
    s-=1
    e-=1
    G[s][s] = 0
    G[e][e] = 0
    G[s][e] = l
    G[e][s] = l

for k in range(n):
    for i in range(n):
        for j in range(n):
            G[i][j] = min(G[i][k] + G[j][k], G[i][j])

ans = 0
for i in range(n):
    for j in range(n):
        if G[i][j] < INF and G[i][j] > ans:
            ans = G[i][j]

print(ans)

