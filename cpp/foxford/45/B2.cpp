#include <iostream>
#include <string>
#include <random>

using namespace std;
typedef long long ll;
#define pnn pair<Node *, Node *>
const ll INF = 1e10;

struct Node
{
    ll key, prior;
    Node *l = nullptr;
    Node *r = nullptr;
    Node(ll key) : key(key), prior((rand() << 15) | rand()) {}
};

Node *merge(Node *a, Node *b)
{
    if (!a)
        return b;
    if (!b)
        return a;
    if (a->prior > b->prior)
    {
        a->r = merge(a->r, b);
        return a;
    }
    else
    {
        b->l = merge(a, b->l);
        return b;
    }
}

pnn split(Node *v, ll x)
{
    if (!v)
        return {0, 0};
    if (v->key < x)
    {
        pnn A = split(v->r, x);
        v->r = A.first;
        return {v, A.second};
    }
    else
    {
        pnn A = split(v->l, x);
        v->l = A.second;
        return {A.first, v};
    }
}

bool find(Node *v, ll x)
{
    if (!v)
        return false;
    if (v->key == x)
        return true;
    if (v->key < x)
        return find(v->r, x);
    else
        return find(v->l, x);
}

Node *prev(Node *&t, long long x)
{
    Node *cur = t, *succ = nullptr;
    while (cur != nullptr)
    {
        if (cur->key < x)
        {
            succ = cur;
            cur = cur->r;
        }
        else
        {
            cur = cur->l;
        }
    }

    return succ;
}

Node *next(Node *&t, long long x)
{
    Node *cur = t, *succ = nullptr;

    while (cur != nullptr)
    {
        if (cur->key > x)
        {
            succ = cur;
            cur = cur->l;
        }
        else
        {
            cur = cur->r;
        }
    }
    return succ;
}

int main()
{
    ll x;
    Node *root = nullptr;
    string str;
    while (cin >> str)
    {
        cin >> x;
        if (str == "insert")
        {
            if (find(root, x))
                continue;
            pnn A = split(root, x);
            root = merge(A.first, merge(new Node(x), A.second));
        }
        else if (str == "delete")
        {
            if (!find(root, x))
                continue;
            pnn A = split(root, x);
            pnn B = split(A.second, x + 1);
            root = merge(A.first, B.second);
        }
        else if (str == "exists")
        {
            if (find(root, x))
                cout << "true\n";
            else
                cout << "false\n";
        }
        else if (str == "next")
        {
            Node *s = next(root, x);
            if (s == nullptr)
                cout << "none\n";
            else
                cout << s->key << "\n";
        }
        else if (str == "prev")
        {
            Node *s = prev(root, x);
            if (s == nullptr)
                cout << "none\n";
            else
                cout << s->key << "\n";
        }
    }
    return 0;
}