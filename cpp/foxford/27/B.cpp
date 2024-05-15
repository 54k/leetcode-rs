#include <bits/stdc++.h>

using namespace std;

#define ll long long

int main()
{
    int n, m;
    cin >> n >> m;

    vector<vector<int>> dp(n + 1, vector<int>(m + 1));
    dp[1][1] = 1;

    for (int i = 1; i <= n; i++)
    {
        for (int j = 1; j <= m; j++)
        {
            dp[i][j] = 1;
            for (int k = 1; k < i; k++)
            {
                dp[i][j] += dp[k][j];
            }

            for (int k = 1; k < j; k++)
            {
                dp[i][j] += dp[i][k];
            }
        }
    }

    cout << dp[n][m];
}