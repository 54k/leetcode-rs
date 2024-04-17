class SegmentTree:
    def __init__(self, arr):
        self.n = len(arr)
        self.tree = [0] * (4 * self.n)
        self.build(arr, 1, 0, self.n - 1)

    def build(self, arr, v, tl, tr):
        if tl == tr:
            self.tree[v] = arr[tl] if tl % 2 == 0 else -arr[tl]
        else:
            tm = (tl + tr) // 2
            self.build(arr, 2 * v, tl, tm)
            self.build(arr, 2 * v + 1, tm + 1, tr)
            self.tree[v] = self.tree[2 * v] + self.tree[2 * v + 1]

    def update(self, v, tl, tr, pos, new_val):
        if tl == tr:
            self.tree[v] = new_val if tl % 2 == 0 else -new_val
        else:
            tm = (tl + tr) // 2
            if pos <= tm:
                self.update(2 * v, tl, tm, pos, new_val)
            else:
                self.update(2 * v + 1, tm + 1, tr, pos, new_val)
            self.tree[v] = self.tree[2 * v] + self.tree[2 * v + 1]

    def query(self, v, tl, tr, l, r):
        if l > r:
            return 0
        if l == tl and r == tr:
            return self.tree[v]
        tm = (tl + tr) // 2
        return self.query(2 * v, tl, tm, l, min(r, tm)) + self.query(2 * v + 1, tm + 1, tr, max(l, tm + 1), r)

def alternating_sum(arr, queries):
    seg_tree = SegmentTree(arr)
    result = []
    for query in queries:
        if query[0] == 0:
            seg_tree.update(1, 0, seg_tree.n - 1, query[1] - 1, query[2])
        else:
            l, r = query[1], query[2]
            result.append(seg_tree.query(1, 0, seg_tree.n - 1, l - 1, r - 1) * (-1 if l%2==0 else 1))
    return result

# Чтение входных данных
n = int(input())
arr = list(map(int, input().split()))
m = int(input())
queries = []
for _ in range(m):
    query = list(map(int, input().split()))
    queries.append(query)

# Получение и вывод результатов
result = alternating_sum(arr, queries)
for res in result:
    print(res)
