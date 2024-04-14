#include <bits/stdc++.h>
using namespace std;

struct segtree
{
    struct node
    {
        long long seg, pref, suf, sum;
    };

    vector<node> tree;
    int size;

    node combine(node a, node b)
    {
        return {
            max(a.seg, max(a.suf + b.pref, b.seg)),
            max(a.pref, a.sum + b.pref),
            max(b.suf, b.sum + a.suf),
            a.sum + b.sum,
        };
    }

    node one_element(int x)
    {
        return {
            max(0, x),
            max(0, x),
            max(0, x),
            x};
    }

    const node ZERO = {0, 0, 0, 0};

    segtree(vector<int> &a)
    {
        int n = a.size();
        size = 1;
        while (size < n)
        {
            size *= 2;
        }
        tree.assign(size * 2 - 1, ZERO);
        build(a, 0, 0, size);
    }

    void build(vector<int> &a, int x, int lx, int rx)
    {
        if (rx - lx == 1)
        {
            if (lx < a.size())
            {
                tree[x] = one_element(a[lx]);
            }
            return;
        }
        int m = (lx + rx) / 2;
        build(a, x * 2 + 1, lx, m);
        build(a, x * 2 + 2, m, rx);
        tree[x] = combine(tree[x * 2 + 1], tree[x * 2 + 2]);
    }

    void set(int i, int v)
    {
        set(i, v, 0, 0, size);
    }

    void set(int i, int v, int x, int lx, int rx)
    {
        if (rx - lx == 1)
        {
            tree[x] = one_element(v);
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

    node calc(int l, int r)
    {
        return calc(l, r, 0, 0, size);
    }

    node calc(int l, int r, int x, int lx, int rx)
    {
        if (l >= rx || lx >= r)
        {
            return ZERO;
        }
        if (lx >= l && rx <= r)
        {
            return tree[x];
        }
        int m = (lx + rx) / 2;
        node s1 = calc(l, r, 2 * x + 1, lx, m);
        node s2 = calc(l, r, 2 * x + 2, m, rx);
        return combine(s1, s2);
    }
};

int main()
{
    int n, m, i, v;
    cin >> n >> m;

    vector<int> a(n);
    for (auto &e : a)
    {
        cin >> e;
    }

    segtree tree(a);

    cout << tree.tree[0].seg << "\n";

    while (m-- > 0)
    {
        cin >> i >> v;
        tree.set(i, v);
        cout << tree.tree[0].seg << "\n";
    }

    return 0;
}