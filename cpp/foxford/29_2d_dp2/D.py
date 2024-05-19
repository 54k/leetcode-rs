n, a, k, b, m = map(int, input().split())

INF = float('inf')

F = [INF] * (n+1)
F[0] = -INF
F[1] = a

for i in range(n-1):
    a = (a * k + b) % m
    left = 0
    right = n

    while left + 1 < right:
        mid = (left + right) // 2
        if F[mid] >= a:
            right = mid
        else:
            left = mid

    F[right] = a

for i in range(n-1, 0, -1):
    if F[i] != INF:
        print(i)
        break

# print(F)
