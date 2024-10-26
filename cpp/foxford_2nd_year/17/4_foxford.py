n, r = map(int, input().split())
A = list(map(int, input().split()))
A.sort()
i = 0
j = 0
ans = 0
for i in range(len(A)):
    while j < len(A) and A[j] - A[i] <= r:
        j+=1
    if (j < n) or (j == n and A[n - 1] - A[i] <= r):
        ans += j - 1 - i   
print(ans)