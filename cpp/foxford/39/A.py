n, s, f = map(int, input().split())

s -= 1
f -= 1

INF = 10 ** 10

visited = [False] * n
graph = [[0] * n for _ in range(n)]

for i in range(n):
    graph[i] = list(map(int, input().split()))

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

