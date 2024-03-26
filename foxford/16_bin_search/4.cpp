#include <bits/stdc++.h>
#include <iomanip>

using namespace std;

int a, b, c, d;

double f(double x) {
    return a * (x * x * x) + b * (x * x) + c * x + d;
}

int main() {
    cin >> a >> b >> c >> d;
    if (a < 0) {
        a *= -1;
        b *= -1;
        c *= -1;
        d *= -1;
    }
    const double EPS = 1e-4;
    double lo = -2000.0, hi = 2000.0;

    while (hi - lo > EPS) {
        double mid = (lo + hi) / 2.0;
        if (f(mid) < 0) {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    cout  << setprecision(10) << lo << endl;
    return 0;
}