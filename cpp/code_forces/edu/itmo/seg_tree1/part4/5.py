class SegmentTree:
    def __init__(self, n):
        self.size = n
        self.tree = [[]] * (4 * self.size)
    
    def merge(self, left, right):
        res = []
        i, j = 0, 0
        while i < len(left) and j < len(right):
            if left[i] < right[j]:
                res.append(left[i])
                i+=1
            else:
                res.append(right[j])
                j+=1

        if i < len(left): 
            for e in left[i:]: res.append(e)
        if j < len(right): 
            for e in right[j:]: res.append(e)

        return res

    def update(self, index, value, x = 1, tl = 0, tr = None):
        if tr is None:
            tr = self.size-1

        if tr == tl:
            self.tree[x] = [value]
            return

        tm = (tl + tr) // 2
        if index <= tm:
            self.update(index, value, x*2, tl, tm)
        else:
            self.update(index, value, x*2+1, tm+1, tr)
        self.tree[x] = self.merge(self.tree[2 * x], self.tree[2 * x + 1])

    def query(self, l, r, x=1, tl=0, tr=None):
        if tr is None:
            tr = self.size - 1

        if r < tl or l > tr:
            return []

        if l <= tl and r >= tr:
            return [e for e in self.tree[x] if e != 0]

        mid = (tl + tr) // 2
        return self.merge(self.query(l, r, 2 * x, tl, mid), self.query(l, r, 2 * x + 1, mid+1, tr))


def process_events(n, events):
    segment_tree = SegmentTree(n)
    result = []
    for event in events:
        if event[0] == 1:
            i, h = event[1:]
            segment_tree.update(i, h)
        else:
            l, r, p = event[1:]
            res = segment_tree.query(l, r)
            
            s, e = -1, len(res)
            while e - s > 1:
                m = (s + e) // 2
                if res[m] <= p:
                    s = m
                else:
                    e = m
            result.append(e)

    return result


def main():
    n, m = map(int, input().split())
    events = [list(map(int, input().split())) for _ in range(m)]
    result = process_events(n, events)
    for destroyed in result:
        print(destroyed)


if __name__ == "__main__":
    main()

