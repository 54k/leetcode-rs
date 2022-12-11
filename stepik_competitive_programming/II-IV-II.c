#include <stdio.h>

#define ll long long

ll mod = 1000000007;

ll norm(ll a) {
    return ((a % mod) + mod) % mod;
}

ll modpow(ll x, ll n, ll m) {
    if (n == 0) return 1 % m;
    ll u = modpow(x, n / 2, m);
    u = (u * u) % m;
    if (n % 2 == 1) u = (u * x) % m;
    return u;
}

ll moddiv(ll a, ll b) {
    return norm((a * modpow(b, mod - 2, mod)));
}

ll modmul(ll a, ll b) {
    return norm(norm(a) * norm(b));
}

int main() {
    ll a, b, c, d;
    scanf("%lld", &a);
    scanf("%lld", &b);
    scanf("%lld", &c);
    scanf("%lld", &d);

    // ((a *d + b * c) / b * d) % mod
    ll r = moddiv(modmul(a, d) + modmul(b, c), modmul(b, d));
    printf("%lld", r);

    return 0;
}