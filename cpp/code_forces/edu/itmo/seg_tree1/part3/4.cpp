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
        {
            return tree[x];
        }
        int m = (lx + rx) / 2;
        return sum(l, r, x * 2 + 1, lx, m) + sum(l, r, x * 2 + 2, m, rx);
    }
};

int main()
{
    int n, x;
    cin >> n;

    vector<int> v(n * 2);
    vector<int> ans(n);
    vector<int> seen(n, -1);

    for (auto &x : v)
    {
        int e;
        cin >> e;
        e--;
        x = e;
    }

    segtree t(n * 2);

    for (int i = 0; i < v.size(); ++i)
    {
        int el = v[i];
        if (seen[el] == -1)
        {
            t.set(i);
            seen[el] = i;
        }
        else
        {
            ans[el] += t.sum(seen[el] + 1, i);
            t.set(seen[el]);
        }
    }

    seen.assign(n, -1);

    for (int i = v.size() - 1; i >= 0; --i)
    {
        int el = v[i];
        if (seen[el] == -1)
        {
            t.set(i);
            seen[el] = i;
        }
        else
        {
            ans[el] += t.sum(i, seen[el]);
            t.set(seen[el]);
        }
    }

    for (auto &e : ans)
    {
        cout << e << " ";
    }
    cout << endl;

    return 0;
};