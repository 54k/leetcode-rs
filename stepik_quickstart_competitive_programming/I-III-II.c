#include <stdio.h>

static int n, m;

int solve(const int k[]) {
    int l = -1, r = n;
    int step = 0;

    while (l + 1 < r) {
        int mid = (l + r) / 2;

        if (k[step] > 0) {
            r = mid;
        } else {
            l = mid;
        }
        ++step;
    }

    return r;
}

int main() {
    scanf("%d", &n);
    scanf("%d", &m);

    int k[m];

    for (int i = 0; i < m; ++i) {
        scanf("%d", &k[i]);
    }

    printf("%d\n", solve(k));
    return 0;
}
