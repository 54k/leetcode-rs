#include <stdio.h>

int S, N;
int dp[301][10001];
int w[301];

void print_ans(int k, int s, int d) {
    if (k * s == 0) {
        printf("%d\n", d);
        return;
    }
    if (dp[k - 1][s] == dp[k][s]) {
        print_ans(k - 1, s, d);
    } else {
        print_ans(k - 1, s - w[k], d + 1);
        printf("%d ", k);
    }
}

int main() {
    scanf("%d %d", &S, &N);

    dp[0][0] = 1;

    int wi;
    for (int i = 1; i <= N; ++i) {
        scanf("%d", &w[i]);
        wi = w[i];
        for (int j = 0; j <= S; ++j) {
            if (wi <= j) {
                dp[i][j] = dp[i - 1][j] | dp[i - 1][j - wi];
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    for (int i = S; i >= 0; --i) {
        if (dp[N][i]) {
            printf("%d ", i);
            print_ans(N, i, 0);
            return 0;
        }
    }

    return 0;
}