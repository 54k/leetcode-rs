#include <stdio.h>

#define ll long long

ll norm(ll d, ll m) {
    return (d % m + m) % m;
}

ll add(ll x, ll y, ll m) {
    return norm(norm(x, m) + norm(y, m), m);
}

ll mul(ll x, ll y, ll m) {
    return norm(norm(x, m) * norm(y, m), m);
}

ll poly(int x, int n, int c[], int m) {
    ll next = norm(x, m); // x^1
    ll ans = add(c[0], mul(next, c[1], m), m);
    for (int i = 2; i <= n; ++i) {
        next = mul(x, next, m);
        ans = add(ans, mul(next, c[i], m), m);
    }
    return ans;
}

int solve(int n, int m, int c[]) {
    for (int i = 1; i <= m; ++i) {
        ll normalized = norm(poly(i, n, c, m), m);
        if (normalized == 0) {
            return i;
        }
    }
    return -1;
}

int main(void) {
    int m, n;
    scanf("%d", &n);
    scanf("%d", &m);

    int c[n + 1];
    for (int i = 0; i <= n; ++i) {
        scanf("%d", &c[n - i]);
    }

    printf("%d", solve(n, m, c));
    return 0;
}