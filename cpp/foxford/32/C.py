n = int(input())
cost = list(map(int, input().split()))
adj = [[] for _ in range(n)]
for i in range(n):
    d = list(map(int, input().split()))[1:]
    for x in d:
        adj[i].append(x-1)

res = []
vis = [False] * n

def topsort(v):
    vis[v] = True
    for u in adj[v]:
        if not vis[u]:
            topsort(u)
    res.append(v)

topsort(0)

total = 0
for x in res:
    total += cost[x]

print(f"{total} {len(res)}")
print(" ".join(map(lambda x: f"{x+1}", res)))

# from collections import deque, defaultdict

# def find_min_time(n, times, dependencies):
#     # Граф зависимостей и счетчики входящих ребер
#     graph = defaultdict(list)
#     in_degree = [0] * n

#     # Заполнение графа и счетчиков входящих ребер
#     for i in range(n):
#         for dep in dependencies[i]:
#             graph[dep].append(i)
#             in_degree[i] += 1
    
#     # Очередь для топологической сортировки
#     queue = deque()
#     for i in range(n):
#         if in_degree[i] == 0:
#             queue.append(i)
    
#     # Топологическая сортировка и вычисление времени
#     top_order = []
#     min_time = [0] * n
#     while queue:
#         node = queue.popleft()
#         top_order.append(node)
#         for neighbor in graph[node]:
#             min_time[neighbor] = max(min_time[neighbor], min_time[node] + times[node])
#             in_degree[neighbor] -= 1
#             if in_degree[neighbor] == 0:
#                 queue.append(neighbor)
    
#     # Итоговое время для детали 1
#     total_time = min_time[0] + times[0]
    
#     return total_time, top_order

# # Чтение входных данных
# n = int(input())
# times = list(map(int, input().split()))
# dependencies = []

# for i in range(n):
#     line = list(map(int, input().split()))
#     dependencies.append(line[1:] if line[0] > 0 else [])

# # Поиск минимального времени и топологического порядка
# total_time, top_order = find_min_time(n, times, dependencies)

# # Номера деталей для производства в порядке изготовления
# result_order = [x + 1 for x in top_order if x != 0]  # Исключаем деталь 1

# # Вывод результата
# print(total_time, len(result_order))
# print(' '.join(map(str, result_order)))
