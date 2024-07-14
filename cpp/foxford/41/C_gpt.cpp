#include <bits/stdc++.h>
using namespace std;

const int INF = 1e9;
const int N = 1e5 + 1;

struct SegmentTree
{
    vector<int> arr;
    vector<int> tree;
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
                tree[v] = arr[l] > 0 ? 0 : 1;
            }
            return;
        }
        int mid = (l + r) / 2;
        build(v * 2 + 1, l, mid, arr);
        build(v * 2 + 2, mid, r, arr);
        tree[v] = tree[v * 2 + 1] + tree[v * 2 + 2];
    }

    void set(int i, int val)
    {
        set(0, 0, n, i, val);
    }

    void set(int v, int l, int r, int i, int val)
    {
        if (l + 1 == r)
        {
            tree[v] = val > 0 ? 0 : 1;
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
        tree[v] = tree[v * 2 + 1] + tree[v * 2 + 2];
    }

    int get(int l, int r, int k)
    {
        return get(0, 0, n, l, r, k);
    }

    int get(int v, int l, int r, int ql, int qr, int k)
    {
        if (l >= qr || r <= ql)
        {
            return -1;
        }
        if (l + 1 == r)
        {
            return tree[v] ? l : -1;
        }

        int mid = (l + r) / 2;

        int sum_left = sum(v * 2 + 1, l, mid, ql, qr);
        if (k <= sum_left)
        {
            return get(v * 2 + 1, l, mid, ql, qr, k);
        }
        return get(v * 2 + 2, mid, r, ql, qr, k - sum_left);
    }

    int sum(int v, int l, int r, int ql, int qr)
    {
        if (qr <= l || r <= ql)
        {
            return 0;
        }

        if (ql <= l && r <= qr)
        {
            return tree[v];
        }
        int mid = (l + r) / 2;
        return sum(v * 2 + 1, l, mid, ql, qr) + sum(v * 2 + 2, mid, r, ql, qr);
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

    SegmentTree seg_tree(n, arr);

    int m;
    cin >> m;
    vector<pair<char, int>> queries(m);
    for (int i = 0; i < m; i++)
    {
        char op;
        int a, b, c;
        cin >> op;
        if (op == 'u')
        {
            cin >> a >> b;
            seg_tree.set(a - 1, b);
        }
        else
        {
            cin >> a >> b >> c;
            int res = seg_tree.get(a - 1, b, c);
            cout << (res == -1 ? -1 : res + 1) << " ";
        }
    }
    cout << endl;
    return 0;
}
