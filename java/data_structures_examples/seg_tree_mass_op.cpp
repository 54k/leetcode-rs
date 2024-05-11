#include <bits/stdc++.h>
using namespace std;

struct segtree
{

    const int INF = 0;
    vector<int> a, t, push;
    int n;

    explicit segtree(vector<int> &arr)
    {
        a = arr;
        n = arr.size();
        t.resize(4 * n);
        push.resize(4 * n);
        build(0, 0, n);
    }

    void build(int v, int l, int r)
    {
        if (l + 1 == r)
        {
            if (l < n)
                t[v] = a[l];
            return;
        }
        int m = (l + r) / 2;
        build(v * 2 + 1, l, m);
        build(v * 2 + 2, m, r);
        t[v] = t[v * 2 + 1] + t[v * 2 + 2];
    }

    void make_push(int v, int l, int m, int r)
    {
        if (push[v] == 0)
            return;

        push[v * 2 + 1] += push[v];
        push[v * 2 + 2] += push[v];
        t[v * 2 + 1] += push[v] * (m - l);
        t[v * 2 + 2] += push[v] * (r - m);
        push[v] = 0;
    }

    int get(int ql, int qr)
    {
        return get(0, 0, n, ql, qr);
    }

    int get(int v, int l, int r, int ql, int qr)
    {
        if (qr <= l || r <= ql)
        {
            return INF;
        }

        if (ql <= l && r <= qr)
        {
            return t[v];
        }

        int m = (l + r) / 2;
        make_push(v, l, m, r);
        return get(v * 2 + 1, l, m, ql, qr) + get(v * 2 + 2, m, r, ql, qr);
    }

    void add(int ql, int qr, int d)
    {
        add(0, 0, n, ql, qr, d);
    }

    void add(int v, int l, int r, int ql, int qr, int d)
    {
        if (qr <= l || r <= ql)
        {
            return;
        }

        if (ql <= l && r <= qr)
        {
            push[v] += d;
            t[v] += d * (r - l);
            return;
        }
        int m = (l + r) / 2;
        make_push(v, l, m, r);
        add(v * 2 + 1, l, m, ql, qr, d);
        add(v * 2 + 2, m, r, ql, qr, d);
        t[v] = t[v * 2 + 1] + t[v * 2 + 2];
    }
};

int main()
{
    vector<int> arr = {0, 0, 0, 0};
    int n = arr.size();
    segtree tree(arr);
    // segtree tree = arr;
    tree.add(0, n, 1);
    tree.add(n / 2, n, 2);
    cout << tree.get(0, n) << endl;     // 8
    cout << tree.get(n / 2, n) << endl; // 6
}