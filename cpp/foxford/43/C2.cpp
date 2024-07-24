#include <cstdlib>
#include <iostream>
using namespace std;
const int INF = 2000000000;
struct node
{
    int x;
    int y;
    int size;
    node *left;
    node *right;
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
pair<node *, node *> split(node *root, int val)
{
    if (root == nullptr)
        return {nullptr, nullptr};
    if (root->x > val)
    {
        auto res = split(root->right, val);
        root->right = res.first;
        update_size(root);
        return {root, res.second};
    }
    else
    {
        auto res = split(root->left, val);
        root->left = res.second;
        update_size(root);
        return {res.first, root};
    }
}
node *merge(node *root1, node *root2)
{
    if (root1 == nullptr)
        return root2;
    else if (root2 == nullptr)
        return root1;
    if (root1->y > root2->y)
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
int exists(node *root, int val)
{
    if (root == nullptr)
        return -INF;
    else if (root->x > val)
        return get_size(root->left) + 1 + exists(root->right, val);
    else if (root->x < val)
        return exists(root->left, val);
    else
        return get_size(root->left) + 1;
}
node *insert(node *root, int val)
{
    if (exists(root, val) > 0)
        return root;
    auto subtree = split(root, val);
    node *newnode = new node(val);
    return merge(merge(subtree.first, newnode), subtree.second);
}
node *remove(node *root, int val)
{
    if (!exists(root, val))
        return root;
    auto subtree1 = split(root, val);
    auto subtree2 = split(subtree1.second, val - 1);
    return merge(subtree1.first, subtree2.second);
}
int kth(node *root, int k)
{
    if (root == nullptr || k > root->size)
        return INF;
    int s = get_size(root->left);
    if (k <= s)
        return kth(root->left, k);
    else if (k == s + 1)
        return root->x;
    else
        return kth(root->right, k - s - 1);
}
int main()
{
    int n;
    cin >> n;
    node *root = nullptr;
    for (int i = 0; i < n; ++i)
    {
        int action;
        int x;
        cin >> action >> x;
        if (action == 1)
        {
            root = insert(root, x);
            cout << exists(root, x) << endl;
        }
        else
        {
            root = remove(root, kth(root, x));
        }
    }
}