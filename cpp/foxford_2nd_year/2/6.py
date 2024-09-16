n = int(input())
ans = 0
while n > 0:
    n -= 1
    ans += 1 if int(input()) == 0 else 0
print(ans)