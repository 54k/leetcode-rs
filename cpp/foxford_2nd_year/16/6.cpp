#include <bits/stdc++.h>
using namespace std;

bool check(long long mid, long long n, long long x, long long y) {
    long long ans = 1;
    ans += (mid - min(x, y)) / x;
    ans += (mid - min(x, y)) / y;
    return ans >= n;
}

int main() {
    long long n, x, y;
    cin >> n >> x >> y;
    long long lo = min(x, y) - 1;
    long long hi = n * max(x, y);
    while (lo + 1 < hi) {
        long long mid = lo + (hi - lo) / 2;
        if (check(mid, n, x, y)) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    cout << hi << endl;
	return 0;
}