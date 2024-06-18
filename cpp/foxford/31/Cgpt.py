def build_graph(n, k):
    # Создаем пустую матрицу смежности
    adjacency_matrix = [[0] * n for _ in range(n)]
    
    # Если k >= n, то невозможно построить такой граф
    if k > n:
        raise ValueError("k cannot be greater than n")
    
    # Разбиваем вершины на k компонент
    current_vertex = 0
    for i in range(k):
        # Количество вершин в текущей компоненте
        component_size = (n - current_vertex) // (k - i)
        for j in range(component_size - 1):
            adjacency_matrix[current_vertex + j][current_vertex + j + 1] = 1
            adjacency_matrix[current_vertex + j + 1][current_vertex + j] = 1
        
        # Переходим к следующей компоненте
        current_vertex += component_size

    return adjacency_matrix

def print_adjacency_matrix(matrix):
    for row in matrix:
        print(" ".join(map(str, row)))

# Чтение входных данных
n, k = map(int, input().split())

# Построение графа и вывод матрицы смежности
adjacency_matrix = build_graph(n, k)
print_adjacency_matrix(adjacency_matrix)
