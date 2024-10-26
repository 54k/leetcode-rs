n,k = map(int,input().split())
a = [int(i) for i in input().split()]
b = [int(j) for j in input().split()]
for i in range(len(b)):
    l = 0
    r = n
    while (r - l) > 1:
        m = (l + r) // 2
        if a[m] > b[i]:
            r = m
        else:
            l = m
    if r == n:
        print (a[r - 1])
    elif a[r] - b[i] < b[i] - a[l]:
        print (a[r])
    else:
        print (a[l])