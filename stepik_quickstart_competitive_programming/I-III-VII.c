#include <stdio.h>

#define NF (-1000000001)

static int n, m;

int solve(const int arr[], int k) {
    int l = 0, r = n - 1;

    while (l <= r) {
        int mid = (l + r) / 2;
        int mi = arr[mid];

        if (mi <= k) {
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    return l == n ? NF : l + 1;
}


int main() {
    scanf("%d", &n);
    scanf("%d", &m);

    int arr[n];

    for (int i = 0; i < n; ++i) {
        scanf("%d", &arr[i]);
    }

    for (int i = 0; i < m; ++i) {
        int k;
        scanf("%d", &k);
        int s = solve(arr, k);
        if (s == NF) printf("NO SOLUTION\n");
        else printf("%d\n", s);
    }

    return 0;
}
