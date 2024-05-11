#include <bits/stdc++.h>
#include <numeric>
#include <fstream>
#include <string>
using namespace std;

#define fi(a, b) for (int i = a; i < b; ++i)
#define fj(a, b) for (int j = a; j < b; ++j)

typedef long long ll;
typedef unsigned long long ull;

typedef vector<int> vi;
typedef vector<long long> vl;
typedef vector<vector<int>> vvi;
typedef vector<vector<long long>> vvl;

const double eps = 1e-10;
const int mod = 1e9 + 7;

void solve()
{
    int n, c, k;
    cin >> n >> c >> k;
    ll Mod = pow(10ll, k);
    string s;
    cin >> s;

    vector<ll> dp(n);

    for (int i = 0; i < n; i++)
    {
        if (i <= 9 && stoll(s.substr(0, i + 1)) <= c && (i == 0 || s[0] != '0'))
        {
            dp[i]++;
        }

        for (int j = max(1, i - 10); j <= i; ++j)
        {
            if (stoll(s.substr(j, i - j + 1)) <= c && (i == j || s[j] != '0'))
            {
                dp[i] += dp[j - 1];
            }
        }
        dp[i] %= Mod;
    }
    cout << dp[n - 1];
}

int main()
{
    ios_base::sync_with_stdio(false);
    cin.tie(0);
    cout.tie(0);
    cout.precision(30);
    solve();
}