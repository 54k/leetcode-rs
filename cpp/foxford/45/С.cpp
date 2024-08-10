#include <cstdio>
#include <cstdlib>
#include <utility>
#include <iostream>
using namespace std;

typedef long long ll;
#define pnn pair<Node *, Node *>
const ll INF = 1e10;

struct node
{
    int x, y, size;
    node *left, *right;

    node(int val)
    {
        x = val;
        y = rand();
        size = 1;
        left = nullptr;
        right = nullptr;
    }

    ~node()
    {
        if (left)
            delete[] left;
        if (right)
            delete[] right;
    }
};

int get_size(node *root)
{
    if (root == nullptr)
        return 0;
    return root->size;
}

void update(node *root)
{
    if (root == nullptr)
    {
        return;
    }
    root->size = 1 + get_size(root->left) + get_size(root->right);
}

pair<node *, node *> split(node *root, int k)
{
    if (root == nullptr)
        return {nullptr, nullptr};
    if (get_size(root) <= k)
    {
        return {root, nullptr};
    }
    if (k == 0)
    {
        return {nullptr, root};
    }

    if (get_size(root->left) >= k)
    {
        auto res = split(root->left, k);
        root->left = res.second;
        update(root);
        return {res.first, root};
    }
    else
    {
        auto res = split(root->right, k - get_size(root->left) - 1);
        root->right = res.first;
        update(root);
        return {root, res.second};
    }
}

node *merge(node *left, node *right)
{
    if (left == nullptr)
        return right;
    if (right == nullptr)
        return left;

    if (left->y < right->y)
    {
        left->right = merge(left->right, right);
        update(left);
        return left;
    }
    else
    {
        right->left = merge(left, right->left);
        update(right);
        return right;
    }
}

node *insert(node *root, int x)
{
    node *new_node = new node(x);
    return merge(root, new_node);
}

node *shift_to_beginning(node *root, int l, int r)
{
    auto s1 = split(root, l - 1);
    auto s2 = split(s1.second, r - l + 1);
    return merge(s2.first, merge(s1.first, s2.second));
}

void print_tree(node *root)
{
    if (root == nullptr)
        return;
    print_tree(root->left);
    cout << root->x << " ";
    print_tree(root->right);
}

int main()
{
    int n, m;
    cin >> n >> m;
    node *root = nullptr;
    for (int i = 1; i <= n; i++)
    {
        root = insert(root, i);
    }

    // cout << endl;
    // print_tree(root);

    while (m-- > 0)
    {
        int l, r;
        cin >> l >> r;

        root = shift_to_beginning(root, l, r);

        // cout << endl;
        // print_tree(root);
    }

    // cout << endl;
    print_tree(root);
    cout << endl;
    return 0;
}