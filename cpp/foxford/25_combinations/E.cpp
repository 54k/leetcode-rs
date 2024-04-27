#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n;
    cin >> n;
    vector<long long> dp(n + 1);
    dp[0] = 1;
    {
        for (auto &c : {1, 2, 5, 10})
        {
            for (int i = 0; i <= n; i++)
                if (i >= c)
                {
                    dp[i] += dp[i - c];
                }
        }
    }
    cout << dp[n] << endl;

    return 0;
}

// #include<iostream>
// using namespace std;
// int main()
// {
//     int n, i;
//     long long ans = 0;
//     cin >> n;
//     for (i = 0; i <= n; i += 5)
//         ans += (long long) (1 + i / 10) * (1 + (n - i) / 2);
//     cout << ans << endl;
//     return 0;
// }