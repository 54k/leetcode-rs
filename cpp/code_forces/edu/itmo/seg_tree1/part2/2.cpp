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
        {
            size *= 2;
        }
        tree.assign(size * 2 - 1, 0);
        build(a, 0, 0, size);
    }

    int combine(int left, int right)
    {
        return left + right;
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

    void set(int i)
    {
        set(i, 0, 0, size);
    }

    void set(int i, int x, int lx, int rx)
    {
        if (rx - lx == 1)
        {
            tree[x] = 1 - tree[x];
            return;
        }

        int m = (lx + rx) / 2;
        if (i < m)
        {
            set(i, x * 2 + 1, lx, m);
        }
        else
        {
            set(i, x * 2 + 2, m, rx);
        }
        tree[x] = combine(tree[x * 2 + 1], tree[x * 2 + 2]);
    }

    int find(int k)
    {
        return find(k, 0, 0, size);
    }

    int find(int k, int x, int lx, int rx)
    {
        if (rx - lx == 1)
        {
            return lx;
        }

        int m = (lx + rx) / 2;
        if (k < tree[2 * x + 1])
        {
            return find(k, x * 2 + 1, lx, m);
        }
        else
        {
            return find(k - tree[2 * x + 1], x * 2 + 2, m, rx);
        }
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

    segtree tree(a);
    while (m-- > 0)
    {
        int op;
        cin >> op;
        if (op == 1)
        {
            int i;
            cin >> i;
            tree.set(i);
        }
        else
        {
            int k;
            cin >> k;
            cout << tree.find(k) << "\n";
        }
    }

    return 0;
}