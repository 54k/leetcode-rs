#include <stdio.h>

#define N 1001
#define S 100500

long G[N][N];

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

int n, m;

int main() {
    scanf("%d %d", &n, &m);

    for (int i = 1; i <= m; ++i) {
        int u, v;
        scanf("%d %d", &u, &v);
        G[u][v] = 1;
        G[v][u] = 1;
    }

    for (int s = 1; s <= n; ++s) {
        for (int i = 1; i <= n; ++i) {
            G[0][i] = -1;
        }
        push(s);
        G[0][s] = 0;
        while (size() > 0) {
            int u = pop();
            G[0][0] += G[0][u];
            for (int v = 1; v <= n; ++v) {
                if (G[u][v] && G[0][v] == -1) {
                    G[0][v] = G[0][u] + 1;
                    push(v);
                }
            }
        }
    }

    printf("%ld", G[0][0] / 2);
    return 0;
}