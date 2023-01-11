#include <stdio.h>

int cnt = 0;

int gcd(int a, int b) {
    if (b == 0) return a;
    cnt++;
    return gcd(b, a % b);
}

int solve(int a, int b) {
    gcd(a, b);
    return cnt;
}

int main(void) {
    int a, b;
    scanf("%d", &a);
    scanf("%d", &b);
    printf("%d", solve(a, b));
    return 0;
}