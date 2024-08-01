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
// Разбиваем дерево так, что в левом дереве будет k вершин
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
void print(node *root)
{
    if (root == nullptr)
        return;
    print(root->left);
    cout << root->data << endl;
    print(root->right);
}
int main()
{
    int n;
    cin >> n;
    node *root = nullptr;
    for (int i = 0; i < n; ++i)
    {
        int pos, id;
        cin >> pos >> id;
        auto res = split(root, pos - 1);
        root = new node(id);
        root = merge(merge(res.first, root), res.second);
    }
    print(root);
}