import math

def count_cells(x1, y1, x2, y2):
    # Вычисляем изменения по осям x и y
    dx = abs(x2 - x1)
    dy = abs(y2 - y1)
    
    # Количество клеток = dx + dy - НОД(dx, dy)
    return dx + dy - math.gcd(dx, dy)

# Чтение данных
x1, y1, x2, y2 = map(int, input().split())

# Вывод результата
print(count_cells(x1, y1, x2, y2))
