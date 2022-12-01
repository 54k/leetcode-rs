#include <stdio.h>

int main() {
    int n;

    scanf("%d", &n);

    long arr[n];
    for (int i = 0; i < n; ++i) {
        scanf("%ld", &arr[i]);
    }

    long max = -1000000000, cur = 0;

    for (int i = 0; i < n; ++i) {
        cur = cur + arr[i] > arr[i] ? cur + arr[i] : arr[i];
        if (cur > max) max = cur;
    }
    printf("%ld", max);
    return 0;
}