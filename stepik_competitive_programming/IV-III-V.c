#include <stdio.h>

#define Q 100001
#define S 5001

char G[S][S];
char D[S][S];
int V[S][S][5];

struct edge_t {
    int x;
    int y;
    int vx;
    int vy;
    int d;
} A[Q];
int e_f, e_b;

void push(int x, int y, int vx, int vy) {
    A[e_b].x = x;
    A[e_b].y = y;
    A[e_b].vx = vx;
    A[e_b].vy = vy;

    if (vx < 0 && vy == 0) {
        A[e_b].d = 1;
    } else if (vx == 0 && vy > 0) {
        A[e_b].d = 2;
    } else if (vx > 0 && vy == 0) {
        A[e_b].d = 3;
    } else if (vx == 0 && vy < 0) {
        A[e_b].d = 4;
    } else {
        A[e_b].d = 0;
    }

    e_b = (e_b + 1) % Q;
}

struct edge_t pop() {
    struct edge_t v = A[e_f];
    e_f = (e_f + 1) % Q;
    if (e_f == e_b) {
        e_f = e_b = 0;
    }
    return v;
}

int size() {
    return ((e_b - e_f) + Q) % Q;
}

int N, M;

int main() {
    scanf("%d %d", &N, &M);
    for (int i = 1; i <= 2 * N + 1; ++i) {
        int j = 1;
        while (j <= 2 * M + 1) {
            char ch;
            scanf("%c", &ch);
            if (ch != '\n' && ch != '\0') {
                G[i][j] = ch;
                j++;
            }
        }
    }

    for (int i = 1; i <= 2 * N + 1; ++i) {
        for (int j = 1; j <= 2 * M + 1; ++j) {
            if (G[i][j] == 'S') {
                push(i, j, 0, 0);
            }
        }
    }

    while (size() > 0) {
        struct edge_t e = pop();
        int x = e.x;
        int y = e.y;
        int vx = e.vx;
        int vy = e.vy;
        int d = e.d;

        char ch = G[x][y];

        if (x <= 0 || x > 2 * N + 1 || y <= 0 || y > 2 * M + 1 || V[x][y][d] ||
            (ch != ' ' && ch != 'D' && ch != 'S'))
            continue;

        V[x][y][d] = 1;

        if (ch == 'D') {
            D[x][y] = 1;
        }

        char l = G[x - 1][y];
        char r = G[x + 1][y];
        char t = G[x][y + 1];
        char b = G[x][y - 1];

        if (l == '-' || l == '|') {
            push(x, y, 1, 0);
        }
        if (r == '-' || r == '|') {
            push(x, y, -1, 0);
        }
        if (t == '-' || t == '|') {
            push(x, y, 0, -1);
        }
        if (b == '-' || b == '|') {
            push(x, y, 0, 1);
        }
        push(x + vx, y + vy, vx, vy);
    }

    printf("\n");
    for (int i = 1; i <= 2 * N + 1; ++i) {
        for (int j = 1; j <= 2 * M + 1; ++j) {
            if (G[i][j] == 'D' && !D[i][j]) {
                printf(" ");
            } else {
                printf("%c", G[i][j]);
            }
        }
        printf("\n");
    }

    return 0;
}