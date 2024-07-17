#include <bits/stdc++.h>

using namespace std;

class SegmentTree
{
public:
    SegmentTree(const vector<int> &data)
    {
        n = data.size();
        tree.resize(4 * n);
        lazy.resize(4 * n, false);
        build(data, 0, 0, n - 1);
    }

    void update(int l, int r)
    {
        update(0, 0, n - 1, l, r);
    }

    int query(int l, int r)
    {
        return query(0, 0, n - 1, l, r);
    }

private:
    vector<int> tree;
    vector<bool> lazy;
    int n;

    void build(const vector<int> &data, int node, int start, int end)
    {
        if (start == end)
        {
            tree[node] = data[start];
        }
        else
        {
            int mid = (start + end) / 2;
            build(data, 2 * node + 1, start, mid);
            build(data, 2 * node + 2, mid + 1, end);
            tree[node] = tree[2 * node + 1] + tree[2 * node + 2];
        }
    }

    void propagate(int node, int start, int end)
    {
        if (lazy[node])
        {
            tree[node] = (end - start + 1) - tree[node];
            if (start != end)
            {
                lazy[2 * node + 1] = !lazy[2 * node + 1];
                lazy[2 * node + 2] = !lazy[2 * node + 2];
            }
            lazy[node] = false;
        }
    }

    void update(int node, int start, int end, int l, int r)
    {
        propagate(node, start, end);
        if (start > end || start > r || end < l)
            return;

        if (start >= l && end <= r)
        {
            tree[node] = (end - start + 1) - tree[node];
            if (start != end)
            {
                lazy[2 * node + 1] = !lazy[2 * node + 1];
                lazy[2 * node + 2] = !lazy[2 * node + 2];
            }
            return;
        }

        int mid = (start + end) / 2;
        update(2 * node + 1, start, mid, l, r);
        update(2 * node + 2, mid + 1, end, l, r);
        tree[node] = tree[2 * node + 1] + tree[2 * node + 2];
    }

    int query(int node, int start, int end, int l, int r)
    {
        propagate(node, start, end);
        if (start > end || start > r || end < l)
            return 0;

        if (start >= l && end <= r)
            return tree[node];

        int mid = (start + end) / 2;
        int left_sum = query(2 * node + 1, start, mid, l, r);
        int right_sum = query(2 * node + 2, mid + 1, end, l, r);
        return left_sum + right_sum;
    }
};

int main()
{
    int n, m;
    cin >> n >> m;
    vector<int> heights(n);
    for (int i = 0; i < n; ++i)
    {
        cin >> heights[i];
    }

    SegmentTree segTree(heights);

    for (int i = 0; i < m; ++i)
    {
        int type, l, r;
        cin >> type >> l >> r;
        --l;
        --r; // convert to 0-based indexing
        if (type == 0)
        {
            cout << segTree.query(l, r) << endl;
        }
        else if (type == 1)
        {
            segTree.update(l, r);
        }
    }

    return 0;
}
