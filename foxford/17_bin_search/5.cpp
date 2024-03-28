#include <bits/stdc++.h>
using namespace std;

int main() {
    long long n, a, b, w, h;
    cin >> n >> a >> b >> w >> h;
    long long lo = -1, hi = max(w, h);
    while (lo + 1 != hi) {
        long long d = (lo + hi) / 2;
        long long f = max((w / (b + 2 * d)) * (h / (a + 2 * d)), (w / (a + 2 * d)) * (h / (b + 2 * d)));
        if (f >= n) {
            lo = d;
        } else {
            hi = d;
        }
    }
    cout << lo << endl;
    return 0;
}