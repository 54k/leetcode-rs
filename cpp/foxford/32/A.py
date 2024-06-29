n, m = map(int, input().split())
adj = [[] for _ in range(n)]
for _ in range(m):
    f, t = map(int, input().split())
    adj[f-1].append(t-1)
    adj[t-1].append(f-1)

color = [-1] * n
def dfs(v):
    for u in adj[v]:
        if color[u] == -1:
            color[u] = 1 - color[v]
            if not dfs(u):
                return False
        elif color[u] == color[v]:
                return False
    return True

for i in range(n):
    if color[i] == -1:
        color[i] = 0
        if not dfs(i):
            print("NO")
            exit(0)

ll = []
for i, v in enumerate(color):
    if v == 0:
        ll.append(f"{i+1}")

print("YES")
print(" ".join(ll))
