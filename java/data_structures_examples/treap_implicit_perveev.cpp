#include <bits/stdc++.h>
using namespace std;
mt19937 rnd(238);
const int INF = (int)1e9;

// https://www.youtube.com/watch?v=LXkpC_Mis_U&list=PLjCCarnDJNsuH2okFGVjvnGkta3YtwTWo&index=31
struct Node
{
    Node *l;
    Node *r;

    int value;
    int priority;
    int sz;
    int min, add;

    Node(int _value)
    {
        value = min = _value;
        priority = rnd();
        sz = 1;
        add = 0;
        l = r = nullptr;
    }
};

int getSz(Node *t)
{
    if (t == nullptr)
        return 0;
    return t->sz;
}

int getMin(Node *t)
{
    if (t == nullptr)
    {
        return INF;
    }
    return t->min;
}

void update(Node *t)
{
    t->sz = getSz(t->l) + getSz(t->r) + 1;
    t->min = min(min(getMin(t->l), getMin(t->r)), t->value);
}

void push(Node *t)
{
    if (t->add == 0)
        return;

    if (t->l != nullptr)
    {
        t->l->add += t->add;
        t->l->min += t->add;
        t->l->value += t->add;
    }

    if (t->r != nullptr)
    {
        t->r->add += t->add;
        t->r->min += t->add;
        t->r->value += t->add;
    }

    t->add = 0;
}

// слева х элементов, справа n - x элементов
pair<Node *, Node *> split(Node *t, int x)
{
    if (t == nullptr)
        return {nullptr, nullptr};

    push(t);
    // +1 учитываем корень
    int sz = getSz(t->l) + 1;
    if (x >= sz)
    {
        auto [l, r] = split(t->r, x - sz);
        t->r = l;
        update(t);
        return {t, r};
    }
    else
    {
        auto [l, r] = split(t->l, x);
        t->l = r;
        update(t);
        return {l, t};
    }
}

Node *merge(Node *l, Node *r)
{
    if (l == nullptr)
        return r;
    if (r == nullptr)
        return l;

    if (l->priority > r->priority)
    {
        push(l);
        l->r = merge(l->r, r);
        update(l);
        return l;
    }
    else
    {
        push(r);
        r->l = merge(l, r->l);
        update(r);
        return r;
    }
}

Node *insert(Node *t, int x)
{
    Node *newNode = new Node(x);
    return merge(t, newNode);
}

void print(Node *t)
{
    if (t == nullptr)
        return;

    push(t);
    print(t->l);
    cout << t->value << " ";
    print(t->r);
}

// find kth smallest element
Node *findKth(Node *t, int k)
{
    if (t == nullptr)
        return nullptr;

    // 0 1 2 .. getSz(t->l) - 1
    if (k < getSz(t->l))
    {
        return findKth(t->l, k);
    }
    else if (k == getSz(t->l))
    {
        return t;
    }
    else
    {
        // -1 because of root
        return findKth(t->r, k - getSz(t->l) - 1);
    }
}

Node *erase(Node *t, int x)
{
    // < x, >= x
    auto [l, r] = split(t, x);
    // == x, > x
    auto [r1, r2] = split(r, x + 1);
    delete r1; // not a very proper delete, but you don't need it in olympiad programming
    return merge(l, r2);
}

Node *shift(Node *t, int k)
{
    auto [l, r] = split(t, k);
    return merge(r, l);
}

// [l, r)
int getMin(Node *&t, int l, int r)
{
    // [0, l) [l, n)
    auto [left, right] = split(t, l);
    // [l, r) [r, n)
    auto [right1, right2] = split(right, r - l);
    int ans = getMin(right1);
    t = merge(left, merge(right1, right2));
    return ans;
}

void add(Node *t, int l, int r, int d)
{
    // [0, l) [l, n)
    auto [left, right] = split(t, l);
    // [l, r) [r, n)
    auto [right1, right2] = split(right, r - l);

    right1->add += d;
    right1->min += d;
    right1->value += d;

    t = merge(left, merge(right1, right2));
}

int main()
{
    Node *t = nullptr;
    for (int i = 0; i < 10; i++)
    {
        t = insert(t, i);
    }
    print(t);
    cout << endl;

    t = shift(t, 3);
    print(t);
    cout << endl;

    int ans = getMin(t, 2, 5);

    cout << ans << endl;

    add(t, 2, 5, -2);
    ans = getMin(t, 2, 5);
    cout << ans << endl;

    return 0;
}