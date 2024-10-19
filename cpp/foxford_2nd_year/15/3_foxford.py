n = int(input())
A = list(map(int, input().split()))
ans = 0
for i in range(1, n):
    if A[i] != A[i - 1] + 1:
        ans += 1
print(ans)