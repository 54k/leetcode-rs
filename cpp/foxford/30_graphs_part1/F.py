n, m = map(int, input().split())
adj = [[0] * (n) for _ in range(n)]
for _ in range(m):
    v, u = map(int, input().split())
    adj[v-1][u-1] = 1
    adj[u-1][v-1] = 1

for i in range(n):
    for j in range(n):
        for k in range(n):
            if i != j and i != k and j != k:
                if adj[i][j] and adj[i][k] and adj[k][j]:
                    print("YES")
                    exit(0)

print("NO")