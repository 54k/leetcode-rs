#include <bits/stdc++.h>
using namespace std;

struct segtree
{
    vector<long long> tree;
    int size;

    segtree(vector<int> &arr)
    {
        size = 1;
        while (size < arr.size())
            size *= 2;
        tree.assign(size * 2 - 1, 0);
        build(arr, 0, 0, size);
    }

    void build(const vector<int> &arr, int x, int lx, int rx)
    {
        if (rx - lx == 1)
        {
            if (lx < arr.size())
            {
                tree[x] = lx % 2 == 1 ? arr[lx] : -arr[lx];
            }
            return;
        }
        int m = (lx + rx) / 2;
        build(arr, x * 2 + 1, lx, m);
        build(arr, x * 2 + 2, m, rx);
        tree[x] = tree[x * 2 + 1] + tree[x * 2 + 2];
    }

    void set(int i, int v)
    {
        set(i, v, 0, 0, size);
    }

    void set(int i, int v, int x, int lx, int rx)
    {
        if (rx - lx == 1)
        {
            tree[x] = lx % 2 == 1 ? v : -v;
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

    long long sum(int l, int r)
    {
        return sum(l, r, 0, 0, size);
    }

    long long sum(int l, int r, int x, int lx, int rx)
    {
        if (r <= lx || l >= rx)
        {
            return 0;
        }
        if (l <= lx && r >= rx)
        {
            return tree[x];
        }
        int m = (lx + rx) / 2;
        return sum(l, r, x * 2 + 1, lx, m) + sum(l, r, x * 2 + 2, m, rx);
    }
};

int main()
{
    int n, m;
    cin >> n;
    vector<int> v(n);
    for (auto &x : v)
    {
        cin >> x;
    }
    segtree t(v);
    cin >> m;
    while (m-- > 0)
    {
        int q, l, r;
        cin >> q;
        if (!q)
        {
            cin >> l >> r;
            t.set(l - 1, r);
        }
        else
        {
            cin >> l >> r;
            cout << t.sum(l - 1, r) * (l % 2 == 0 ? 1 : -1) << "\n";
        }
    }
    cout << endl;
    return 0;
}