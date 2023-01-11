#include <stdio.h>

#define ull unsigned long long

ull solve(ull a, ull b, ull c) {
    return a * b * c / ((a * b) + (b * c) + (a * c));
}

int main() {
    int a, b, c;

    scanf("%d", &a);
    scanf("%d", &b);
    scanf("%d", &c);

    printf("%lld", solve(a, b, c));

    return 0;
}