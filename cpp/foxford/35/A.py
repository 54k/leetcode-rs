import sys 
N, M = map(int, input().split()) 
INF = 30000 
D = [INF] * (N + 1) 
D[1] = 0 
Edges = [] 
for i in range(M): 
    Edges.append(list(map(int, input().split()))) 
for i in range(N - 1): 
    for start, end, weight in Edges: 
        if D[start] != INF and D[start] + weight < D[end]: 
            D[end] = D[start] + weight 
print(" ".join(map(str, D[1:])))