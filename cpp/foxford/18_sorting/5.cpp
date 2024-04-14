#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n;
    cin >> n;
    vector<pair<int, int>> v1(n);
    vector<pair<int, int>> v2(n);

    for (int i = 0; i < n; i++)
    {
        cin >> v1[i].first;
        v1[i].second = i;
    }
    for (int i = 0; i < n; i++)
    {
        cin >> v2[i].first;
        v2[i].second = i;
    }

    sort(v1.begin(), v1.end());
    sort(v2.rbegin(), v2.rend());

    long long sum = 0;
    vector<long long> ans(n, 0ll);
    for (int i = 0; i < n; i++)
    {
        long long prod = v1[i].first * 1ll * v2[i].first;
        sum += prod;
        ans[v1[i].second] = v2[i].second;
    }
    cout << sum << "\n";
    for (auto &x : ans)
    {
        cout << x + 1 << " ";
    }
    cout << endl;
    return 0;
}