#include <stdio.h>
#include <stdbool.h>

#define ll long long
#define max(x, y) (x < y ? y : x)
#define min(x, y) (x < y ? x : y)
#define FOR(i, x, y) for (int i = x; i < y; ++i)

#define N_MAX 10e7

bool answer(const ll arr[], ll n, ll dist, ll k) {
    ll ans = 0;
    FOR(i, 0, n) {
        ans += arr[i] / dist;
    }
    return ans >= k;
}

ll solve(ll arr[], ll n, ll k) {
    ll l = 1;
    ll r = N_MAX;

    while (r - l > 1) {
        ll mid = l + (r - l) / 2;

        if (answer(arr, n, mid, k)) {
            l = mid;
        } else {
            r = mid;
        }
    }

    return answer(arr, n, l, k) ? l : 0;
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