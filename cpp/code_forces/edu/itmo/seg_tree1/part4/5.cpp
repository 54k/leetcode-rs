#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n, m, q, l, r;
    cin >> n >> m;

    vector<int> tree(4 * n, INT_MAX);

    function<void(int, int, int, int, int)> update = [&](int v, int tl, int tr, int pos, int new_val)
    {
        if (tl == tr)
        {
            tree[v] = new_val;
        }
        else
        {
            int tm = (tl + tr) / 2;
            if (pos <= tm)
            {
                update(v * 2, tl, tm, pos, new_val);
            }
            else
            {
                update(v * 2 + 1, tm + 1, tr, pos, new_val);
            }
            tree[v] = min(tree[v * 2], tree[2 * v + 1]);
        }
    };

    function<int(int, int, int, int, int, int)> get_left_le = [&](int v, int tl, int tr, int l, int r, int x)
    {
        if (l > r)
        {
            return INT_MAX;
        }

        if (tl == tr)
        {
            return tree[v] <= x ? tl : INT_MAX;
        }

        int tm = (tl + tr) / 2;

        if (tl < l)
        {
            return min(
                get_left_le(v * 2, tl, tm, l, min(r, tm), x),
                get_left_le(v * 2 + 1, tm + 1, tr, max(tm + 1, l), r, x));
        }

        if (tree[v * 2] <= x)
        {
            return get_left_le(v * 2, tl, tm, l, min(r, tm), x);
        }
        else
        {
            return get_left_le(v * 2 + 1, tm + 1, tr, max(tm + 1, l), r, x);
        }
    };

    while (m-- > 0)
    {
        cin >> q >> l >> r;
        if (q == 1)
        {
            update(1, 0, n - 1, l, r);
        }
        else
        {
            int p, ans, left_le;
            cin >> p;
            ans = 0;
            left_le = get_left_le(1, 0, n - 1, l, n - 1, p);
            while (left_le < r)
            {
                update(1, 0, n - 1, left_le, INT_MAX);
                left_le = get_left_le(1, 0, n - 1, l, n - 1, p);
                ans++;
            }
            cout << ans << "\n";
        }
    }
    cout << endl;
    return 0;
}