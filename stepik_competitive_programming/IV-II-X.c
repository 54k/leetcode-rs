#include <stdio.h>

#define N 200001

typedef struct n_t {
    int u;
    int v;
} n_t;

n_t arr[N];
int ord[N];
int n, m;

int main() {
    scanf("%d %d", &n, &m);

    for (int i = 0; i < m; ++i) {
        int u, v;
        scanf("%d %d", &u, &v);
        arr[i].u = u;
        arr[i].v = v;
    }

    for (int i = 1; i <= n; ++i) {
        int q;
        scanf("%d", &q);
        ord[q] = i;
    }

    for (int i = 0; i < m; ++i) {
        n_t e = arr[i];
        if (ord[e.u] >= ord[e.v]) {
            printf("NO\n");
            return 0;
        }
    }

    printf("YES\n");
    return 0;
}