#include <stdio.h>
#include <stdlib.h>

int M = 1000000007;

int mod(int a) {
    return ((a % M) + M % M);
}

int modadd(int a, int b) {
    return mod(mod(a) + mod(b));
}

int go(int arr[], int dp[], int i) {
    if (i < 0) {
        return 0;
    }
    if (i == 0) {
        return 1;
    }
    if (dp[i] != -1) {
        return dp[i];
    }
    if (arr[i] == 1) {
        dp[i] = 0;
    } else {
        dp[i] = modadd(modadd(go(arr, dp, i - 1), go(arr, dp, i - 2)), go(arr, dp, i - 3));
    }
    return dp[i];
}

int main(void) {
    int n;
    scanf("%d", &n);
    char *s = malloc(n * sizeof(int));
    scanf("%s", s);

    int arr[n + 1];
    arr[0] = 0;
    int i = 0;
    while (*s != '\0') {
        arr[++i] = *(s++) - '0';
    }
    free(s - n);

    int dp[n + 1];
    for (int j = 0; j <= n; ++j) {
        dp[j] = -1;
    }
    dp[0] = 1;
    for (int j = 1; j <= n; ++j) {
        go(arr, dp, j);
    }
    printf("%d", dp[n]);
    return 0;
}