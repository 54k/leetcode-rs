#include <stdio.h>

int mod(int x, int mod) {
    return ((x % mod) + mod) % mod;
}

int gcd(int a, int b) {
    if (b == 0) return a;
    return gcd(b, mod(a, b));
}

int solve(int a, int b, int c) {
    int g = gcd(a, b);
    if (mod(c, g) != 0) {
        return 0;
    }
    return 1;
}

int main(void) {
    int a, b, c;
    scanf("%d", &a);
    scanf("%d", &b);
    scanf("%d", &c);
    printf("%d", solve(a, b, c));
    return 0;
}