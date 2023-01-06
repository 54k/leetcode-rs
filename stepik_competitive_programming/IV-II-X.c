#include <stdio.h>
#include <stdlib.h>

#define N 200001

typedef struct n_t {
    int u;
    int v;
} n_t;

n_t arr[N];
int ord[N];
int n, m;

int sort_n_t(const void *a, const void *b) {
    n_t *x = (n_t *) a;
    n_t *y = (n_t *) b;
    return ord[x->u] < ord[y->u];
}

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

    qsort(arr, m, sizeof(n_t), sort_n_t);

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