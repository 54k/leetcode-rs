#include <bits/stdc++.h>
using namespace std;
using li = long long;
const li INF = 1e18;

int main()
{
    int t;
    cin >> t;
    while (t--)
    {
        int n, k;
        cin >> n >> k;
        vector<li> a(n);
        for (auto &x : a)
        {
            cin >> x;
        }

        vector<vector<li>> dp(n + 1, vector<li>(k + 1, INF));
        dp[0][0] = 0;

        for (int i = 0; i < n; ++i)
        {
            for (int j = 0; j <= k; ++j)
            {
                li mn = INF;
                for (int d = 0; j + d <= k && i + d < n; ++d)
                {
                    mn = min(mn, a[i + d]);
                    dp[i + d + 1][j + d] = min(dp[i + d + 1][j + d], dp[i][j] + (d + 1) * mn);
                }
            }
        }
        cout << *min_element(dp[n].begin(), dp[n].end()) << "\n";
    }
}