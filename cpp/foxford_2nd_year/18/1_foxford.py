n = int(input())
A = list(map(int, input().split()))
max_num = 0
for i in range(n):
    if A[i] > A[max_num]:
        max_num = i
A[0], A[max_num] = A[max_num], A[0]
for elem in A:
    print(elem, end = ' ')