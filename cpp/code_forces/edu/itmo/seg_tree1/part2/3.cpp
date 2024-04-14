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
        tree.assign(size * 2 + 1, 0);
        build(a, 0, 0, size);
    }

    int combine(int a, int b)
    {
        return max(a, b);
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
            tree[x] = combine(tree[x * 2 + 1], tree[x * 2 + 2]);
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

        tree[x] = combine(tree[x * 2 + 1], tree[x * 2 + 2]);
    }

    int first_above(int v)
    {
        return first_above(v, 0, 0, size);
    }

    int first_above(int v, int x, int lx, int rx)
    {
        if (tree[x] < v)
        {
            return -1;
        }

        if (rx - lx == 1)
        {
            return lx;
        }
        int m = (lx + rx) / 2;
        int res = first_above(v, x * 2 + 1, lx, m);
        if (res == -1)
        {
            res = first_above(v, x * 2 + 2, m, rx);
        }
        return res;
    }
};

int main()
{
    ios::sync_with_stdio(false);
    int n, m;
    cin >> n >> m;

    vector<int> a(n);
    for (auto &e : a)
    {
        cin >> e;
    }

    segtree st(a);

    while (m-- > 0)
    {
        int c;
        cin >> c;

        if (c == 1)
        {
            int i, v;
            cin >> i >> v;
            st.set(i, v);
        }
        else
        {
            int x, l;
            cin >> x >> l;
            cout << st.first_above(x, l) << "\n";
        }
    }

    return 0;
}