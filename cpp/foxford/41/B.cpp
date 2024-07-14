#include <bits/stdc++.h>
using namespace std;

const int INF = 1e9;

struct SegmentTree
{
    vector<int> arr;
    vector<pair<int, int>> tree;
    int n;
    SegmentTree(int n, vector<int> &arr) : tree(4 * n), n(n)
    {
        build(0, 0, n, arr);
    }

    void build(int v, int l, int r, vector<int> &arr)
    {
        if (l + 1 == r)
        {
            if (l < n)
            {
                tree[v] = make_pair(arr[l], l + 1);
            }
            return;
        }
        int mid = (l + r) / 2;
        build(v * 2 + 1, l, mid, arr);
        build(v * 2 + 2, mid, r, arr);
        pair<int, int> p = make_pair(max(tree[v * 2 + 1].first, tree[v * 2 + 2].first), tree[v * 2 + 1].first >= tree[v * 2 + 1].first ? tree[v * 2 + 1].second : tree[v * 2 + 2].second);
        tree[v] = p;
    }

    pair<int, int> get(int l, int r)
    {
        return get(0, 0, n, l, r);
    }

    pair<int, int> get(int v, int l, int r, int ql, int qr)
    {
        if (qr <= l || r <= ql)
        {
            return make_pair(-INF, -INF);
        }

        if (ql <= l && r <= qr)
        {
            return tree[v];
        }

        int mid = (l + r) / 2;
        auto left = get(v * 2 + 1, l, mid, ql, qr);
        auto right = get(v * 2 + 2, mid, r, ql, qr);
        pair<int, int> p = make_pair(max(left.first, right.first), left.first >= right.first ? left.second : right.second);
        return p;
    }
};

int main()
{
    int n;
    cin >> n;
    vector<int> arr(n);
    for (int i = 0; i < n; i++)
    {
        cin >> arr[i];
    }

    int k;
    cin >> k;
    vector<pair<int, int>> queries(k);
    for (int i = 0; i < k; i++)
    {
        cin >> queries[i].first >> queries[i].second;
    }

    SegmentTree seg_tree(n, arr);

    for (auto &q : queries)
    {
        auto p = seg_tree.get(q.first - 1, q.second);
        cout << p.first << " " << p.second << "\n";
    }

    return 0;
}