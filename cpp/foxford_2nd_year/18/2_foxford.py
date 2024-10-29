N = int(input())
A = list(map(int, input().split()))
for i in range(N):
    A[i] = (A[i], i + 1)
sum = 0
A.sort(reverse=True)
for i in range(N):
    sum += (i + 1) * A[i][0]
print(sum)
for elem in A:
    print(elem[1], end=' ')