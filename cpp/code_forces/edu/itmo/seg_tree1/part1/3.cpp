#include <bits/stdc++.h>
using namespace std;

class segtree
{
public:
    struct node
    {
        int min, cnt;
    };

    const node ZERO = {INT_MAX, 0};
    vector<node> tree;
    int size = 1;

    segtree(vector<int> &a)
    {
        init(a.size());
        build(a);
    }

    void init(int n)
    {
        while (size < n)
        {
            size *= 2;
        }
        tree.assign(size * 2 - 1, {INT_MAX, 0});
    }

    void build(vector<int> &a)
    {
        build(a, 0, 0, size);
    }

    void build(vector<int> &a, int x, int lx, int rx)
    {
        if (rx - lx == 1)
        {
            if (lx < a.size())
            {
                tree[x] = {a[lx], 1};
            }
            return;
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
            tree[x] = {v, 1};
        }
        else
        {
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
    }

    node get(int l, int r)
    {
        return get(l, r, 0, 0, size);
    }

    node get(int l, int r, int x, int lx, int rx)
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
        node n1 = get(l, r, x * 2 + 1, lx, m);
        node n2 = get(l, r, x * 2 + 2, m, rx);
        return combine(n1, n2);
    }

    node combine(node a, node b)
    {
        if (a.min < b.min)
        {
            return a;
        }
        else if (a.min > b.min)
        {
            return b;
        }
        else
        {
            return {a.min, a.cnt + b.cnt};
        }
    }
};

int main()
{
    ios::sync_with_stdio(false);
    int n, m;
    cin >> n >> m;

    vector<int> a(n);
    for (int i = 0; i < n; i++)
    {
        cin >> a[i];
    }

    segtree st(a);

    for (int i = 0; i < m; i++)
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
            int l, r;
            cin >> l >> r;
            segtree::node res = st.get(l, r);
            cout << res.min << " " << res.cnt << "\n";
        }
    }

    return 0;
}