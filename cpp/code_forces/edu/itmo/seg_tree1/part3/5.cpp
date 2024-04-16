#include <bits/stdc++.h>
using namespace std;

struct segtree
{
    vector<long long> tree;
    int size;

    segtree(int n)
    {
        size = 1;
        while (size < n)
            size *= 2;
        tree.assign(size * 2 - 1, 0);
    }

    void add(int l, int r, int v)
    {
        add(l, v, 0, 0, size);
        add(r, -v, 0, 0, size);
    }

    void add(int i, int v, int x, int lx, int rx)
    {
        if (rx - lx == 1)
        {
            tree[x] += v;
            return;
        }
        int m = (lx + rx) / 2;
        if (i < m)
        {
            add(i, v, x * 2 + 1, lx, m);
        }
        else
        {
            add(i, v, x * 2 + 2, m, rx);
        }
        tree[x] = tree[x * 2 + 1] + tree[x * 2 + 2];
    }

    long long get(int i)
    {
        return sum(0, i, 0, 0, size);
    }

    long long sum(int l, int r, int x, int lx, int rx)
    {
        if (l >= rx || r <= lx)
            return 0;
        if (l <= lx && rx <= r)
        {
            return tree[x];
        }
        int m = (lx + rx) / 2;
        return sum(l, r, x * 2 + 1, lx, m) + sum(l, r, x * 2 + 2, m, rx);
    }
};

int main()
{
    ios::sync_with_stdio(false);
    int n, m, op, l, r, v, i;
    cin >> n >> m;
    segtree tree(n + 1);
    while (m-- > 0)
    {
        cin >> op;
        if (op == 1)
        {
            cin >> l >> r >> v;
            tree.add(l, r, v);
        }
        else
        {
            cin >> i;
            cout << tree.get(i + 1) << " ";
        }
    }
    cout << endl;
    return 0;
}