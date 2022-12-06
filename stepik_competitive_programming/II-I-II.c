#include <stdio.h>

int prime(int n) {
    for (int x = 2; x * x <= n; ++x) {
        if (n % x == 0) {
            return 0;
        }
    }
    return 1;
}

int main() {
    int n;
    scanf("%d", &n);
    printf("%d", prime(n));
    return 0;
}