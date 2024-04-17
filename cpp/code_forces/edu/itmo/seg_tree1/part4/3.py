# https://www.geeksforgeeks.org/counting-segment-inversions-of-an-array-with-updates/
class Node:
    def __init__(self):
        self.freq = [0]*41
        self.cnt = 0

class SegmentTree:
    def __init__(self, arr):
        self.arr = arr
        self.n = len(arr)
        self.tree = [Node() for _ in range(4 * self.n)]
        self.build(1, 0, self.n - 1)
    
    def combine(self, left, right):
        res = Node()
        res.cnt = left.cnt + right.cnt
        pref = [0] * 41
        pref[0] = right.freq[0]

        for i in range(41):
            res.freq[i] = left.freq[i] + right.freq[i]
            if i != 0:
                res.cnt += pref[i-1] * left.freq[i]
            pref[i] = pref[i-1] + right.freq[i]

        return res
        
    def build(self, node, start, end):
        if start == end:
            self.tree[node] = Node()
            self.tree[node].freq[self.arr[start]] = 1
        else:
            mid = (start + end) // 2
            self.build(node * 2, start, mid)
            self.build(node * 2 + 1, mid + 1, end)
            self.tree[node] = self.combine(self.tree[node*2], self.tree[node*2 + 1])

    def update(self, node, start, end, idx, val):
        if start == end:
            self.tree[node].freq[self.arr[start]] = 0
            self.tree[node].freq[val] = 1
            self.arr[start] = val
        else:
            mid = (start + end) // 2
            if start <= idx <= mid:
                self.update(node * 2, start, mid, idx, val)
            else:
                self.update(node * 2 + 1, mid + 1, end, idx, val)
            self.tree[node] = self.combine(self.tree[node*2], self.tree[node*2 + 1])

    def query(self, node, start, end, left, right):
        if start > right or end < left:
            return Node()
        if start >= left and end <= right:
            return self.tree[node]
        mid = (start + end) // 2
        return self.combine(self.query(node * 2, start, mid, left, right), self.query(node * 2 + 1, mid + 1, end, left, right))

n, q = map(int, input().split())
a = list(map(int, input().split()))

tree = SegmentTree(a)

for _ in range(q):
    query = input().split()
    if query[0] == '1':
        left, right = map(int, query[1:])
        print(tree.query(1, 0, n - 1, left - 1, right - 1).cnt)
    else:
        idx, val = map(int, query[1:])
        tree.update(1, 0, n - 1, idx - 1, val)