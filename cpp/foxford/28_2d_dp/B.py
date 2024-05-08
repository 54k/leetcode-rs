MOD = int(1e9+7)

n = int(input())
# F = [[0] * (n+2) for _ in range(2 * n + 1)]
# F[0][0] = 1

# for k in range(1, n*2+1):
#     for b in range(0, n+1):
#         F[k][b] = ((F[k-1][b-1] % MOD) + (F[k-1][b+1] % MOD)) % MOD

# print(F[2*n][0])


C = [0] * (n + 1)
C[0] = 1

for k in range(1, n + 1):
    for i in range(0, k):
        C[k] += ((C[i] % MOD) * (C[k - 1 - i] % MOD)) % MOD
        C[k] %= MOD

print(C[n])
