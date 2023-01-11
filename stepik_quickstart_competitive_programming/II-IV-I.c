#include <stdio.h>

#define ll long long

ll modpow(ll x, ll n, ll m) {
    if (n == 0) return 1 % m;
    ll u = modpow(x, n / 2, m);
    u = (u * u) % m;
    if (n % 2 == 1) u = (u * x) % m;
    return u;
}

int main() {
    ll n, m;
    scanf("%llu", &n);
    scanf("%llu", &m);
    printf("%llu", modpow(n, n, m));
    return 0;
}
