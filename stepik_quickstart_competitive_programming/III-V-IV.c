#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define max(x, y) (x < y ? x : y)
#define INF 10000000000

struct W {
    int w;
    int p;
} a[512];

int cmpval(const void *x, const void *y) {
    struct W *px = (struct W *) x;
    struct W *py = (struct W *) y;

    long double p1 = px->w > 0 ? (long double) px->p / px->w : INF;
    long double p2 = py->w > 0 ? (long double) py->p / py->w : INF;
    return p1 < p2;
}

int main() {
    int S, N;
    scanf("%d %d", &S, &N);
    for (int i = 0; i < N; ++i) {
        scanf("%d", &a[i].w);
    }
    for (int i = 0; i < N; ++i) {
        scanf("%d", &a[i].p);
    }

    qsort(a, N, sizeof(struct W), cmpval);

    long double sum = 0;

    for (int i = 0; i < N; ++i) {
        if (a[i].w <= S) {
            sum += a[i].p;
            S -= a[i].w;
        } else {
            long double p = ((long double) S / (long double) a[i].w) * (long double) a[i].p;
            sum += p;
            break;
        }
    }

    printf("%ld", (long) ceill(sum));

    return 0;
}