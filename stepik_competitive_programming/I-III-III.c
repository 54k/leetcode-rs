#include <stdio.h>

static int n, m;

char *solve(const int arr[], int k) {
    int l = 0;
    int r = n - 1;
    while (l <= r) {
        int mid = l + (r - l) / 2;

        if (arr[mid] < k) {
            l = mid + 1;
        } else if (arr[mid] > k) {
            r = mid - 1;
        } else {
            return "YES";
        }
    }
    return "NO";
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
        printf("%s\n", solve(arr, k));
    }
    return 0;
}
