#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n;
    cin >> n;
    vector<int> t;
    t.resize(n);
    for (auto &x : t)
    {
        cin >> x;
    }

    int ans = 0;
    for (int i = 0; i < n; i++)
    {
        if (t[i] != 1)
        {
            continue;
        }
        int d = 1 << 10;
        for (int j = 0; j < n; j++)
        {
            if (j == i)
            {
                continue;
            }

            if (t[j] == 2)
            {
                d = min(d, abs(j - i));
            }
        }
        ans = max(ans, d);
    }
    cout << ans;
    return 0;
}