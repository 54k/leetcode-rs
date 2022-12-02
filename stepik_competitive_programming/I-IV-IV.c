#include <stdio.h>

#define max(x, y) (x < y ? y : x)
#define min(x, y) (x < y ? x : y)
#define ll long long

ll answer(ll sec, ll x, ll y) {
    ll s = sec - min(x, y);
    return s / x + s / y + 1;
}

ll solve(ll n, ll x, ll y) {
    ll l = min(x, y);
    ll r = n * max(x, y);

    while (l < r) {
        ll mid = l + (r - l) / 2;
        ll ans = answer(mid, x, y);
        if (ans >= n) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    return r;
}

int main() {
    ll n, x, y;

    scanf("%lld", &n);
    scanf("%lld", &x);
    scanf("%lld", &y);

    ll r = solve(n, x, y);
    printf("%lld\n", r);

    return 0;
}