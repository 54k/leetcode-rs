#include <stdio.h>

#define Q 100001
#define S 101

int V[S][S];

struct edge_t {
    int x;
    int y;
    int d;
} A[Q];
int f, b;

void push(int v, int u, int d) {
    A[b].x = v;
    A[b].y = u;
    A[b].d = d;
    b = (b + 1) % Q;
}

struct edge_t pop() {
    struct edge_t v = A[f];
    f = (f + 1) % Q;
    if (f == b) {
        f = b = 0;
    }
    return v;
}

int size() {
    return ((b - f) + Q) % Q;
}

int M, N, p, q, x1, y1, x2, y2;

int main() {
    scanf("%d %d %d %d %d %d %d %d", &M, &N, &p, &q, &x1, &y1, &x2, &y2);

    push(x1, y1, 0);
    while (size() > 0) {
        struct edge_t e = pop();
        int x = e.x;
        int y = e.y;
        int d = e.d;

        if (x <= 0 || x > M || y <= 0 || y > N || V[x][y]) continue;
        V[x][y] = 1;

        if (x == x2 && y == y2) {
            printf("%d", d);
            return 0;
        }

        // p=3,q=4
        // 5, 6
        push(x - q, y - p, d + 1); // 1, 3
        push(x - q, y + p, d + 1); // 1, 9
        push(x - p, y - q, d + 1); // 2, 2
        push(x - p, y + q, d + 1); // 2, 10

        push(x + q, y - p, d + 1); // 9, 3
        push(x + q, y + p, d + 1); // 9, 9
        push(x + p, y - q, d + 1); // 8, 2
        push(x + p, y + q, d + 1); // 8, 10
    }

    printf("-1");
    return 0;
}