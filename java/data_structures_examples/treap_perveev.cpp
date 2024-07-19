#include <bits/stdc++.h>
using namespace std;
mt19937 rnd(238);

// https://www.youtube.com/watch?v=LXkpC_Mis_U&list=PLjCCarnDJNsuH2okFGVjvnGkta3YtwTWo&index=31
struct Node
{
    Node *l;
    Node *r;

    int key;
    int priority;
    int sz;

    Node(int _key)
    {
        key = _key;
        priority = rnd();
        sz = 1;
        l = r = nullptr;
    }
};

int getSz(Node *t)
{
    if (t == nullptr)
        return 0;
    return t->sz;
}

void update(Node *t)
{
    t->sz = getSz(t->l) + getSz(t->r) + 1;
}

// < x, >= x
pair<Node *, Node *> split(Node *t, int x)
{
    if (t == nullptr)
        return {nullptr, nullptr};

    if (t->key < x)
    {
        auto [r1, r2] = split(t->r, x);
        t->r = r1;
        update(t);
        return {t, r2};
    }
    else
    {
        auto [l1, l2] = split(t->l, x);
        t->l = l2;
        update(t);
        return {l1, t};
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
        l->r = merge(l->r, r);
        update(l);
        return l;
    }
    else
    {
        r->l = merge(l, r->l);
        update(r);
        return r;
    }
}

Node *insert(Node *t, int x)
{
    auto [l, r] = split(t, x);
    Node *newNode = new Node(x);
    return merge(merge(l, newNode), r);
}

void print(Node *t)
{
    if (t == nullptr)
        return;

    print(t->l);
    cout << t->key << " ";
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

int main()
{
    Node *t = nullptr;
    for (int i = 0; i < 10; i++)
    {
        t = insert(t, i);
    }
    print(t);
    cout << endl;

    cout << findKth(t, 4)->key << endl;
    erase(t, 4);
    print(t);
    return 0;
}