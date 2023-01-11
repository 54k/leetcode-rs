#include <stdio.h>
#include <stdlib.h>

#define N 200001

typedef struct n_t {
    int v;
} n_t;

typedef struct ll_t {
    n_t *alloc;
    int size;
    int cap;
} ll_t;

ll_t *new() {
    ll_t *pLl = (ll_t *) malloc(sizeof(ll_t));
    pLl->alloc = malloc(sizeof(n_t) * 2);
    pLl->cap = 2;
    return pLl;
}

void add(ll_t *pLl, int v) {
    if (pLl->size + 1 > pLl->cap) {
        pLl->alloc = realloc(pLl->alloc, sizeof(n_t) * pLl->cap * 2);
        pLl->cap *= 2;
    }
    pLl->alloc[pLl->size].v = v;
    pLl->size++;
}

int has(ll_t *pLl, int v) {
    for (int i = 0; i < pLl->size; ++i) {
        if (pLl->alloc[i].v == v) return 1;
    }
    return 0;
}

ll_t *adj[N];
int seen[N];
int path[N];
int n, m, pi;

void dfs(int u) {
    seen[u] = 1;
    for (int i = 0; i < adj[u]->size; ++i) {
        int v = adj[u]->alloc[i].v;
        if (!seen[v]) dfs(v);
    }
    path[pi--] = u;
}

int main() {
    scanf("%d %d", &n, &m);
    pi = n;
    for (int i = 1; i <= n; ++i) {
        adj[i] = new();
    }

    for (int i = 1; i <= m; ++i) {
        int u, v;
        scanf("%d %d", &u, &v);
        add(adj[u], v);
    }

    for (int i = 1; i <= n; ++i) {
        if (!seen[i]) dfs(i);
    }

    while (++pi < n) {
        if (!has(adj[path[pi]], path[pi + 1])) {
            printf("NO");
            return 0;
        }
    }

    printf("YES\n");
    return 0;
}