//
// Created by Boris Korotaev on 01.12.2022.
//
#include <stdio.h>

int M[100500];

int main() {
    int n;

    scanf("%d", &n);

    for (int i = 0; i < n; i++) {
        scanf("%d", &M[i]);
    }

    long sum = 0;
    for (int i = 0; i < n; i++) {
        sum += M[i];
    }
    printf("%ld", sum);

    return 0;
}
