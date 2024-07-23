#include <iostream>
#include <string>
#include <cstdlib>

using namespace std;

struct Node
{
    long long val;
    long long priority;
    Node *l, *r;

    Node(long long _val) : val(_val), priority(rand()), l(nullptr), r(nullptr) {}
};

// < x, >= x
pair<Node *, Node *> split(Node *t, long long x)
{
    if (t == nullptr)
        return {nullptr, nullptr};

    if (t->val < x)
    {
        auto p = split(t->r, x);
        auto l = p.first, r = p.second;
        t->r = l;
        return {t, r};
    }
    else
    {
        auto p = split(t->l, x);
        auto l = p.first, r = p.second;
        t->l = r;
        return {l, t};
    }
}

Node *merge(Node *left, Node *right)
{
    if (left == nullptr)
        return right;
    if (right == nullptr)
        return left;

    if (left->priority > right->priority)
    {
        left->r = merge(left->r, right);
        return left;
    }
    else
    {
        right->l = merge(right->l, left);
        return right;
    }
}

Node *insert(Node *t, long long x)
{
    auto p = split(t, x);
    auto l = p.first, r = p.second;
    Node *node = new Node(x);
    return merge(merge(l, node), r);
}

Node *erase(Node *t, long long x)
{
    auto p = split(t, x);
    auto l = p.first, r = p.second;
    auto p2 = split(r, x + 1);
    auto r1 = p2.first, r2 = p2.second;
    return merge(l, r2);
}

bool find(Node *t, long long x)
{
    if (t == nullptr)
        return false;

    if (t->val < x)
    {
        return find(t->r, x);
    }
    else if (t->val > x)
    {
        return find(t->l, x);
    }
    return t->val == x;
}

int main()
{
    int n;
    cin >> n;

    Node *t = nullptr;
    while (n-- > 0)
    {
        string command;
        long long value;
        cin >> command >> value;

        if (command == "insert")
        {
            if (!find(t, value))
            {
                t = insert(t, value);
            }
        }
        else if (command == "delete") // if you prefer to keep delete, change this to 'erase' everywhere
        {
            t = erase(t, value);
        }
        else if (command == "exists")
        {
            cout << (find(t, value) ? "true" : "false") << "\n";
        }
    }

    cout << endl;
    return 0;
}
