#include <stdio.h>
#include <stdlib.h>

#define N 500000

static long dp[N];
static int p[N];

///
/// Stack
///
typedef struct t_node {
    long val;
    int idx;
    struct t_node *next;
} t_node;

typedef struct t_stack {
    t_node *head;
} t_stack;

void s_push(t_stack *self, long val, int idx) {
    t_node *node = (t_node *) malloc(sizeof(t_node));
    node->val = val;
    node->idx = idx;
    if (self->head != NULL) {
        node->next = self->head;
    }
    self->head = node;
}

t_node *s_pop(t_stack *self) {
    t_node *h = self->head;
    if (h != NULL) {
        self->head = self->head->next;
    }
    return h;
}

t_node *s_top(t_stack *self) {
    return self->head;
}

int s_is_empty(t_stack *self) {
    return self->head == NULL;
}

///
/// Deque
///
struct t_deque {
    t_stack *left;
    t_stack *right;
    int size;
};

void drain(t_stack *to, t_stack *from) {
    if (s_is_empty(to)) {
        while (!s_is_empty(from)) {
            t_node *pop = s_pop(from);
            s_push(to, pop->val, pop->idx);
        }
    }
}

struct t_deque *d_new() {
    struct t_deque *d = malloc(sizeof(struct t_deque));
    d->left = malloc(sizeof(t_stack));;
    d->right = malloc(sizeof(t_stack));;
    d->size = 0;
    return d;
}

t_node *d_pop_front(struct t_deque *self) {
    drain(self->left, self->right);
    self->size--;
    return s_pop(self->left);
}

t_node *d_top_front(struct t_deque *self) {
    drain(self->left, self->right);
    return s_top(self->left);
}

void d_push_back(struct t_deque *self, long val, int idx) {
    self->size++;
    s_push(self->right, val, idx);
}

t_node *d_pop_back(struct t_deque *self) {
    drain(self->right, self->left);
    self->size--;
    return s_pop(self->right);
}

t_node *d_top_back(struct t_deque *self) {
    drain(self->right, self->left);
    return s_top(self->right);
}

int d_is_empty(struct t_deque *self) {
    return self->size == 0;
}

int main() {
    int n, K, val;
    scanf("%d %d", &n, &K);

    struct t_deque *d = d_new();
    d_push_back(d, 0, -1);

    for (int i = 0; i < n; ++i) {
        scanf("%d", &val);

        t_node *min = d_top_front(d);
        dp[i] = min->val + val;
        p[i] = min->idx;

        while (!d_is_empty(d) && d_top_back(d)->val >= dp[i]) {
            d_pop_back(d);
        }
        d_push_back(d, dp[i], i);

        while (!d_is_empty(d) && d_top_front(d)->idx < i - K + 1) {
            d_pop_front(d);
        }
    }

    long ans = dp[n - 1];
    int ans_length = 0;
    for (int x = n - 1; x != -1; ++ans_length) {
        dp[ans_length] = x + 1;
        x = p[x];
    }
    printf("%ld %d\n", ans, ans_length);
    while (--ans_length >= 0) {
        printf("%ld ", dp[ans_length]);
    }
    return 0;
}