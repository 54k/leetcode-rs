n = int(input())
A = []
inf = -10**9
# заполняем список 
A.append([inf]*(n+2))
for i in range(n):
    A.append([inf]+list(map(int,input().split()))+[inf])
A.append([inf]*(n+2))
x, y = n, 1
ans = A[x][y]
for i in range(2*n-2):
    if A[x-1][y] > A[x][y+1]:
        x -= 1
    else:
        y += 1
    ans += A[x][y]
print(ans)