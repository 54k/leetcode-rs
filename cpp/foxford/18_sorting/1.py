input()
arr = [x for x in map(int, input().split())]
me = -1
mx = -1
for (i, e) in enumerate(arr):
    if e > me:
        me = e
        mx = i

arr[0], arr[mx] = arr[mx], arr[0]
print(" ".join([str(x) for x in arr]))