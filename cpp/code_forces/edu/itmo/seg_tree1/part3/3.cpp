#include <bits/stdc++.h>
using namespace std;

struct segtree
{
    vector<int> tree;
    int size;

    segtree(int n)
    {
        size = 1;
        while (size < n)
            size *= 2;
        tree.assign(size * 2 + 1, 0);
    }

    void set(int i)
    {
        set(i, 1, 0, 0, size);
    }

    void set(int i, int v, int x, int lx, int rx)
    {
        if (rx - lx == 1)
        {
            tree[x] = v;
            return;
        }

        int m = (lx + rx) / 2;
        if (i < m)
        {
            set(i, v, x * 2 + 1, lx, m);
        }
        else
        {
            set(i, v, x * 2 + 2, m, rx);
        }
        tree[x] = tree[x * 2 + 1] + tree[x * 2 + 2];
    }

    int sum(int l, int r)
    {
        return sum(l, r, 0, 0, size);
    }

    int sum(int l, int r, int x, int lx, int rx)
    {
        if (l >= rx || r <= lx)
        {
            return 0;
        }
        if (l <= lx && rx <= r)
        {
            return tree[x];
        }
        int m = (lx + rx) / 2;
        return sum(l, r, x * 2 + 1, lx, m) +
               sum(l, r, x * 2 + 2, m, rx);
    }
};

int main()
{
    int n, x;
    cin >> n;
    n *= 2;
    vector<int> v(n, -1);
    vector<int> ans(n / 2, 0);
    segtree t(n);

    for (int i = 0; i < n; i++)
    {
        cin >> x;
        x--; // 0 indexed
        if (v[x] == -1)
        {
            v[x] = i;
        }
        else
        {
            ans[x] = t.sum(v[x], i);
            t.set(v[x]);
        }
    }

    for (auto &e : ans)
    {
        cout << e << " ";
    }
    cout << endl;
    return 0;
};