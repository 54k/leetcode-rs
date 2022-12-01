//
// Created by Boris Korotaev on 01.12.2022.
//
#include <stdio.h>

long long M[10000500];

int mod = 1791791791;
int n;
long long cur, a, b;

long nextRand() {
    cur = (cur * a + b) % mod;
    return cur;
}

int main() {
    scanf("%d", &n);
    scanf("%lld", &cur);
    scanf("%lld", &a);
    scanf("%lld", &b);

    for (int i = 0; i < n; ++i) {
        M[i] = nextRand();
    }

    int maxi = -1;
    for (int i = 0; i < n; ++i) {
        if (maxi < 0 || M[maxi] < M[i]) maxi = i;
    }

    int maxi2 = -1;
    for (int i = 0; i < n; ++i) {
        if (maxi != i) {
            if (maxi2 < 0 || M[maxi2] < M[i]) { maxi2 = i; }
        }
    }

    printf("%d %d", maxi + 1, maxi2 == maxi ? maxi2 + 2 : maxi2 + 1);

    return 0;
}
