#include <stdio.h>
#include <stdlib.h>

int M = 1000000007;

int mod(int x) {
    return ((x % M) + M) % M;
}

int modmul(int x, int y) {
    return mod(mod(x) * mod(y));
}

int modsub(int x, int y) {
    return mod(mod(x) - mod(y));
}

/// База dp[i] = dp[i-1] * 2
/// Формула dp[i] = dp[i-1] * 2 - dp[i - k - 1]
/// 1. Подумайте в сторону префикс сумм (dp[1] = dp[0] + a[1], dp[2] = dp[1] + a[2])
/// 2. Выведите формулу при линии без препятствий
/// 3. Подумайте что происходит с суммой, когда встречается препятствие
/// 4. Выведите зависимость появление препятствий и суммой.
/// 8 3
/// 01100010 = 1 | 1 0 0 1 1 1 0 1 = 1 | 1 0 0 1 1 2 0 3
///                0 1 2 3 4 5 6 7       0 1 2 3 4 5 6 7 | k = 3
int main(void) {
    int n, k;
    scanf("%d", &n);
    scanf("%d", &k);

    int arr[n + 1];
    arr[0] = 1;
    int dp[n + 1];

    {
        char *s = malloc(n * sizeof(char));
        scanf("%s", s);
        int i = 1;
        while (*s != '\0') {
            arr[i] = !(*(s++) - '0');
            dp[i] = 0;
            ++i;
        }
        free(s - n);
    }

    dp[0] = 1;
    dp[1] = 1;

    int gap = 0;
    for (int i = 2; i <= n; ++i) {
        if (arr[i]) {
            gap = 0;
        } else if (++gap == k) {
            break;
        }

        if (i - k - 1 < 0) {
            dp[i] = arr[i - 1] ? modmul(dp[i - 1], 2) : dp[i - 1];
        } else {
            if (arr[i - 1]) {
                dp[i] = modsub(modmul(dp[i - 1], 2), arr[i - k - 1] * dp[i - k - 1]);
            } else {
                dp[i] = modsub(dp[i - 1], dp[i - k - 1]);
            }
        }

        dp[i - 1] = dp[i - 1] * arr[i - 1];
    }
    dp[n] *= arr[n];
    printf("%d", dp[n]);
    return 0;
}