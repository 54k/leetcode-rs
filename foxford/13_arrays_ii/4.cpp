#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n, m;
    cin >> n >> m;
    vector<long long> nd;
    vector<long long> md;
    while (n-- > 0)
    {
        int e;
        cin >> e;
        nd.emplace_back(e);
    }

    while (m-- > 0)
    {
        int e;
        cin >> e;
        md.emplace_back(e);
    }

    long long ans = 4294967295;
    for (auto &x : nd)
    {
        vector<long long>::iterator it = upper_bound(md.begin(), md.end(), x);

        if (it != md.end())
        {
            auto val = *(it);
            // cout << x << " " << val << endl;
            ans = min(ans, abs(x - val));
        }

        if (it != md.begin())
        {
            auto val = *(--it);
            // cout << x << " " << val << endl;
            ans = min(ans, abs(x - val));
        }
    }

    cout << ans << endl;

    return 0;
}