#include <stdio.h>

#define ll unsigned long long

ll N = 1000000000000000000;

ll gcd(ll a, ll b) {
    if (b == 0) return a;
    return gcd(b, ((a % b) + b) % b);
}

ll lcm(ll a, ll b) {
    return a / gcd(a, b) * b;
}

int main() {
    ll a, b;
    scanf("%llu", &a);
    scanf("%llu", &b);
    ll c = lcm(a, b);
    if (c <= N) printf("%llu", c);
    else printf("-1");
    return 0;
}