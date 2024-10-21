n = int(input())
k1 = 0
k2 = 0
k0 = 0
for i in range(n):
    a = int(input())
    if a % 3 == 0:
        k0 += 1
    elif a % 3 == 1:
        k1 += 1
    else:
        k2 += 1
ans = k0 * (k0 - 1) // 2 + k1 * k2
print(ans)