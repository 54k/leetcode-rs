#include <bits/stdc++.h>
using namespace std;
int main()
{
    long long w, h, n;
    cin >> w >> h >> n;
    long long lo = 0, hi = max(w, h) * n;
    while (lo + 1 < hi)
    {
        long long mid = lo + (hi - lo) / 2;
        long long cnt = (mid / w) * (mid / h);
        if (cnt < n)
        {
            lo = mid;
        }
        else
        {
            hi = mid;
        }
    }
    cout << hi << endl;
    return 0;
}