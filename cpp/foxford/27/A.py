n = int(input())

# todo WA
F = [[0] * (n + 1) for _ in range(n + 1)]

for i in range(0, n + 1):
    F[0][i] = 1

for k in range(1, n + 1):
    for i in range(1, k + 1):
        F[k][i] += F[k-i][i-1] + F[k][i-1]
    
    for i in range(k + 1, n + 1):
        F[k][i] = F[k][k]

print(F[n][n])



