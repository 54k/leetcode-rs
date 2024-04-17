#include <bits/stdc++.h>
using namespace std;

#define ll long long

struct segtree
{
    struct node
    {
        vector<ll> freq;
        ll cnt;
        node() : freq(41), cnt(0) {}
    };

    vector<node> tree;
    int size;

    segtree(vector<int> &arr)
    {
        size = arr.size();
        tree.assign(4 * size, node());
        build(1, 0, size - 1, arr);
    }

    node combine(const node &left, const node &right)
    {
        node res;
        res.cnt = left.cnt + right.cnt;
        vector<ll> pref(41);
        pref[0] = right.freq[0];
        for (int i = 1; i < 41; i++)
        {
            res.freq[i] = left.freq[i] + right.freq[i];
            res.cnt += left.freq[i] * pref[i - 1];
            pref[i] = pref[i - 1] + right.freq[i];
        }
        return res;
    }

    void build(int x, int tl, int tr, vector<int> &arr)
    {
        if (tl == tr)
        {
            tree[x] = node();
            tree[x].freq[arr[tl]] = 1;
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
            tree[x].freq[v] = 1;
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

    ll query(int l, int r)
    {
        return query(l, r, 1, 0, size - 1).cnt;
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
        return combine(query(l, r, x * 2, tl, tm), query(l, r, x * 2 + 1, tm + 1, tr));
    }
};

// https://www.geeksforgeeks.org/counting-segment-inversions-of-an-array-with-updates/
int main()
{
    ios::sync_with_stdio(false);
    int n, m;
    cin >> n >> m;

    vector<int> v(n);
    for (auto &x : v)
    {
        cin >> x;
    }

    segtree tree(v);

    while (m-- > 0)
    {
        int q, l, r;
        cin >> q;
        if (q == 2)
        {
            cin >> l >> r;
            tree.update(l - 1, r);
        }
        else
        {
            cin >> l >> r;
            cout << tree.query(l - 1, r - 1) << "\n";
        }
    }
    cout << endl;
    return 0;
}