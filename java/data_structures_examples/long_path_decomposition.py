# https://leetcode.com/discuss/study-guide/4299594/Binary-Lifting-Technique-A-Beginners-Guide

# Функция, которая строит long path decomposition для дерева
def build_long_path_decomposition(tree, root):
    # Список путей
    paths = []
    # Словарь, который хранит номер пути для каждой вершины
    path_of = {}
    # Словарь, который хранит позицию вершины на пути
    pos_of = {}
    # Стек для обхода в глубину
    stack = [(root, -1, 0)]
    # Цикл, пока стек не пуст
    while stack:
        # Достаем вершину, ее родителя и глубину из стека
        v, p, d = stack.pop()
        # Если у вершины нет родителя или родитель на другом пути
        if p == -1 or path_of[p] != path_of.get(v, -1):
            # Создаем новый путь
            path = [v]
            paths.append(path)
            # Запоминаем номер пути и позицию для вершины
            path_of[v] = len(paths) - 1
            pos_of[v] = 0
        else:
            # Добавляем вершину в существующий путь
            path = paths[path_of[v]]
            path.append(v)
            # Запоминаем позицию для вершины
            pos_of[v] = len(path) - 1
        # Добавляем детей вершины в стек
        for u in tree[v]:
            if u != p:
                stack.append((u, v, d + 1))
    # Возвращаем список путей, словари номеров путей и позиций
    return paths, path_of, pos_of

# Функция, которая строит двоичные подъемы для дерева
def build_binary_lifts(tree, root):
    # Словарь, который хранит глубину каждой вершины
    depth_of = {}
    # Словарь, который хранит список предков для каждой вершины
    ancestors_of = {}
    # Стек для обхода в глубину
    stack = [(root, -1, 0)]
    # Цикл, пока стек не пуст
    while stack:
        # Достаем вершину, ее родителя и глубину из стека
        v, p, d = stack.pop()
        # Запоминаем глубину для вершины
        depth_of[v] = d
        # Создаем список предков для вершины
        ancestors = [p]
        # Добавляем в список предков по степеням двойки
        i = 0
        while ancestors[i] != -1:
            ancestors.append(ancestors_of[ancestors[i]][i])
            i += 1
        # Запоминаем список предков для вершины
        ancestors_of[v] = ancestors
        # Добавляем детей вершины в стек
        for u in tree[v]:
            if u != p:
                stack.append((u, v, d + 1))
    # Возвращаем словари глубин и предков
    return depth_of, ancestors_of

# Функция, которая отвечает на запрос level ancestor
def level_ancestor(tree, root, v, k):
    # Строим long path decomposition для дерева
    paths, path_of, pos_of = build_long_path_decomposition(tree, root)
    # Строим двоичные подъемы для дерева
    depth_of, ancestors_of = build_binary_lifts(tree, root)
    # Находим глубину вершины v
    d = depth_of[v]
    # Находим глубину, на которую нужно подняться
    d = d - k
    # Если глубина отрицательная, то ответа не существует
    if d < 0:
        return None
    # Находим максимальный прыжок вверх по двоичным подъемам
    i = 0
    while ancestors_of[v][i] != -1 and depth_of[ancestors_of[v][i]] > d:
        i += 1
    # Прыгаем вверх на этот прыжок
    v = ancestors_of[v][i - 1]
    # Находим, сколько осталось подняться
    i = d - depth_of[v]
    # Находим путь, в который входит вершина v
    p = path_of[v]
    # Находим позицию вершины v на этом пути
    j = pos_of[v]
    # Находим вершину на этом пути, которая находится на нужной глубине
    u = paths[p][j - i]
    # Возвращаем эту вершину
    return u
