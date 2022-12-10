#include <stdio.h>

int mod(int a, int m) {
    return ((a % m) + m) % m;
}

int gcd(int a, int b) {
    if (b == 0) return a;
    return gcd(b, mod(a, b));
}

int main() {
    int N;
    scanf("%d", &N);
    int arr[N];
    for (int i = 0; i < N; ++i) {
        scanf("%d", &arr[i]);
    }
    int prev = arr[0];
    for (int i = 1; i < N; ++i) {
        prev = gcd(prev, arr[i]);
    }
    printf("%d", prev);
    return 0;
}
