n, a, k, b, m = map(int, input().split())

INF = float('inf')

F = [INF] * (n+1)
F[0] = -INF
F[1] = a

pos = [-1] * n
prev = [-1] * n
ans = 1

arr = [a]
pos[1] = 0

for i in range(1, n):
    a = (a * k + b) % m
    arr.append(a)
    left = 0
    right = n

    while left + 1 < right:
        mid = (left + right) // 2
        if F[mid] >= a:
            right = mid
        else:
            left = mid

    F[right] = a
    pos[right] = i
    prev[i] = pos[right-1]
    ans = max(ans, right)

answer = []
p = pos[ans]
while p != -1:
    answer.append(arr[p])
    p = prev[p]

print(" ".join(list(map(str, answer[::-1]))))

