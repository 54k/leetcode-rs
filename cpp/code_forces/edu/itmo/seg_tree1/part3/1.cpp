#include <bits/stdc++.h>
using namespace std;

struct segtree
{
    vector<int> tree;
    int size;

    segtree(vector<int> &a)
    {
        size = 1;
        while (size < a.size())
            size *= 2;
        tree.assign(size * 2 - 1, 0);
        build(a, 0, 0, size);
    }

    void build(vector<int> &a, int x, int lx, int rx)
    {
        if (rx - lx == 1)
        {
            if (lx < a.size())
            {
                tree[x] = a[lx];
            }
        }
        else
        {
            int m = (lx + rx) / 2;
            build(a, x * 2 + 1, lx, m);
            build(a, x * 2 + 2, m, rx);
            tree[x] = tree[x * 2 + 1] + tree[x * 2 + 2];
        }
    }

    void set(int i, int v)
    {
        set(i, v, 0, 0, size);
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
        if (r <= lx || l >= rx)
            return 0;

        if (l <= lx && rx <= r)
            return tree[x];

        int m = (lx + rx) / 2;
        return sum(l, r, x * 2 + 1, lx, m) +
               sum(l, r, x * 2 + 2, m, rx);
    }
};

int main()
{
    ios::sync_with_stdio(false);
    int n, x;
    cin >> n;
    int i = n;
    vector<int> v(n, 0);
    segtree sg(v);

    while (i-- > 0)
    {
        cin >> x;
        cout << sg.sum(x - 1, n) << " ";
        sg.set(x - 1, 1);
    }
    cout << endl;

    return 0;
}