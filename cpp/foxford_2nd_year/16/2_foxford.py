n = int(input())
x = 0
for i in range(n):
    a = int(input())
    if a % 2 == 0:
        x += 1
y = n - x
ans = x * (x - 1) // 2 + y * (y - 1) // 2
print(ans)