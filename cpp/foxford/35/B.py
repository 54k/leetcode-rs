import sys 
N, M, K, S, F = map(int, input().split()) 
INF = 10 ** 10 
dist = [INF] * (N + 1) 
dist[S] = 0 
Edges = [] 
for i in range(M): 
    Edges.append(list(map(int, input().split()))) 
for i in range(K): 
    dist_new = dist[:] 
    for start, end, price in Edges: 
        if dist[start] + price < dist_new[end]: 
            dist_new[end] = dist[start] + price 
    dist = dist_new 
if dist[F] == INF: 
    print(-1) 
else: 
    print(dist[F])