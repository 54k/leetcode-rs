import sys
n, m, k = map(int, input().split())
INF = 10 ** 10
A = [[-INF for i in range(n)] for j in range(n)]
R = [[0 for i in range(n)] for j in range(n)]
Prev = [[None for i in range(n)] for j in range(n)]
# Заполняем матрицу весов
for i in range(1, m + 1):
    u, v, w = map(int, input().split())
    u -= 1
    v -= 1
    if w > A[u][v]:
        A[u][v] = w
        R[u][v] = i
        Prev[u][v] = u
# Запускаем Флойда
for k in range(n):
    for i in range(n):
        for j in range(n):
            if A[i][k] > -INF and A[k][j] > -INF and A[i][k] + A[k][j] > A[i][j]:
                A[i][j] = A[i][k] + A[k][j]
                Prev[i][j] = Prev[k][j]
# Считываем маршрут Уотерса
Path = list(map(int, input().split()))
k = len(Path)
for i in range(k):
    Path[i] -= 1
# Проверяем ответ на циклы отрицательного веса
for i in range(1, k):
    u = Path[i - 1]
    v = Path[i]
    for i in range(n):
        if A[u][i] > -INF and A[i][i] > 0 and A[i][v] > -INF:
            print("infinitely kind")
            sys.exit(0)
Ans = []
# Восстанавливаем путь с конца
for i in range(k - 1, 0, -1):
    # Восстанавливаем перелеты между двумя соседними концертами
    u = Path[i - 1]
    v = Path[i]
    while u != v:
        Ans.append(R[Prev[u][v]][v])
        v = Prev[u][v]
print(len(Ans))
print(" ".join(map(str, Ans[::-1])))