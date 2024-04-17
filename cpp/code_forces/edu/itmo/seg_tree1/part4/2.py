class SegmentTree:
    def __init__(self, arr, mod):
        self.n = len(arr)
        self.mod = mod
        self.tree = [0] * (4 * self.n)
        self.lazy = [None] * (4 * self.n)
        self.build(arr, 1, 0, self.n - 1)

    def build(self, arr, v, tl, tr):
        if tl == tr:
            self.tree[v] = arr[tl]
        else:
            tm = (tl + tr) // 2
            self.build(arr, 2 * v, tl, tm)
            self.build(arr, 2 * v + 1, tm + 1, tr)
            self.tree[v] = self.multiply(self.tree[2 * v], self.tree[2 * v + 1])

    def multiply(self, a, b):
        c = [[0, 0], [0, 0]]
        for i in range(2):
            for j in range(2):
                for k in range(2):
                    c[i][j] += a[i][k] * b[k][j]
                    c[i][j] %= self.mod
        return c

    def update(self, v, tl, tr, pos, new_val):
        if tl == tr:
            self.tree[v] = new_val
        else:
            tm = (tl + tr) // 2
            if pos <= tm:
                self.update(2 * v, tl, tm, pos, new_val)
            else:
                self.update(2 * v + 1, tm + 1, tr, pos, new_val)
            self.tree[v] = self.multiply(self.tree[2 * v], self.tree[2 * v + 1])

    def query(self, v, tl, tr, l, r):
        if l > r:
            return [[1, 0], [0, 1]]
        if l == tl and r == tr:
            return self.tree[v]
        tm = (tl + tr) // 2
        left_query = self.query(2 * v, tl, tm, l, min(r, tm))
        right_query = self.query(2 * v + 1, tm + 1, tr, max(l, tm + 1), r)
        return self.multiply(left_query, right_query)

# Чтение входных данных
r, n, m = map(int, input().split())
matrices = []
for _ in range(n):
    matrix = [list(map(int, input().split())) for _ in range(2)]
    matrices.append(matrix)

# Создание и инициализация дерева отрезков
seg_tree = SegmentTree(matrices, r)

# Обработка запросов
for _ in range(m):
    l, r = map(int, input().split())
    result = seg_tree.query(1, 0, n - 1, l - 1, r - 1)
    for row in result:
        print(*row)
    print()
