#include <bits/stdc++.h>
using namespace std;

struct SegTree
{
    struct Node
    {
        vector<int> val;
    };

    vector<Node> tree;
    int n;

    SegTree(const vector<int> &arr)
    {
        n = arr.size();
        tree = vector<Node>(4 * n);
        build(0, 0, n, arr);
    }

    Node combine(Node &a, Node &b)
    {
        Node node = Node();
        int i = 0, j = 0;
        for (; i < a.val.size() && j < b.val.size();)
        {
            if (a.val[i] <= b.val[j])
            {
                node.val.push_back(a.val[i]);
                i++;
            }
            else
            {
                node.val.push_back(b.val[j]);
                j++;
            }
        }

        for (; i < a.val.size(); i++)
            node.val.push_back(a.val[i]);

        for (; j < b.val.size(); j++)
            node.val.push_back(b.val[j]);

        return node;
    }

    void build(int v, int l, int r, const vector<int> &arr)
    {
        if (l + 1 == r)
        {
            if (l < arr.size())
                tree[v].val.push_back(arr[l]);
            return;
        }

        int mid = (l + r) / 2;
        build(v * 2 + 1, l, mid, arr);
        build(v * 2 + 2, mid, r, arr);
        tree[v] = combine(tree[v * 2 + 1], tree[v * 2 + 2]);
    }

    Node get(int l, int r)
    {
        return get(0, 0, n, l, r);
    }

    Node get(int v, int l, int r, int ql, int qr)
    {
        if (qr <= l || ql >= r)
            return Node();

        if (ql <= l && r <= qr)
        {
            return tree[v];
        }
        int mid = (l + r) / 2;
        Node left = get(v * 2 + 1, l, mid, ql, qr);
        Node right = get(v * 2 + 2, mid, r, ql, qr);
        return combine(left, right);
    }
};

int main()
{
    int n, q;
    cin >> n;
    vector<int> arr(n);
    for (int i = 0; i < n; i++)
    {
        cin >> arr[i];
    }
    SegTree tree(arr);
    cin >> q;
    while (q-- > 0)
    {
        int l, r, k;
        cin >> l >> r >> k;
        SegTree::Node node = tree.get(l - 1, r);

        for (int i = 0; i < k; i++)
            cout << node.val[i] << " ";
        cout << "\n";
    }
    cout << endl;
    return 0;
}