#include <bits/stdc++.h>
using namespace std;

struct segtree
{
    vector<long long> tree;
    int size;

    void build(int n)
    {
        size = 1;
        while (size < n)
            size *= 2;
        tree.assign(2 * size - 1, 0);
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

    long long sum(int l, int r)
    {
        return sum(l, r, 0, 0, size);
    }

    long long sum(int l, int r, int x, int lx, int rx)
    {
        if (l >= rx || lx >= r)
        {
            return 0;
        }

        if (lx >= l && rx <= r)
        {
            return tree[x];
        }

        int m = (lx + rx) / 2;

        long long s1 = sum(l, r, x * 2 + 1, lx, m);
        long long s2 = sum(l, r, x * 2 + 2, m, rx);

        return s1 + s2;
    }
};

int main()
{
    ios::sync_with_stdio(false);
    int n, m;
    cin >> n >> m;
    segtree st;
    st.build(n);

    for (int i = 0; i < n; i++)
    {
        int x;
        cin >> x;
        st.set(i, x);
    }

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
            cout << st.sum(l, r) << "\n";
        }
    }

    return 0;
}