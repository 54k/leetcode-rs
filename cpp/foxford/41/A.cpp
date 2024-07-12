#include <bits/stdc++.h>
using namespace std;

const int INF = 1e9;
const int N = 1e5 + 1;

struct SegmentTree
{
    vector<pair<long long, long long>> tree;
    int n;
    SegmentTree(int n) : tree(4 * n), n(n)
    {
        build(0, 0, n);
    }

    void build(int v, int l, int r)
    {
        if (l + 1 == r)
        {
            long long calc = ((long long)pow(l + 1, 2) % 12345) + ((long long)pow(l + 1, 3) % 23456);
            tree[v] = make_pair(calc, calc);
            return;
        }
        int mid = (l + r) / 2;
        build(v * 2 + 1, l, mid);
        build(v * 2 + 2, mid, r);
        pair<long long, long long> p = make_pair(min(tree[v * 2 + 1].first, tree[v * 2 + 2].first), max(tree[v * 2 + 1].second, tree[v * 2 + 2].second));
        tree[v] = p;
    }

    void set(int i, int val)
    {
        set(0, 0, n, i, val);
    }

    void set(int v, int l, int r, int i, int val)
    {
        if (l + 1 == r)
        {
            tree[v] = make_pair(val, val);
            return;
        }
        int mid = (l + r) / 2;
        if (i < mid)
        {
            set(v * 2 + 1, l, mid, i, val);
        }
        else
        {
            set(v * 2 + 2, mid, r, i, val);
        }
        pair<long long, long long> p = make_pair(min(tree[v * 2 + 1].first, tree[v * 2 + 2].first), max(tree[v * 2 + 1].second, tree[v * 2 + 2].second));
        tree[v] = p;
    }

    pair<long long, long long> get(int l, int r)
    {
        return get(0, 0, n, l, r);
    }

    pair<long long, long long> get(int v, int l, int r, int ql, int qr)
    {
        if (qr <= l || r <= ql)
        {
            return make_pair(INF, -INF);
        }

        if (ql <= l && r <= qr)
        {
            return tree[v];
        }

        int mid = (l + r) / 2;
        auto left = get(v * 2 + 1, l, mid, ql, qr);
        auto right = get(v * 2 + 2, mid, r, ql, qr);
        pair<long long, long long> p = make_pair(min(left.first, right.first), max(left.second, right.second));
        return p;
    }
};

int main()
{
    int k;
    cin >> k;
    vector<pair<int, int>> queries(k);
    for (int i = 0; i < k; i++)
    {
        cin >> queries[i].first >> queries[i].second;
    }
    SegmentTree seg_tree(N);
    for (auto &q : queries)
    {
        if (q.first > 0)
        {
            auto min_max = seg_tree.get(q.first - 1, q.second);
            cout << min_max.second - min_max.first << "\n";
        }
        else
        {
            seg_tree.set((-q.first) - 1, q.second);
        }
    }

    return 0;
}