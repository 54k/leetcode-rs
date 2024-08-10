#include <iostream>
#include <cstdlib>
using namespace std;
const int INF = 2000000000;

struct node
{
    int data;
    int y;
    int size;
    node *left;
    node *right;
    node(int val)
    {
        data = val;
        y = rand();
        size = 1;
        left = nullptr;
        right = nullptr;
    }
    ~node()
    {
        if (left != nullptr)
            delete[] left;
        if (right != nullptr)
            delete[] right;
    }
};

void update_size(node *root)
{
    if (root == nullptr)
        return;
    root->size = 1;
    if (root->left != nullptr)
        root->size += root->left->size;
    if (root->right != nullptr)
        root->size += root->right->size;
}

int get_size(node *root)
{
    return (root == nullptr) ? 0 : root->size;
}

pair<node *, node *> split(node *root, int k)
// Разбиваем дерево так, что в левом поддереве будет k вершин
{
    if (root == nullptr)
        return {nullptr, nullptr};
    if (get_size(root) <= k)
        return {root, nullptr};
    if (k == 0)
        return {nullptr, root};
    if (get_size(root->left) >= k)
    {
        auto res = split(root->left, k);
        root->left = res.second;
        update_size(root);
        return {res.first, root};
    }
    else
    {
        auto res = split(root->right, k - get_size(root->left) - 1);
        root->right = res.first;
        update_size(root);
        return {root, res.second};
    }
}

node *merge(node *root1, node *root2)
{
    if (root1 == nullptr)
        return root2;
    else if (root2 == nullptr)
        return root1;
    if (root1->y < root2->y)
    {
        root1->right = merge(root1->right, root2);
        update_size(root1);
        return root1;
    }
    else
    {
        root2->left = merge(root1, root2->left);
        update_size(root2);
        return root2;
    }
}

node *build(int n)
{
    node *root = nullptr;
    for (int val = 1; val <= n; ++val)
    {
        node *newnode = new node(val);
        root = merge(root, newnode);
    }
    return root;
}

void print(node *root)
{
    if (root == nullptr)
        return;
    print(root->left);
    cout << root->data << " ";
    print(root->right);
}

int main()
{
    int n, m;
    cin >> n >> m;
    node *root = build(n);
    for (int i = 0; i < m; ++i)
    {
        int l, r;
        cin >> l >> r;
        auto split1 = split(root, r);
        auto split2 = split(split1.first, l - 1);
        root = merge(merge(split2.second, split2.first), split1.second);
    }
    print(root);
    cout << endl;
}