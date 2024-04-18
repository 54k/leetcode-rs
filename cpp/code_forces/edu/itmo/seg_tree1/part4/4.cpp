#include <bits/stdc++.h>
using namespace std;

struct segtree
{
    struct node
    {
        vector<int> v;
        node() : v(41) {}
    };
    vector<node> tree;
    int size;

    segtree(vector<int> &arr)
    {
        size = arr.size();
        tree.assign(size * 4, node());
        build(1, 0, size - 1, arr);
    }

    node combine(const node &left, const node &right)
    {
        node res;
        for (int i = 0; i < 41; i++)
        {
            res.v[i] = left.v[i] + right.v[i];
        }

        return res;
    }

    void build(int x, int tl, int tr, vector<int> &arr)
    {
        if (tl == tr)
        {
            tree[x] = node();
            tree[x].v[arr[tl]] = 1;
        }
        else
        {
            int tm = (tl + tr) / 2;
            build(x * 2, tl, tm, arr);
            build(x * 2 + 1, tm + 1, tr, arr);
            tree[x] = combine(tree[x * 2], tree[x * 2 + 1]);
        }
    }

    void update(int i, int v)
    {
        update(i, v, 1, 0, size - 1);
    }

    void update(int i, int v, int x, int tl, int tr)
    {
        if (tl == tr)
        {
            tree[x] = node();
            tree[x].v[v] = 1;
        }
        else
        {
            int tm = (tl + tr) / 2;
            if (i <= tm)
            {
                update(i, v, x * 2, tl, tm);
            }
            else
            {
                update(i, v, x * 2 + 1, tm + 1, tr);
            }
            tree[x] = combine(tree[x * 2], tree[x * 2 + 1]);
        }
    }

    int query(int l, int r)
    {
        int ans = 0;
        auto res = query(l, r, 1, 0, size - 1);
        for (int i = 0; i < 41; i++)
        {
            if (res.v[i] > 0)
            {
                ans++;
            }
        }
        return ans;
    }

    node query(int l, int r, int x, int tl, int tr)
    {
        if (l > tr || r < tl)
        {
            return node();
        }
        if (l <= tl && r >= tr)
        {
            return tree[x];
        }
        int tm = (tl + tr) / 2;
        return combine(query(l, r, x * 2, tl, tm),
                       query(l, r, x * 2 + 1, tm + 1, tr));
    }
};
/**
7 7
1 2 3 6 5 4 19
1 1 6
1 1 3
1 2 5
1 2 4
2 2 8
1 1 6
1 1 3

*/
int main()
{
    ios::sync_with_stdio(false);
    int n, m;
    cin >> n >> m;
    vector<int> v(n);
    for (auto &x : v)
        cin >> x;

    segtree tree(v);
    while (m-- > 0)
    {
        int q, l, r;
        cin >> q >> l >> r;
        if (q == 1)
        {
            cout << tree.query(l - 1, r - 1) << "\n";
        }
        else
        {
            tree.update(l - 1, r);
        }
    }
    cout << endl;
    return 0;
}