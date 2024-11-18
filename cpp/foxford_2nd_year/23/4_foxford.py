n, m = map(int, input().split())
A = [[j * n + i + 1 for j in range(m)] for i in range(n)]
for row in A:
    print(*row)