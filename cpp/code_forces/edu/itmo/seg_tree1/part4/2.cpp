#include <bits/stdc++.h>
using namespace std;

long long MOD = 0;

struct segtree
{
    struct mat
    {
        long long a, b, c, d;
    };

    vector<mat> tree;
    int size;

    const mat ZERO{1, 0, 0, 1};

    segtree(int n)
    {
        size = n;
        tree.assign(4 * size, ZERO);
        build(1, 0, size - 1);
    }

    void build(int x, int tl, int tr)
    {
        if (tl == tr)
        {
            mat m = ZERO;
            cin >> m.a >> m.b >> m.c >> m.d;
            tree[x] = m;
            return;
        }
        int tm = (tl + tr) / 2;
        build(x * 2, tl, tm);
        build(x * 2 + 1, tm + 1, tr);
        tree[x] = combine(tree[x * 2], tree[x * 2 + 1]);
    }

    mat combine(mat &x, mat &y)
    {
        mat m{
            ((x.a % MOD * y.a % MOD) % MOD + (x.b % MOD * y.c % MOD) % MOD) % MOD,
            ((x.a % MOD * y.b % MOD) % MOD + (x.b % MOD * y.d % MOD) % MOD) % MOD,
            ((x.c % MOD * y.a % MOD) % MOD + (x.d % MOD * y.c % MOD) % MOD) % MOD,
            ((x.c % MOD * y.b % MOD) % MOD + (x.d % MOD * y.d % MOD) % MOD) % MOD};
        return m;
    }

    mat query(int l, int r)
    {
        return query(l, r, 1, 0, size - 1);
    }

    mat query(int l, int r, int x, int tl, int tr)
    {
        if (l > r)
        {
            return ZERO;
        }
        if (l == tl && r == tr)
        {
            return tree[x];
        }
        int tm = (tl + tr) / 2;
        auto lm = query(l, min(r, tm), x * 2, tl, tm);
        auto rm = query(max(tm + 1, l), r, x * 2 + 1, tm + 1, tr);
        return combine(lm, rm);
    }
};

int main()
{
    ios::sync_with_stdio(false);
    int n, m;
    cin >> MOD >> n >> m;
    segtree t(n);

    while (m-- > 0)
    {
        int l, r;
        cin >> l >> r;
        l--, r--;
        segtree::mat res = t.query(l, r);
        cout << res.a << " " << res.b << "\n";
        cout << res.c << " " << res.d << "\n";
    }
    cout << endl;
    return 0;
}