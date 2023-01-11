#include <stdio.h>

#define ll long long

ll md = 1000000007;
ll fac[1000001];

ll mod(ll a) {
    return ((a % md) + md) % md;
}

ll modsub(ll a, ll b) {
    return mod(mod(a) - mod(b));
}

ll modmul(ll a, ll b) {
    return mod(mod(a) * mod(b));
}

ll modadd(ll a, ll b) {
    return mod(mod(a) + mod(b));
}

ll factorial(ll a) {
    if (fac[a] > 0) {
        return fac[a];
    }
    ll f = 1;
    fac[1] = f;
    for (int i = 2; i <= a; ++i) {
        f = modmul(f, i);
        fac[i] = f;
    }
    return f;
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
    ll n, m, l;
    scanf("%lld", &n);
    scanf("%lld", &m);
    scanf("%lld", &l);

    ll ans = 0;
    ll fac_n = factorial(n);
    for (int k = 1; k <= l; ++k) {
        ll mk = modmul(m, k);
        if (mk <= n) {
            ll x, y;
            ll nmk = modsub(n, mk);
            ll fac_nmk = factorial(nmk);
            ll fac_mk = factorial(mk);
            gcd(modmul(
                        fac_nmk,
                        fac_mk
                ), md, &x, &y
            );
            ans = modadd(ans, modmul(fac_n, mod(x)));
        } else {
            break;
        }
    }
    printf("%lld", ans);
    return 0;
}