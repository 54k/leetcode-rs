from math import *
 
a, k, b, m, x = map(int, input().split())
l = 0
r = 100000000000000000000
while r - l != 1:
    mid = (l + r) // 2
    if mid * (a + b) - (mid // k) * a - (mid // m) * b < x:
        l = mid
    else:
        r = mid
print(r)