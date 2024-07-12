import heapq
import math

def calculate_distance(x1, y1, x2, y2):
    return math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2)

def prim_mst(n, cities):
    adj = [[] for _ in range(n)]

    # Создание графа
    for i in range(n):
        for j in range(i + 1, n):
            dist = calculate_distance(cities[i][0], cities[i][1], cities[j][0], cities[j][1])
            adj[i].append((dist, j))
            adj[j].append((dist, i))

    # Алгоритм Прима
    total_cost = 0.0
    visited = [False] * n
    min_heap = [(0, 0)]  # (cost, vertex)

    while min_heap:
        cost, u = heapq.heappop(min_heap)
        if visited[u]:
            continue

        total_cost += cost
        visited[u] = True

        for weight, v in adj[u]:
            if not visited[v]:
                heapq.heappush(min_heap, (weight, v))

    return total_cost

n = int(input())
cities = []
i = 0
while i < n:
    try:
        a, b = map(int, input().split())
    except Exception as e:
        continue
    cities.append((a, b))
    i += 1

result = prim_mst(n, cities)
print(f"{result:.6f}")


