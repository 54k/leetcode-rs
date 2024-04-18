class SegmentTree:
    def __init__(self, arr):
        self.n = len(arr)
        self.tree = [0] * (4 * self.n)
        self.build(arr, 1, 0, self.n - 1)

    def build(self, arr, v, tl, tr):
        if tl == tr:
            self.tree[v] = {arr[tl]: 1}
        else:
            tm = (tl + tr) // 2
            self.build(arr, 2 * v, tl, tm)
            self.build(arr, 2 * v + 1, tm + 1, tr)
            self.tree[v] = self.merge(self.tree[2 * v], self.tree[2 * v + 1])

    def merge(self, left, right):
        merged = left.copy()
        for key, value in right.items():
            if key in merged:
                merged[key] += value
            else:
                merged[key] = value
        return merged

    def update(self, v, tl, tr, pos, new_val):
        if tl == tr:
            self.tree[v] = {new_val: 1}
        else:
            tm = (tl + tr) // 2
            if pos <= tm:
                self.update(2 * v, tl, tm, pos, new_val)
            else:
                self.update(2 * v + 1, tm + 1, tr, pos, new_val)
            self.tree[v] = self.merge(self.tree[2 * v], self.tree[2 * v + 1])

    def query(self, v, tl, tr, l, r):
        if l > r:
            return {}
        if l == tl and r == tr:
            return self.tree[v]
        tm = (tl + tr) // 2
        left_query = self.query(2 * v, tl, tm, l, min(r, tm))
        right_query = self.query(2 * v + 1, tm + 1, tr, max(l, tm + 1), r)
        return self.merge(left_query, right_query)

# Чтение входных данных
n, q = map(int, input().split())
arr = list(map(int, input().split()))

# Создание и инициализация дерева отрезков
seg_tree = SegmentTree(arr)

# Обработка запросов
for _ in range(q):
    query = list(map(int, input().split()))
    if query[0] == 1:
        l, r = query[1], query[2]
        result = seg_tree.query(1, 0, n - 1, l - 1, r - 1)
        print(len(result))
    elif query[0] == 2:
        idx, val = query[1], query[2]
        seg_tree.update(1, 0, n - 1, idx - 1, val)

