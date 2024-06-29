# n, m = map(int, input().split())
# adj = [[0] * n for _ in range(n)]
# for _ in range(m):
#     f, t = map(int, input().split())
#     adj[f-1][t-1] = 1

# for i in range(n-1):
#     for j in range(i+1, n):
#         if not adj[i][j] or not adj[j][i]:
#             print("NO")
#             exit(0)

# print("YES")

def is_semi_complete(n, m, edges):
    # Инициализация матрицы смежности
    adj_matrix = [[False] * n for _ in range(n)]
    
    # Заполнение матрицы смежности
    for u, v in edges:
        adj_matrix[u-1][v-1] = True
    
    # Проверка условий полуполного графа
    for i in range(n):
        for j in range(n):
            if i != j and not (adj_matrix[i][j] or adj_matrix[j][i]):
                return "NO"
    
    return "YES"

# Чтение входных данных
n, m = map(int, input().split())
edges = [tuple(map(int, input().split())) for _ in range(m)]

# Вывод результата
print(is_semi_complete(n, m, edges))
