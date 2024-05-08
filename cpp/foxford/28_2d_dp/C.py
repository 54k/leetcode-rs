n = int(input())

F = [[0] * 11 for _ in range(n+1)]
F[1] = [1] * 11
F[1][0] = 0
F[1][10] = 0

for i in range(2, n + 1):
    for digit in range(0, 10):
        F[i][digit] = F[i-1][digit-1] + F[i-1][digit+1] + F[i-1][digit]

print(sum(F[n]))

