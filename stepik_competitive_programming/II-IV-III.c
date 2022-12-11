#include <stdio.h>

#define ll long long

ll m;

ll mod(ll a) {
    return ((a % m) + m) % m;
}

ll modadd(ll a, ll b) {
    return mod(mod(a) + mod(b));
}

ll modmul(ll a, ll b) {
    return mod(mod(a) * mod(b));
}

ll gcd(ll a, ll b, ll *x, ll *y) {
    if (a == 0) {
        *x = 0;
        *y = 1;
        return b;
    }
    ll x1, y1;
    ll d = gcd(b % a, a, &x1, &y1);
    *x = y1 - (b / a) * x1;
    *y = x1;
    return d;
}

int main() {
    ll a, n;
    scanf("%lld", &a);
    scanf("%lld", &n);
    scanf("%lld", &m);

    ll pow = 1;
    ll ans = 0;
    for (int i = 1; i <= n; ++i) {
        ll x, y;
        ll g = gcd(a, m, &x, &y);
        if (g != 1) {
            printf("-1");
            return 0;
        }
        pow = modmul(pow, mod(x));
        ans = modadd(ans, modmul(i, pow));
    }

    printf("%lld", ans);
    return 0;
}