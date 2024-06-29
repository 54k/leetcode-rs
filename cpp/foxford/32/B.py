# # n, m = map(int, input().split())
# # adj = [[] for _ in range(n)]
# # for _ in range(m):
# #     f, t = map(int, input().split())
# #     adj[f-1].append(t-1)
# #     adj[t-1].append(f-1)

# # vis = [-1] * n
# # cmp = [[] for _ in range(n)]
# # id = 0

# # for i in range(n):
# #     if vis[i] == -1:
# #         vis[i] = id
# #         st = [i]
# #         while len(st):
# #             v = st.pop()
# #             cmp[id].append(f"{v+1}")
# #             for u in adj[v]:
# #                 if vis[u] == -1:
# #                     vis[u] = id
# #                     st.append(u)
# #         id+=1

# # print(id)
# # for cc in cmp:
# #     if cc: 
# #         print(len(cc))
# #         print(" ".join(cc))

# def dfs(v, visited, graph, component):
#     stack = [v]
#     while stack:
#         node = stack.pop()
#         if not visited[node]:
#             visited[node] = True
#             component.append(node)
#             for neighbor in graph[node]:
#                 if not visited[neighbor]:
#                     stack.append(neighbor)

# def find_connected_components(n, edges):
#     # Инициализация графа
#     graph = [[] for _ in range(n)]
#     for u, v in edges:
#         graph[u-1].append(v-1)
#         graph[v-1].append(u-1)

#     visited = [False] * n
#     components = []

#     for i in range(n):
#         if not visited[i]:
#             component = []
#             dfs(i, visited, graph, component)
#             components.append(component)

#     return components

# # Чтение входных данных
# n, m = map(int, input().split())
# edges = [tuple(map(int, input().split())) for _ in range(m)]

# # Нахождение компонент связности
# components = find_connected_components(n, edges)

# # Вывод результата
# print(len(components))
# for component in components:
#     print(len(component))
#     print(' '.join(str(v + 1) for v in component))

def is_bipartite(n, edges):
    graph = [[] for _ in range(n)]
    for u, v in edges:
        graph[u-1].append(v-1)
        graph[v-1].append(u-1)
    
    color = [-1] * n

    def dfs(v, c):
        color[v] = c
        for neighbor in graph[v]:
            if color[neighbor] == -1:
                if not dfs(neighbor, 1 - c):
                    return False
            elif color[neighbor] == color[v]:
                return False
        return True

    for i in range(n):
        if color[i] == -1:
            if not dfs(i, 0):
                return False, []

    table1 = [i + 1 for i in range(n) if color[i] == 0]
    return True, table1

# Чтение входных данных
n, m = map(int, input().split())
edges = [tuple(map(int, input().split())) for _ in range(m)]

# Проверка двудольности графа и вывод результата
is_bipartite_result, table1 = is_bipartite(n, edges)
if is_bipartite_result:
    print("YES")
    print(' '.join(map(str, table1)))
else:
    print("NO")
