x = int(input())
y = int(input())

ans = 1
i = x
while i < y:
    i += (i * 0.1)
    ans += 1

print(ans)