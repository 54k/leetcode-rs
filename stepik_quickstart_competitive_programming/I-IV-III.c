#include <stdio.h>

#define max(w, h) (w < h ? h : w)
#define ll long long

ll answer(ll w, ll h, ll len) {
    return (len / w) * (len / h);
}

ll solve(ll w, ll h, ll n) {
    ll l = 0;
    ll r = max(w, h) * n;

    while (l + 1 < r) {
        ll mid = l + (r - l) / 2;
        if (answer(w, h, mid) >= n) {
            r = mid;
        } else {
            l = mid;
        }
    }

    return r;
}

int main() {
    ll w, h, n;

    scanf("%lld", &w);
    scanf("%lld", &h);
    scanf("%lld", &n);

    ll r = solve(w, h, n);
    printf("%lld", r);

    return 0;
}