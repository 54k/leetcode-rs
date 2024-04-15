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

    int find(int v)
    {
        return find(v, 0, 0, size);
    }

    int find(int v, int x, int lx, int rx)
    {
        if (rx - lx == 1)
        {
            return lx;
        }
        int m = (lx + rx) / 2;
        if (v < tree[x * 2 + 2])
        {
            return find(v, x * 2 + 2, m, rx);
        }
        else
        {
            return find(v - tree[x * 2 + 2], x * 2 + 1, lx, m);
        }
    }
};

int main()
{
    ios::sync_with_stdio(false);
    int n, e;
    cin >> n;
    int i = n;

    vector<int> a(n, 1);
    segtree t(a);

    vector<int> p(n);
    for (auto &x : p)
    {
        cin >> x;
    }

    deque<int> ans;
    for (int i = n - 1; i >= 0; i--)
    {
        int e = p[i];
        int f = t.find(e);
        t.set(f, 0);
        ans.push_front(f + 1);
    }

    for (auto &x : ans)
    {
        cout << x << " ";
    }

    cout << endl;
    return 0;
}