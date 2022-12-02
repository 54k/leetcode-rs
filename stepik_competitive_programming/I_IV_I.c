#include <stdio.h>
#include <math.h>

const long double EPS = 10e-9;

long double f(long double x, int p, const long double cf[]) {
    long double ans = 0.0;
    for (int i = p; i >= 0; --i) {
        long double c = cf[p - i];
        long double pp = powl(x, i);
        ans += pp * c;
    }
    return ans;
}

long double solve(int p, long double cf[]) {
    long double l = -100.0, r = 100.0;

    while (r - l > EPS) {
        long double mid = (l + r) / 2.0;
        long double s = f(mid, p, cf);
        if (s < 0.0) {
            l = mid;
        } else {
            r = mid;
        }
    }

    return (l + r) / 2.0;
}

int main() {
    int n;
    scanf("%d", &n);

    long double cf[n + 1];

    for (int i = 0; i < n + 1; ++i) {
        scanf("%Lf", &cf[i]);
    }

    long double ans = solve(n, cf);
    printf("%.16Lf\n", ans);
    return 0;
}
