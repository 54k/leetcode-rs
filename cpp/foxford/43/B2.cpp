#include <iostream>
#include <cstdlib>
#include <string>
#include <ctime>

using namespace std;

struct Node
{
    int val;
    int priority;
    int size;
    Node *left, *right;

    Node(int _val) : val(_val), priority(rand()), size(1), left(nullptr), right(nullptr) {}
};

int getSize(Node *t)
{
    return t ? t->size : 0;
}

void updateSize(Node *t)
{
    if (t)
    {
        t->size = getSize(t->left) + getSize(t->right) + 1;
    }
}

pair<Node *, Node *> split(Node *t, int key)
{
    if (!t)
        return {nullptr, nullptr};

    if (t->val < key)
    {
        auto p = split(t->right, key);
        t->right = p.first;
        updateSize(t);
        return {t, p.second};
    }
    else
    {
        auto p = split(t->left, key);
        t->left = p.second;
        updateSize(t);
        return {p.first, t};
    }
}

Node *merge(Node *left, Node *right)
{
    if (!left || !right)
        return left ? left : right;

    if (left->priority > right->priority)
    {
        left->right = merge(left->right, right);
        updateSize(left);
        return left;
    }
    else
    {
        right->left = merge(left, right->left);
        updateSize(right);
        return right;
    }
}

Node *insert(Node *t, int key)
{
    if (!t)
        return new Node(key);

    if (key < t->val)
    {
        t->left = insert(t->left, key);
    }
    else if (key > t->val)
    {
        t->right = insert(t->right, key);
    }
    else
    {
        return t;
    }

    updateSize(t);
    return t;
}

Node *erase(Node *t, int key)
{
    if (!t)
        return nullptr;

    if (t->val == key)
    {
        Node *temp = merge(t->left, t->right);
        delete t;
        return temp;
    }

    if (key < t->val)
    {
        t->left = erase(t->left, key);
    }
    else
    {
        t->right = erase(t->right, key);
    }

    updateSize(t);
    return t;
}

int exists(Node *t, int key)
{
    if (!t)
        return 0;

    if (key < t->val)
    {
        return exists(t->left, key);
    }
    else if (key > t->val)
    {
        int leftSize = getSize(t->left) + 1;
        int rightResult = exists(t->right, key);
        return rightResult ? leftSize + rightResult : 0;
    }
    else
    {
        return getSize(t->left) + 1;
    }
}

int main()
{
    srand(time(nullptr));

    int n;
    cin >> n;

    Node *root = nullptr;
    for (int i = 0; i < n; ++i)
    {
        string command;
        int value;
        cin >> command >> value;

        if (command == "insert")
        {
            if (exists(root, value) == 0)
            {
                root = insert(root, value);
            }
        }
        else if (command == "delete")
        {
            root = erase(root, value);
        }
        else if (command == "exists")
        {
            cout << exists(root, value) << endl;
        }
    }

    return 0;
}
