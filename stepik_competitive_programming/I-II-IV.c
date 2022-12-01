#include <stdio.h>

int main() {
    int n, q, l, r;

    scanf("%d", &n);
    scanf("%d", &q);

    int arr[n];
    for (int i = 0; i < n; ++i) {
        scanf("%d", &arr[i]);
    }

    long prefix[n + 1];
    prefix[0] = 0;

    for (int i = 1; i <= n; ++i) {
        prefix[i] = ((long) arr[i - 1]) + prefix[i - 1];
    }

    for (int i = 0; i < q; ++i) {
        scanf("%d", &l);
        scanf("%d", &r);

        printf("%ld\n", prefix[r] - prefix[l - 1]);
    }

    return 0;
}
