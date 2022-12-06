#include <stdio.h>

int mod = 1000007;

int main(void) {
    int i, j, ans;
    scanf("%d", &i);
    scanf("%d", &j);
    ans = (i % mod * i % mod) % mod - (j % mod * j % mod) % mod;
    printf("%d", ans < 0 ? ans + mod : ans);
}