n = int(input())
A = []
for i in range(n):
    A.append(list(map(int, input().split())))
B = [[0] * n for i in range(n)]
for i in range(n):
    for j in range(n):
        B[j][n - i - 1] = A[i][j]
for row in B:
    print(*row)