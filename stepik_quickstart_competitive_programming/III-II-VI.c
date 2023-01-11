#include <stdio.h>

#define ll long long

int main() {
    int n;
    scanf("%d", &n);
    if (n == 0) {
        printf("1");
        return 0;
    }
    ll a = 1;
    ll b = 1;
    ll c;
    for (int i = 1; i <= n; ++i) {
        c = a + b;
        a = b;
        b = c;
    }
    printf("%lld", c);
    return 0;
}
