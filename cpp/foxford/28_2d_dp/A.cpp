#include <bits/stdc++.h>
using namespace std;
using ll = long long;
int main()
{
    ll n, k, pref0 = 1, pref1 = 1;
    cin >> n >> k;
    vector<vector<vector<ll>>> dp(n + 1, vector<vector<ll>>(k, vector<ll>(2, 0)));

    for (int i = 1; i <= n; i++)
    {
        dp[i][1][0] = pref1;
        dp[i][1][1] = pref0;
        for (int j = 2; j < k; j++)
        {
            dp[i][j][0] = dp[i - 1][j - 1][0];
            pref0 += dp[i][j][0];
            dp[i][j][1] = dp[i - 1][j - 1][1];
            pref1 += dp[i][j][1];
        }
    }

    cout << pref0 + pref1 << endl;

    return 0;
}