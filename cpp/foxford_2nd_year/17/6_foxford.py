def f(l):    
    pos = p[0]
    ans = 1
    for i in range(1, n):
        if p[i] - pos >= l:
            ans += 1
            pos = p[i]
    return ans
 
n, k = [int(i) for i in input().split()]
p = [int(i) for i in input().split()]
l = 0
r = 10 ** 10
while l != r - 1:
    m = (l + r) // 2
    if f(m) >= k:
        l = m
    else:
        r = m
print(l)