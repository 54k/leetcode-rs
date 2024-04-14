#include "bits/stdc++.h"
using namespace std;

int main() {
    long long n, m;
    cin >> n >> m;
    vector<long long> v(n);
    for (auto &e : v) {
        cin >> e;
    }

    sort(v.begin(), v.end());
    long long ans = 0;
    for (int i = 0; i < n; i++) {
        long long lo = i, hi = n;
        while (lo + 1 != hi) {
            long long mid = (lo + hi) / 2;
            if (v[mid] <= v[i]+m) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        ans += lo - i;
    }
    cout << ans << endl;
    return 0;
}