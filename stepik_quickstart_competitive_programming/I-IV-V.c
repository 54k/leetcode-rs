#include <stdio.h>
#include <stdbool.h>

#define ll long long
#define max(x, y) (x < y ? y : x)
#define min(x, y) (x < y ? x : y)
#define FOR(i, x, y) for (int i = x; i < y; ++i)

bool answer(const ll arr[], ll n, ll dist, ll k) {
    ll last = arr[0];
    FOR(i, 0, n) {
        if (arr[i] - last >= dist) {
            last = arr[i];
            --k;
        }
    }
    return k <= 1;
}

ll solve(ll arr[], ll n, ll k) {
    ll l = 0;
    ll r = arr[n - 1] - arr[0] + 1;

    while (r - l > 1) {
        ll mid = l + (r - l) / 2;

        if (answer(arr, n, mid, k)) {
            l = mid;
        } else {
            r = mid;
        }
    }

    return l;
}

int main() {
    ll n, k;

    scanf("%lld", &n);
    scanf("%lld", &k);

    ll arr[n];

    FOR(i, 0, n) {
        scanf("%lld", &arr[i]);
    }

    ll r = solve(arr, n, k);
    printf("%lld\n", r);

    return 0;
}