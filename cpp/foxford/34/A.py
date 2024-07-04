# n, m = map(int, input().split())
# a, b = map(int, input().split())

# adj = [[] for _ in range(n)]

# for _ in range(m):
#     f, t = map(int, input().split())
#     adj[f-1].append(t-1)
#     adj[t-1].append(f-1)

# used = [False] * n
# par = [-1] * n
# queue = [a-1]
# used[a-1] = True

# while len(queue) > 0:
#     v = queue.pop()

#     for u in adj[v]:
#         if not used[u]:
#             used[u] = True
#             par[u] = v
#             queue.append(u)

# if par[b-1] == -1:
#     print(-1)
#     exit(0)

# path = []
# j = b-1
# while j != -1:
#     path.append(j+1)
#     j = par[j]

# print(len(path))
# print(" ".join(map(str, reversed(path))))



from collections import deque, defaultdict

def bfs_shortest_path(edges, start, end):
    graph = defaultdict(list)
    for u, v in edges:
        graph[u].append(v)
        graph[v].append(u)

    queue = deque([(start, [start])])
    visited = set([start])
    
    while queue:
        current, path = queue.popleft()
        if current == end:
            return len(path) - 1, path
        for neighbor in graph[current]:
            if neighbor not in visited:
                visited.add(neighbor)
                queue.append((neighbor, path + [neighbor]))
                
    return -1

n, m = map(int, input().split())
start, end = map(int, input().split())

edges = []
for _ in range(m):
    u, v = map(int, input().split())
    edges.append((u, v))

result = bfs_shortest_path(edges, start, end)

if result == -1:
    print(-1)
else:
    length, path = result
    print(length)
    print(" ".join(map(str, path)))
