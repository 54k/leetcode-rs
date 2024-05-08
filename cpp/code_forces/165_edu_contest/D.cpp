#include <bits/stdc++.h>
using namespace std;

#define sz(a) int((a).size())
using li = long long;

int main()
{
    int t;
    cin >> t;
    while (t--)
    {
        int n, k;
        cin >> n >> k;
        vector<int> a(n), b(n);
        for (auto &x : a)
            cin >> x;
        for (auto &x : b)
            cin >> x;
        vector<int> ord(n);
        iota(ord.begin(), ord.end(), 0);
        sort(ord.begin(), ord.end(), [&](int i, int j)
             { return b[i] > b[j]; });
        li f = 0, p = 0;
        for (int i : ord)
            p += max(0, b[i] - a[i]);
        li ans = 0;
        multiset<int> s;

        if (sz(s) == k)
            ans = max(ans, p - f);
        for (int i : ord)
        {
            p -= max(0, b[i] - a[i]);
            s.insert(a[i]);
            f += a[i];
            if (sz(s) > k)
            {
                f -= *s.rbegin();
                s.erase(--s.end());
            }
            if (sz(s) == k)
                ans = max(ans, p - f);
        }
        cout << ans << '\n';
    }
}