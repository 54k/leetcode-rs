from collections import deque

# Направления движения: 0 - вниз, 1 - вправо, 2 - вверх, 3 - влево
DX = [1, 0, -1, 0]
DY = [0, 1, 0, -1]

def bfs(maze, start, end, n, m):
    # Очередь для BFS
    queue = deque()
    visited = set()

    # Добавляем в очередь начальные позиции для всех 4 направлений
    for direction in range(4):
        queue.append((start[0], start[1], direction, 0))
        visited.add((start[0], start[1], direction))

    while queue:
        x, y, direction, distance = queue.popleft()

        if (x, y) == end:
            return distance

        # Движение прямо
        nx, ny = x + DX[direction], y + DY[direction]
        if 0 <= nx < n and 0 <= ny < m and maze[nx][ny] != 'X':
            if (nx, ny, direction) not in visited:
                queue.append((nx, ny, direction, distance + 1))
                visited.add((nx, ny, direction))

        # Правый поворот
        new_direction = (direction + 1) % 4
        if (x, y, new_direction) not in visited:
            queue.append((x, y, new_direction, distance + 1))
            visited.add((x, y, new_direction))

    return -1  # На случай, если выход не найден

def find_start_and_end(maze):
    start = end = None
    for i in range(len(maze)):
        for j in range(len(maze[i])):
            if maze[i][j] == 'S':
                start = (i, j)
            elif maze[i][j] == 'F':
                end = (i, j)
    return start, end

n, m = map(int, input().split())
maze = [input().strip() for _ in range(n)]

start, end = find_start_and_end(maze)
result = bfs(maze, start, end, n, m)
print(result)
