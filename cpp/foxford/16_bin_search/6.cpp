#include <bits/stdc++.h>
using namespace std;

int main() {
    long long n, x, y;
    cin >> n >> x >> y;

    function<bool(long long)> f = [&](long long t) { 
        int tot = 1;
        t -= min(x, y);
        tot += t / x;
        tot += t / y;
        return tot >= n;
    };

    long long lo = (min(x, y))-1;
    long long hi = n * (max(x, y));

    while (lo + 1 < hi) {
        long long mid = lo + (hi - lo) / 2;
        if (f(mid)) {
            hi = mid;
        } else {
            lo = mid;
        }
    }

    cout << hi << endl;

    return 0;
}