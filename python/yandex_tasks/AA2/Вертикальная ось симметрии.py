"""
Дан массив точек с целочисленными координатами (x, y).
Определить, существует ли вертикальная прямая,
делящая точки на 2 симметричных относительно этой прямой множества.
Note: Для удобства точку можно представлять не как массив [x, y], а как объект {x, y}
"""

from collections import defaultdict
from pprint import pprint


def is_vert_sym(points) -> bool:
    print(points)
    if len(points) == 0:
        return True

    left_bound = min(points, key=lambda p: p[0])[0]
    right_bound = max(points, key=lambda p: p[0])[0]

    deltas = defaultdict(int)  # по умолчанию 0 будет

    for p in points:
        left_dist = p[0] - left_bound
        right_dist = right_bound - p[0]

        if left_dist < right_dist:
            deltas[(left_dist, p[1])] += 1  # tuple is a hashable type
        elif left_dist > right_dist:
            deltas[(right_dist, p[1])] -= 1

        # если расстояние равно то ничего не делаем (точка симметрична в себя)

    for d in deltas.values():
        if d != 0:  # если для каждого игрека нашлась пара, то множество симметрично
            return False

    return True


print(is_vert_sym([(0, 0), (0, 0), (1, 1), (2, 2), (3, 1), (4, 0), (4, 0)]))  # True
print(is_vert_sym([(0, 0), (0, 0), (1, 1), (2, 2), (3, 1), (4, 0)]))  # False
print(is_vert_sym([]))  # True
print(is_vert_sym([(0, 0)]))  # True
print(is_vert_sym([(0, 0), (10, 0)]))  # True
print(is_vert_sym([(0, 0), (11, 1)]))  # False
print(is_vert_sym([(0, 0), (1, 0), (3, 0)]))  # False
