n,m,k=map(int,input().split())
l=[[int(0) for j in range(m)] for i in range(n)]
for i in range(k):
    x,y=map(int,input().split())
    l[x-1][y-1]='*'
for i in range(n):
    for j in range(m):
        if l[i][j]==0:
            for b in ([1,0],[0,1],[-1,0],[0,-1],[1,1],[-1,-1],[-1,1],[1,-1]):
                x=b[0]
                y=b[1]
                if 0<=i+x<n and 0<=y+j<m and l[i+x][j+y]=='*':
                    l[i][j]=l[i][j]+1
for i in l:
    print(*i)