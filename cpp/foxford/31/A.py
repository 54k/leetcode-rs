n = int(input())
mat = [[0] * n for _ in range(n)]
for i in range(n):
    adj = list(map(int, input().split()))
    if not adj[0]:
        continue
    for v in adj[1:]:
        mat[i][v-1] = 1

for r in mat:
    print(" ".join(map(str, r)))