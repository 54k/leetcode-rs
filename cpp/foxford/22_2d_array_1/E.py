n,m=map(int,input().split())
mx = 0
a = []
for i in range(n):
    for e in map(int, input().split()):
        mx = max(e, mx)
        a.append(e)

cnt = 0
for i in range(n*m):
    if a[i] * 2 < mx: 
        cnt+=1

print(cnt)

