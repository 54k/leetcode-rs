#include <bits/stdc++.h>
using namespace std;

struct segtree
{
    vector<int> a;
    vector<int> t, push;

    void build(int v, int l, int r)
    {
        if (l + 1 == r)
        {
            t[v] = a[l];
            return;
        }
        int m = (l + r) / 2;
        build(v * 2 + 1, l, m);
        build(v * 2 + 2, m + 1, r);
        t[v] = min(t[v * 2 + 1], t[v * 2 + 2]);
    }
}