INF = 10 ** 10

n = int(input())
graph = [[-1] * n for _ in range(n)]
cost = list(map(int, input().split()))

m = int(input())
for _ in range(m):
    a, b = map(int, input().split())
    a -= 1
    b -= 1
    graph[a][b] = cost[a]
    graph[b][a] = cost[b]

s, f = 0, n-1

visited = [False] * n

min_vertex = s
min_dist = 0
dist = [INF] * n
dist[s] = 0

while min_dist < INF:
    i = min_vertex
    visited[i] = True
    for j in range(n):
        if graph[i][j] == -1:
            continue
        if graph[i][j] + dist[i] < dist[j]:
            dist[j] = dist[i] + graph[i][j]

    min_dist = INF
    for j in range(n):
        if not visited[j] and dist[j] < min_dist:
            min_dist = dist[j]
            min_vertex = j

if dist[f] == INF:
    print("-1")
else:
    print(dist[f])


