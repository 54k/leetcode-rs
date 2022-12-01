#include <stdio.h>

int n, m;

int solve(int k) {
    int l = -1;
    int r = n;
    int ans = 0;

    while (l + 1 < r) {
        int mid = (r + l) / 2;

        if (mid < k) {
            l = mid;
        } else {
            r = mid;
        }
        ++ans;
    }

    return ans;
}

int main() {
    scanf("%d", &n);
    scanf("%d", &m);

    for (int i = 0; i < m; ++i) {
        int k;
        scanf("%d", &k);
        printf("%d\n", solve(k));
    }

    return 0;
}