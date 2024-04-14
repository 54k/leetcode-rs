a,k,m,b,x = map(int, input().split())
lo, hi = 0, 10e18

while lo + 1 != hi:
    t = lo + (hi - lo) // 2
    if t * (a + b) - (t // k) * a - (t // m) * b < x:
        lo = t
    else:
        hi = t
    
print(int(hi))    
