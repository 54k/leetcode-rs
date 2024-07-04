N = int(input())
price = list(map(int, input().split()))
INF = 10 ** 10
a = [[INF] * (3 * N) for i in range(3 * N)]
for i in range(N):
    a[3 * i][3 * i + 1] = price[i]
    a[3 * i + 1][3 * i + 2] = price[i]
M = int(input())
for i in range(M):
    u, v = map(int, input().split())
    u -= 1
    v -= 1
    a[3 * u + 1][3 * v] = 0
    a[3 * u + 2][3 * v + 1] = 0
    a[3 * v + 1][3 * u] = 0
    a[3 * v + 2][3 * u + 1] = 0
INF = 10 ** 10
dist = [INF] * (3 * N)
dist[0] = 0
used = [False] * (3 * N)
while True:
    min_dist = INF
    for i in range(3 * N):
        if not used[i] and dist[i] < min_dist:
            min_dist = dist[i]
            min_vertex = i
    if min_dist == INF:
        break
    i = min_vertex
    used[i] = True
    for j in range(3 * N):
        if dist[i] + a[i][j] < dist[j]:
            dist[j] = dist[i] + a[i][j]
if dist[3 * (N - 1)] == INF:
    print(-1)
else:
    print(dist[3 * (N - 1)])