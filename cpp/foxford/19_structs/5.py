n = int(input())
h = sorted(list(map(int, input().split())))

ans = 0
prev = -1000
for i in range(len(h)):
    if h[i] < n:
        continue
    if h[i] - prev >= 3:
        ans+=1
        prev = h[i]

print(ans)