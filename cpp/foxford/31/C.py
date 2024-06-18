n, k = map(int, input().split())
mat = [[0] * n for _ in range(n)]

for i in range(n-k+1):
    mat[i+1][i] = 1
    mat[i][i+1] = 1

for r in mat:
    print(" ".join(map(str, r)))
