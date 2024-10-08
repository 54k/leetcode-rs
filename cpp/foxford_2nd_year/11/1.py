n = int(input())
tmp3 = 0
for i in range(n):
    a = int(input())
    if a % 3 == 0:
        tmp3 += 1
tmp_ne3 = n - tmp3
ans = tmp3 * (tmp3 - 1) // 2 + tmp3 * tmp_ne3
print(ans)