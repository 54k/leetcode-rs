#include <bits/stdc++.h>
using namespace std;

int main()
{
    const int mod = 16714589;
    int n, m, a;
    cin >> n >> m >> a;

    int lg = (int)log2(n);
    vector<vector<int>> st(lg + 1, vector<int>(n, 1 << 30));

    for (int l = 0; l <= lg; l++)
    {
        for (int i = 0; i + (1 << l) <= n; i++)
        {
            if (l == 0)
            {
                if (i == 0)
                    st[l][i] = a;
                else
                {
                    st[l][i] = (23 * st[l][i - 1] + 21563) % mod;
                }
            }
            else
            {
                st[l][i] = min(st[l - 1][i], st[l - 1][i + (1 << (l - 1))]);
            }
        }
    }

    function<int(int, int)> get_min = [&](int l, int r)
    {
        l--, r--;
        int lg = log2(r - l + 1);
        return min(st[lg][l], st[lg][r - (1 << lg) + 1]);
    };

    int u, v;
    cin >> u >> v;
    int ans = get_min(u, v);
    for (int i = 2; i <= m; i++)
    {
        u = ((17 * u + 751 + ans + 2 * (i - 1)) % n) + 1;
        v = ((13 * v + 593 + ans + 5 * (i - 1)) % n) + 1;
        ans = get_min(min(u, v), max(u, v));
    }
    cout << u << " " << v << " " << ans << endl;

    return 0;
}