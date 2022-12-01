//
// Created by Boris Korotaev on 01.12.2022.
//
#include <stdio.h>

int M[10000500];

int main() {
    int n;

    scanf("%d", &n);

    for (int i = 0; i < n; i++) {
        scanf("%d", &M[i]);
    }

    int maxi = 0;
    for (int i = 1; i < n; i++) {
        if (M[maxi] < M[i]) maxi = i;
    }
    printf("%d", maxi + 1);

    return 0;
}
