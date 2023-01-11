#include <stdio.h>

int main() {
    int S, N;
    scanf("%d %d", &S, &N);
    long ans = 0, it;
    for (int i = 0; i < N; ++i) {
        scanf("%ld", &it);
        if (it < S) {
            ans += it;
            S -= it;
        } else {
            ans += S;
            S = 0;
        }
    }
    printf("%ld", ans);
    return 0;
}