#include <stdio.h>

#define S 250000

int G[501][501];

int A[S], f, b;

void push(int v) {
    A[b] = v;
    b = (b + 1) % S;
}

int pop() {
    int v = A[f];
    f = (f + 1) % S;
    if (f == b) {
        f = b = 0;
    }
    return v;
}

int size() {
    return ((b - f) + S) % S;
}

int n, m, s;

int main() {
    scanf("%d %d", &n, &m);
    for (int i = 1; i <= m; ++i) {
        int u, v;
        scanf("%d %d", &u, &v);
        G[u][v] = i;
        G[v][u] = i;
    }
    scanf("%d", &s);

    push(s);
    G[0][s] = 1;

    while (size() > 0) {
        int u = pop();
        printf("%d ", u);
        for (int i = 1; i <= n; ++i) {
            if (!G[0][i] && G[u][i]) {
                G[0][i] = 1;
                push(i);
            }
        }
    }

    return 0;
}