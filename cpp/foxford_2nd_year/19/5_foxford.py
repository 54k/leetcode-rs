def InsertionSort(A):
    for i in range(1, len(A)):
        new_elem = A[i]
        j = i - 1
        while j >= 0 and A[j] > new_elem:
            A[j + 1] = A[j]
            j -= 1
        A[j + 1] = new_elem
min_size = int(input())
A = list(map(int, input().split()))
InsertionSort(A)
ans = 0
for size in A:
    if size >= min_size:
        min_size = size + 3
        ans += 1
print(ans)