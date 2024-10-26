#include <bits/stdc++.h>
using namespace std;

int main()
{
    long long n, k;
    cin >> n >> k;
    vector<long long> v(n);
    for (auto &x : v)
    {
        cin >> x;
    }

    function<long long(long long)> f = [&](long long d)
    {
        long long ans = 1;
        long long prev = v[0];

        for (auto &x : v)
        {
            if (prev + d > x)
            {
                continue;
            }
            ans++;
            prev = x;
        }
        return ans;
    };

    long long lo = 0;
    long long hi = 1e9;
    while (lo + 1 != hi)
    {
        long long mid = (lo + hi) / 2;
        if (f(mid) >= k)
        {
            lo = mid;
        }
        else
        {
            hi = mid;
        }
    }
    cout << lo << endl;
    return 0;
}