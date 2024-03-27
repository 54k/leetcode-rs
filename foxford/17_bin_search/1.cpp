#include <bits/stdc++.h>
using namespace std;

int main() {
    long long w, h, n;
    cin >> w >> h >> n;
    long long lo = 0;
    long long hi = n * max(w, h);

    while (lo + 1 != hi) {
        long long mid = lo + (hi - lo) / 2;
        long long f = (mid / w) * (mid / h);
        if (f < n) {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    cout << hi << endl;
    return 0;
}