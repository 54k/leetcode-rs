n = int(input())
mat = []
for _ in range(n):
    row = list(map(int, input().split()))
    mat.append(row)

vis = [0] * n
vis[0] = 1
queue = [0] 
sz = 0
while len(queue):
    v = queue.pop()
    sz += 1
    for i in range(n):
        if mat[v][i] and not vis[i]:
            vis[i] = 1
            queue.append(i)

print("YES" if sz == n else "NO")
