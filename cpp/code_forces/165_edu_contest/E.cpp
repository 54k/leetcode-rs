#include <bits/stdc++.h>
using namespace std;

#define sz(a) int((a).size())

vector<int> t, p;

void push(int v)
{
    if ((v * 2 + 2) >= sz(t))
        return;

    t[v * 2 + 1] += p[v];
    p[v * 2 + 1] += p[v];

    t[v * 2 + 2] += p[v];
    p[v * 2 + 2] += p[v];
    p[v] = 0;
}

void upd(int v, int l, int r, int L, int R, int x)
{
    if (L >= R)
        return;
    if (l == L && r == R)
    {
        t[v] += x;
        p[v] += x;
        return;
    }

    int m = (l + r) / 2;
    push(v);
    upd(v * 2 + 1, l, m, l, min(m, R), x);
    upd(v * 2 + 2, m, r, max(m, L), R, x);
    t[v] = min(t[v * 2 + 1], t[v * 2 + 2]);
}

int get(int v, int l, int r, int L, int R)
{
    if (L >= R)
        return 1e9;
    if (l == L && r == R)
        return t[v];
    int m = (l + r) / 2;
    push(v);
    return min(
        get(v * 2 + 1, l, m, l, min(m, R)),
        get(v * 2 + 2, m, r, max(m, L), R));
}

void solve()
{
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto &x : a)
        cin >> x, --x;
    t = p = vector<int>(4 * n);

    vector<vector<int>> pos(n);
    int ans = 0, st = 0;
    for (int i = 0; i < n; ++i)
    {
        int x = a[i];
        pos[x].push_back(i);
        int k = sz(pos[x]);
        if (k > 0)
            upd(0, 0, n, st, pos[x][k - 1] + 1, +1);
        if (k > 1)
            upd(0, 0, n, st, pos[x][k - 2] + 1, -2);
        if (k > 2)
            upd(0, 0, n, st, pos[x][k - 3] + 1, +1);

        if (get(0, 0, n, st, i + 1) == 0)
        {
            ans += 1;
            st = i + 1;
        }
    }
    cout << ans << '\n';
}

int main()
{
    int t;
    cin >> t;
    while (t--)
        solve();
}